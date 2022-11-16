mod error;
mod repository;
mod schema;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::{Logger, NormalizePath};
use error::ApiError;
use repository::{NewPost, Repository};

#[actix_web::post("/posts")]
async fn create_post(
    repo: web::Data<Repository>,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let new_post = new_post.into_inner();
    let post = repo.create_post(new_post).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::get("/posts")]
async fn list_posts(
    repo: web::Data<Repository>,
) -> Result<HttpResponse, ApiError> {
    let posts = repo.list_posts().await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[actix_web::get("/posts/{id}")]
async fn get_post(
    repo: web::Data<Repository>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let post = repo.get_post(id).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().init();
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let repo = web::Data::new(Repository::new(&database_url));
    HttpServer::new(move ||
            App::new()
            .app_data(repo.clone())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .service(create_post)
            .service(list_posts)
            .service(get_post)
        )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
