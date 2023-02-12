use axum::{
    extract,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use dotenv;
use entity::post::{self, ActiveModel};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, DbConn, DbErr, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/posts", post(create_post).get(all_post))
        .route("/posts/:id", get(find_post).patch(update_post));
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
#[derive(Deserialize)]
struct UpdatePost {
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

    Ok(Json(ResponsePost {
        id: post.id,
        title: post.title.to_string(),
        body: post.body.to_string(),
    }))
}

async fn find_post(Path(id): Path<i32>) -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let post = post::Entity::find_by_id(id).one(&db).await.unwrap();

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
    extract::Json(payload): extract::Json<UpdatePost>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let mut post: ActiveModel = post::Entity::find_by_id(id)
        .one(&db)
        .await
        .unwrap()
        .unwrap()
        .into();
    post.title = Set(payload.title);
    post.body = Set(payload.body);
    let post = post.update(&db).await.unwrap();

    Ok(Json(ResponsePost {
        id,
        title: post.title.to_string(),
        body: post.body.to_string(),
    }))
}

async fn all_post() -> Result<impl IntoResponse, StatusCode> {
    let db = connection().await.unwrap();
    let posts = post::Entity::find().all(&db).await.unwrap();
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

pub async fn connection() -> Result<DbConn, DbErr> {
    let database_url = dotenv::var("DATABASE_URL").unwrap();
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to setup the database");

    Ok(db)
}