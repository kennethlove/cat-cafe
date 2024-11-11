use axum::error_handling::HandleErrorLayer;
use axum::response::IntoResponse;
use axum::{extract::Path, http::StatusCode, routing::get, BoxError, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use std::time::Duration;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Uuid;
use surrealdb::{RecordId, Surreal};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use shared::{Cat, NewCat};

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    DB.connect::<Ws>("localhost:8000").await.unwrap();
    tracing::debug!("connected to surrealdb");
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    tracing::debug!("authenticated with surrealdb");
    DB.use_ns("cat-cafe").use_db("cats").await.unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/cats", get(get_cats).post(create_cat))
        .route(
            "/cats/:uuid",
            get(get_cat).delete(delete_cat).put(update_cat),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_cats() -> Result<impl IntoResponse, StatusCode> {
    match DB.select("cat").await {
        Ok(cats) => Ok((StatusCode::OK, Json::<Vec<Cat>>(cats))),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_cat(Path(uuid): Path<Uuid>) -> Result<impl IntoResponse, StatusCode> {
    let mut response = DB
        .query("SELECT * FROM cat WHERE identifier = $uuid")
        .bind(("uuid", uuid))
        .await
        .unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();
    let name: Option<String> = response.take("name").unwrap();
    let breed: Option<String> = response.take("breed").unwrap();

    match id {
        None => Err(StatusCode::NOT_FOUND),
        _ => {
            let cat = Cat {
                id: id.unwrap(),
                identifier: uuid,
                name: name.unwrap(),
                breed: breed.unwrap(),
            };
            Ok((StatusCode::OK, Json(cat)))
        }
    }
}

async fn create_cat(Json(payload): Json<NewCat>) -> Result<impl IntoResponse, StatusCode> {
    let identifier = Uuid::new_v4();
    let cat: Option<Cat> = DB
        .create("cat")
        .content(NewCat {
            identifier: Some(identifier),
            name: payload.name.clone(),
            breed: payload.breed.clone(),
        })
        .await
        .unwrap();
    Ok((StatusCode::CREATED, Json(cat.unwrap())))
}

async fn update_cat(
    Path(uuid): Path<Uuid>,
    Json(payload): Json<NewCat>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut response = DB
        .query("SELECT * FROM cat WHERE identifier = $uuid")
        .bind(("uuid", uuid))
        .await
        .unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();
    match id {
        None => Err(StatusCode::NOT_FOUND),
        _ => {
            let cat: Option<Cat> = DB
                .update(id.unwrap())
                .content(NewCat {
                    identifier: Some(uuid),
                    name: payload.name.clone(),
                    breed: payload.breed.clone(),
                })
                .await
                .unwrap();

            Ok((StatusCode::OK, Json(Some(cat.unwrap()))))
        }
    }
}

async fn delete_cat(Path(uuid): Path<Uuid>) -> StatusCode {
    let mut response = DB
        .query("SELECT * FROM cat WHERE identifier = $uuid")
        .bind(("uuid", uuid))
        .await
        .unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();
    match id {
        None => StatusCode::NO_CONTENT,
        _ => {
            let id: RecordId = id.unwrap();
            let split_id: Vec<String> = id.to_string().split(':').map(String::from).collect();

            let _: Option<Cat> = DB
                .delete((split_id[0].clone(), split_id[1].clone()))
                .await
                .unwrap();
            StatusCode::NO_CONTENT
        }
    }
}

async fn root() -> &'static str {
    "Hello World!"
}