use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use surrealdb::RecordId;
use uuid::Uuid;
use shared::{Cat, NewCat};
use crate::DB;

pub async fn get_cats() -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cats").await.expect("Failed to use cat database!");
    match DB.select("cat").await {
        Ok(cats) => Ok((StatusCode::OK, Json::<Vec<Cat>>(cats))),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_cat(Path(uuid): Path<Uuid>) -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cats").await.expect("Failed to use cat database!");
    let mut response = DB
        .query("SELECT * FROM cat WHERE identifier = $uuid LIMIT 1")
        .bind(("uuid", uuid.to_string()))
        .await
        .unwrap();

    let cat: Option<Cat> = response.take(0).unwrap();
    Ok((StatusCode::OK, Json(cat)))
}

pub async fn create_cat(Json(payload): Json<NewCat>) -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cats").await.expect("Failed to use cat database!");
    let identifier = Uuid::new_v4();
    let cat: Option<Cat> = DB
        .create("cat")
        .content(NewCat {
            identifier: Some(identifier.to_string()),
            name: payload.name.clone(),
            breed: payload.breed.clone(),
            microchip: payload.microchip.clone(),
            image: payload.image.clone(),
        })
        .await
        .unwrap();
    Ok((StatusCode::CREATED, Json(cat.unwrap())))
}

pub async fn update_cat(
    Path(uuid): Path<Uuid>,
    Json(payload): Json<NewCat>,
) -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cats").await.expect("Failed to use cat database!");
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
                    microchip: payload.microchip.clone(),
                    image: payload.image.clone(),
                })
                .await
                .unwrap();

            Ok((StatusCode::OK, Json(Some(cat))))
        }
    }
}

pub async fn delete_cat(Path(uuid): Path<Uuid>) -> StatusCode {
    DB.use_ns("cat-cafe").use_db("cats").await.expect("Failed to use cat database!");
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
