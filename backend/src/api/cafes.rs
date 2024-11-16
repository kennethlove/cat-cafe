use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use shared::{Cafe, NewCafe};
use crate::DB;
use uuid::Uuid;

pub async fn get_cafes() -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cafes").await.expect("Failed to use cafe database!");
    match DB.select("cafe").await {
        Ok(cafes) => Ok((StatusCode::OK, Json::<Vec<Cafe>>(cafes))),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_cafe(Path(uuid): Path<uuid::Uuid>) -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cafes").await.expect("Failed to use cafe database!");
    tracing::debug!("getting cafe with uuid: {}", uuid);
    let mut response = DB
        .query("SELECT * FROM cafe WHERE identifier = $uuid LIMIT 1")
        .bind(("uuid", uuid.to_string()))
        .await
        .unwrap();

    let cafe: Option<Cafe> = response.take(0).unwrap();
    Ok((StatusCode::OK, Json(cafe)))
}

pub async fn create_cafe(Json(payload): Json<NewCafe>) -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cafes").await.expect("Failed to use cafe database!");
    let identifier = Uuid::new_v4();
    let cafe: Option<Cafe> = DB
        .create("cafe")
        .content(Cafe {
            identifier: identifier.to_string(),
            name: payload.name.clone(),
        })
        .await
        .unwrap();
    Ok((StatusCode::CREATED, Json(cafe.unwrap())))
}