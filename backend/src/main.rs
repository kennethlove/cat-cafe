use axum::error_handling::HandleErrorLayer;
use axum::response::IntoResponse;
use axum::{extract::Path, http::StatusCode, routing::get, BoxError, Json, Router};
use std::sync::LazyLock;
use std::time::Duration;
use axum::http::HeaderValue;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{RecordId, Surreal};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowMethods};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

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

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:8080".parse().unwrap()))
        .allow_methods(vec![
            "GET".parse().unwrap(),
            "POST".parse().unwrap(),
            "PUT".parse().unwrap(),
            "DELETE".parse().unwrap(),
        ]);

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
                .layer(cors_layer)
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
        .bind(("uuid", uuid.to_string()))
        .await
        .unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();
    let identifier: Option<String> = response.take("identifier").unwrap();
    let name: Option<String> = response.take("name").unwrap();
    let breed: Option<String> = response.take("breed").unwrap();

    match id {
        None => Err(StatusCode::NOT_FOUND),
        Some(_) => {
            let cat = Cat {
                // id: id.unwrap().parse().unwrap(),
                identifier: identifier.unwrap(),
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
            identifier: Some(identifier.to_string()),
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
        .bind(("uuid", uuid.to_string()))
        .await
        .unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();
    let identifier: Option<String> = response.take("identifier").unwrap();
    match id {
        None => Err(StatusCode::NOT_FOUND),
        Some(id) => {
            let cat: Option<Cat> = DB
                .update(id)
                .content(NewCat {
                    identifier: identifier.clone(),
                    name: payload.name.clone(),
                    breed: payload.breed.clone(),
                })
                .await
                .unwrap();

            Ok((StatusCode::OK, Json(Some(cat))))
        }
    }
}

async fn delete_cat(Path(uuid): Path<Uuid>) -> StatusCode {
    let mut response = DB
        .query("SELECT * FROM cat WHERE identifier = $uuid")
        .bind(("uuid", uuid.to_string()))
        .await
        .unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();

    match id {
        None => StatusCode::NO_CONTENT,
        Some(id) => {
            let _: Option<Cat> = DB
                .delete(id)
                .await
                .unwrap();
            StatusCode::NO_CONTENT
        }
    }
}

async fn root() -> &'static str {
    "Hello World!"
}
