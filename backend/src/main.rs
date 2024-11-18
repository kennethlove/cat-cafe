mod api;

use std::collections::HashMap;
use api::{
    cafes::{get_cafes, create_cafe, get_cafe},
    cats::{create_cat, delete_cat, get_cat, get_cats, update_cat},
};
use axum::error_handling::HandleErrorLayer;
use axum::response::IntoResponse;
use axum::{extract::Path, http::StatusCode, routing::get, BoxError, Json, Router};
use axum::extract::Multipart;
use axum::routing::post;
use minio_rsc::client::{BucketArgs, KeyArgs};
use minio_rsc::error::Result as MinioResult;
use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;
use std::fs::File;
use std::io::Write;
use std::sync::LazyLock;
use std::time::Duration;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use url::Url;
use uuid::Uuid;

use shared::{FILE_PUBLIC_PATH, FILE_UPLOAD_PATH};

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

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

    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:8080".parse().unwrap()))
        .allow_headers(vec!["content-type".parse().unwrap()])
        .allow_methods(vec![
            "GET".parse().unwrap(),
            "POST".parse().unwrap(),
            "PUT".parse().unwrap(),
            "DELETE".parse().unwrap(),
        ]);

    let app = Router::new()
        .route("/", get(root))
        .route("/cafes", get(get_cafes).post(create_cafe))
        .route("/cafes/:uuid", get(get_cafe))//.put(update_cafe))
        // .route("/cafes/:uuid/cats", get(get_cafe_cats).post(create_cafe_cat))
        .route("/cats", get(get_cats).post(create_cat))
        .route(
            "/cats/:uuid",
            get(get_cat).delete(delete_cat).put(update_cat),
        )
        .route("/cats/:uuid/images", post(upload_image))
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



async fn upload_image(Path(uuid): Path<Uuid>, mut multipart: Multipart) -> Result<impl IntoResponse, StatusCode> {
    while let Some(field) = multipart
        .next_field().await.expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        let extension = field.file_name().unwrap().split('.').last().unwrap();
        let file_name = format!("{}.{}", uuid, extension);
        let content_type = format!("image/{}", extension);
        let bytes = field.bytes().await.unwrap();

        // Create a path for the soon-to-be-created file
        // let file_path = format!("{}{}", FILE_UPLOAD_PATH, file_name);

        let provider = StaticProvider::new("QaSFuiRltxT79hSRQpsk", "OxxsZiTOmE7DEOvlqLoq0D23usZhBr5klWZPcdhJ", None);
        let minio = Minio::builder()
            .endpoint("localhost:9000")
            .provider(provider)
            .region("home")
            .secure(false)
            .build()
            .unwrap();

        let (buckets, owner) = minio.list_buckets().await.unwrap();
        let metadata: HashMap<String, String> = [("filename".to_owned(), file_name.to_owned())].into();
        let key = KeyArgs::new(file_name.clone())
            .content_type(Some(content_type))
            .metadata(metadata);
        minio.put_object(&buckets.first().unwrap().name, key, bytes).await.expect("Failed to upload file!");

        let _file = minio.get_object(&buckets.first().unwrap().name, &file_name).await.expect("Failed to get file!");
        tracing::debug!("uploaded file: {:?}", _file);

        // Open a handle to the file
        // std::fs::create_dir_all(FILE_UPLOAD_PATH).expect("Failed to create directory!");
        // let mut file_handle = File::create(file_path).expect("Failed to open file handler!");

        // Write the data to the file
        // file_handle.write_all(&data).expect("Failed to write to file!");


        return Ok((StatusCode::CREATED, Json(format!("{}{}", FILE_PUBLIC_PATH, file_name))));
    };
    Err(StatusCode::BAD_REQUEST)
}

async fn root() -> &'static str {
    "Hello World!"
}
