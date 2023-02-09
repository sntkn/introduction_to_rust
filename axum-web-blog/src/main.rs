use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
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

async fn create_post(extract::Json(payload): extract::Json<CreatePost>) -> Json<Post> {
    let post = Post {
        title: payload.title.to_string(),
        body: payload.body.to_string(),
    };
    Json(post)
}
