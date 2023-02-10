use axum::{
    extract,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv;
use entity::post;
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DbConn, DbErr};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/posts", post(create_post));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn hello_world() -> Json<HelloWorld> {
    let hello = HelloWorld {
        text: "Hello World".to_string(),
    };
    Json(hello)
}

#[derive(Serialize)]
struct HelloWorld {
    text: String,
}

#[derive(Deserialize)]
struct CreatePost {
    title: String,
    body: String,
}

#[derive(Serialize)]
struct Post {
    title: String,
    body: String,
}

async fn create_post(
    extract::Json(payload): extract::Json<CreatePost>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let post = post::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::set(payload.title.to_string()),
        body: ActiveValue::set(payload.body.to_string()),
        published: ActiveValue::set(false),
    };
    let post = post.insert(&db).await.unwrap(); //or(Err(StatusCode::NOT_FOUND));

    Ok(Json(Post {
        title: post.title.to_string(),
        body: post.body.to_string(),
    }))
}

pub async fn connection() -> Result<DbConn, DbErr> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    Ok(db)
}
