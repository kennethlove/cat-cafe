use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use surrealdb::RecordId;
use uuid::Uuid;
use serde::Deserialize;
use shared::{Cat, CatStatus, NewCat};
use crate::DB;

#[derive(Deserialize)]
pub struct Sorting {
    sort_by_field: String,
    sort_direction: String,
}

#[derive(Deserialize)]
pub struct Filtering {
    filter_by_status: String,
}

pub async fn get_cats(sorting: Query<Sorting>, filtering: Query<Filtering>) -> Result<impl IntoResponse, StatusCode> {
    DB.use_ns("cat-cafe").use_db("cats").await.expect("Failed to use cat database!");
    match DB.select("cat").await {
        Ok(mut cats) => {
            match sorting.sort_by_field.to_lowercase().as_str() {
                "breed" => { cats.sort_by_key(|c: &Cat| c.breed.clone()); },
                "microchip" => { cats.sort_by_key(|c: &Cat| c.microchip.clone()); },
                _ => { cats.sort_by_key(|c: &Cat| c.name.clone()); },
            }
            match sorting.sort_direction.to_lowercase().as_str() {
                "desc" => { cats.reverse(); },
                _ => {}
            }
            let cats = match filtering.filter_by_status.to_lowercase().as_str() {
                "new" => {
                    cats.iter().filter(|c| c.status == CatStatus::New).cloned().collect::<Vec<Cat>>()
                },
                "waiting" => {
                    cats.iter().filter(|c| c.status == CatStatus::Waiting).cloned().collect::<Vec<Cat>>()
                },
                "in-cafe" => {
                    cats.iter().filter(|c| c.status == CatStatus::InCafe).cloned().collect::<Vec<Cat>>()
                },
                "fostered" => {
                    cats.iter().filter(|c| c.status == CatStatus::Fostered).cloned().collect::<Vec<Cat>>()
                },
                "adopted" => {
                    cats.iter().filter(|c| c.status == CatStatus::Adopted).cloned().collect::<Vec<Cat>>()
                },
                _ => cats.clone(),
            };
            Ok((StatusCode::OK, Json::<Vec<Cat>>(cats)))
        },
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
