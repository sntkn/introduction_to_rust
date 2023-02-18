use axum::{
    extract,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv;
use sea_orm::{Database, DbConn, DbErr};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
mod repository;
use crate::repository::{PostCreate, PostMutation, PostQuery, PostUpdate};

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/posts", post(create_post).get(all_post))
        .route(
            "/posts/:id",
            get(find_post).patch(update_post).delete(delete_post),
        );
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
struct RequestCreatePost {
    title: String,
    body: String,
}

#[derive(Deserialize)]
struct RequestUpdatePost {
    title: String,
    body: String,
}

#[derive(Serialize)]
struct ResponsePost {
    id: i32,
    title: String,
    body: String,
}

async fn create_post(
    extract::Json(payload): extract::Json<RequestCreatePost>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let data = PostCreate {
        title: payload.title,
        body: payload.body,
    };
    let post = PostMutation::create_post(&db, data).await.unwrap();

    Ok(Json(ResponsePost {
        id: post.id,
        title: post.title.to_string(),
        body: post.body.to_string(),
    }))
}

async fn find_post(Path(id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let post = PostQuery::find_post_by_id(&db, id).await.unwrap();

    match post {
        Some(post) => Ok(Json(ResponsePost {
            id,
            title: post.title.to_string(),
            body: post.body.to_string(),
        })),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_post(
    Path(id): Path<i32>,
    extract::Json(payload): extract::Json<RequestUpdatePost>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let data = PostUpdate {
        title: payload.title,
        body: payload.body,
    };
    let post = PostMutation::update_post_by_id(&db, id, data)
        .await
        .unwrap();

    Ok(Json(ResponsePost {
        id,
        title: post.title.to_string(),
        body: post.body.to_string(),
    }))
}

async fn all_post() -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let posts = PostQuery::find_all_posts(&db).await.unwrap();
    let mut accum: Vec<ResponsePost> = vec![];
    for p in posts.iter() {
        accum.push(ResponsePost {
            id: p.id,
            title: p.title.to_string(),
            body: p.body.to_string(),
        })
    }
    Ok(Json(accum))
}

async fn delete_post(Path(id): Path<i32>) -> StatusCode {
    let db = connection().await.unwrap();
    PostMutation::delete_post_by_id(&db, id)
        .await
        .map(|res| {
            // todo: もうちょっと良い書き方・・・
            if res.rows_affected == 1 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        })
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn connection() -> Result<DbConn, DbErr> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    Ok(db)
}
