use axum::{
    extract::Path,
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{RecordId, Response, Surreal};
use surrealdb::sql::{Thing, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Cat {
    id: RecordId,
    identifier: Uuid,
    name: String,
    breed: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct NewCat {
    identifier: Option<Uuid>,
    name: String,
    breed: String,
}

static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

#[tokio::main]
async fn main() {
    DB.connect::<Ws>("localhost:8000").await.unwrap();
    DB.signin(Root {
        username: "root",
        password: "root",
    }).await.unwrap();
    DB.use_ns("cat-cafe").use_db("cats").await.unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/cats/:uuid", get(get_cat).delete(delete_cat))
        .route("/cats", get(get_cats).post(create_cat));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_cats() -> (StatusCode, Json<Vec<Cat>>) {
    match DB.select("cat").await {
        Ok(cats) => (StatusCode::OK, Json(cats)),
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

async fn get_cat(Path(uuid): Path<Uuid>) -> (StatusCode, Json<Option<Cat>>) {
    let mut response= DB
        .query("SELECT * FROM cat WHERE identifier = $uuid")
        .bind(("uuid", uuid))
        .await.unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();
    let name: Option<String> = response.take("name").unwrap();
    let breed: Option<String> = response.take("breed").unwrap();

    match id {
        None => (StatusCode::NOT_FOUND, Json(None)),
        _ => {
            let cat = Cat {
                id: id.unwrap(),
                identifier: uuid,
                name: name.unwrap(),
                breed: breed.unwrap(),
            };
            (StatusCode::OK, Json(Some(cat)))
        }
    }
}

async fn create_cat(
    Json(payload): Json<NewCat>,
) -> (StatusCode, Json<Cat>) {
    let identifier = Uuid::new_v4();
    let cat: Option<Cat> = DB
        .create("cat")
        .content(NewCat {
            identifier: Some(identifier),
            name: payload.name.clone(),
            breed: payload.breed.clone(),
        })
        .await.unwrap();
    (StatusCode::CREATED, Json(cat.unwrap()))
}

async fn delete_cat(Path(uuid): Path<Uuid>) -> StatusCode {
    let mut response= DB
        .query("SELECT * FROM cat WHERE identifier = $uuid")
        .bind(("uuid", uuid))
        .await.unwrap();
    let id: Option<RecordId> = response.take("id").unwrap();
    match id {
        None => { StatusCode::NO_CONTENT }
        _ => {
            let id: RecordId = id.unwrap();
            let split_id: Vec<String> = id.to_string().split(':').map(String::from).collect();

            let _:Option<Cat> = DB.delete((split_id[0].clone(), split_id[1].clone())).await.unwrap();
            StatusCode::NO_CONTENT
        }
    }
}

async fn root() -> &'static str {
    "Hello World!"
}
