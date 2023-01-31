mod handlers;
mod repositories;

use crate::repositories::{
    label::LabelRepositoryForDB,
    todo::{TodoRepository, TodoRepositoryForDB},
};
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use handlers::{
    label::{all_label, create_label, delete_label},
    todo::{all_todo, create_todo, delete_todo, find_todo, update_todo},
};
use hyper::header::CONTENT_TYPE;
use repositories::label::LabelRepository;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::{env, sync::Arc};
use tower_http::cors::{Any, CorsLayer, Origin};

#[tokio::main]
async fn main() {
    // logging
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
    tracing::debug!("start connect database...");
    let pool = PgPool::connect(database_url)
        .await
        .expect(&format!("fail connect database, url is [{}]", database_url));
    let app = create_app(
        TodoRepositoryForDB::new(pool.clone()),
        LabelRepositoryForDB::new(pool.clone()),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_app<Todo: TodoRepository, Label: LabelRepository>(
    todo_repository: Todo,
    label_repository: Label,
) -> Router {
    Router::new()
        .route("/", get(root))
        // axum は同一パスをメソッドチェーンで記述
        .route("/todos", post(create_todo::<Todo>).get(all_todo::<Todo>))
        .route(
            "/todos/:id",
            get(find_todo::<Todo>)
                .delete(delete_todo::<Todo>)
                .patch(update_todo::<Todo>),
        )
        .route(
            "/labels",
            post(create_label::<Label>).get(all_label::<Label>),
        )
        // .route("/labels/:id", delete(delete_label::<Label>)) // なんか delete はメソッドチェーンしないとエラーになるのでとりあえずコメントアウト
        .layer(Extension(Arc::new(todo_repository)))
        .layer(Extension(Arc::new(label_repository)))
        .layer(
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:3001".parse().unwrap()))
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE]),
        )
}

async fn root() -> &'static str {
    "Hello, world!"
}

// 以下プロダクションコードからは削除される（cfg）
#[cfg(test)]
mod test {
    use super::*;
    use crate::repositories::label::test_utils::LabelRepositoryForMemory;
    use crate::repositories::todo::{
        test_utils::TodoRepositoryForMemory, CreateTodo, TodoWithLabelFromRow,
    };
    use axum::response::Response;
    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() {
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(body, "Hello, world!");
    }

    fn build_todo_req_with_json(path: &str, method: Method, json_body: String) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(json_body))
            .unwrap()
    }

    fn build_todo_req_with_empty(method: Method, path: &str) -> Request<Body> {
        Request::builder()
            .uri(path)
            .method(method)
            .body(Body::empty())
            .unwrap()
    }

    async fn res_to_todo(res: Response) -> TodoWithLabelFromRow {
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: TodoWithLabelFromRow = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body:{}", body));
        todo
    }

    #[tokio::test]
    async fn should_create_todo() {
        let expected = TodoWithLabelFromRow::new(1, "shoud_return_created_todo".to_string());
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();
        let req = build_todo_req_with_json(
            "/todos",
            Method::POST,
            r#"{"text":"shoud_return_created_todo"}"#.to_string(),
        );
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_find_todo() {
        let expected = TodoWithLabelFromRow::new(1, "should_find_todo".to_string());
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();
        todo_repository
            .create(CreateTodo::new("should_find_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_empty(Method::GET, "/todos/1");
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn shoud_get_all_todos() {
        let expected = TodoWithLabelFromRow::new(1, "should_get_all_todos".to_string());
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();
        todo_repository
            .create(CreateTodo::new("should_get_all_todos".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_empty(Method::GET, "/todos");
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body: String = String::from_utf8(bytes.to_vec()).unwrap();
        let todo: Vec<TodoWithLabelFromRow> = serde_json::from_str(&body)
            .expect(&format!("cannot convert Todo instance. body:{}", body));
        assert_eq!(vec![expected], todo);
    }

    #[tokio::test]
    async fn should_update_todo() {
        let expected = TodoWithLabelFromRow::new(1, "should_update_todo".to_string());
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();
        todo_repository
            .create(CreateTodo::new("before_update_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_json(
            "/todos/1",
            Method::PATCH,
            r#"{"id":1, "text":"should_update_todo", "completed":false}"#.to_string(),
        );
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        let todo = res_to_todo(res).await;
        assert_eq!(expected, todo);
    }

    #[tokio::test]
    async fn should_delete_todo() {
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();
        todo_repository
            .create(CreateTodo::new("should_delete_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_empty(Method::DELETE, "/todos/1");
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(StatusCode::NO_CONTENT, res.status());
    }

    #[tokio::test]
    async fn should_not_delete_todo() {
        // id が違ったら 404で削除されない
        let todo_repository = TodoRepositoryForMemory::new();
        let label_repository = LabelRepositoryForMemory::new();
        todo_repository
            .create(CreateTodo::new("should_delete_todo".to_string()))
            .await
            .expect("failed create todo");
        let req = build_todo_req_with_empty(Method::DELETE, "/todos/2");
        let res = create_app(todo_repository, label_repository)
            .oneshot(req)
            .await
            .unwrap();
        assert_eq!(StatusCode::NOT_FOUND, res.status());
    }
}
