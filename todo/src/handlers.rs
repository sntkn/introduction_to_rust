use crate::repositories::{CreateTodo, TodoRepository, UpdateTodo};
use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, RequestParts},
    http::StatusCode,
    response::IntoResponse,
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait] // trait で async を実装するとき
impl<T, B> FromRequest<B> for ValidatedJson<T>
where // where はジェネリック境界を宣言
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, String); // FromRequestに必要でエラーのレスポンス型

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await.map_err(|rejection| {
            let message = format!("Json parse error: [{}]", rejection);
            (StatusCode::BAD_REQUEST, message)
        })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation eeror: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn create_todo<T: TodoRepository>(
    ValidatedJson(payload): ValidatedJson<CreateTodo>, // request は deserialize
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todo = repository.create(payload);
    (StatusCode::CREATED, Json(todo)) // response は serialize
}

pub async fn find_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    //todo!();
    // コンパイルエラーを通すため暫定でOKを返す
    //Ok(StatusCode::OK)
    let todo = repository.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    //todo!();
    let todo = repository.all();
    (StatusCode::OK, Json(todo))
}

pub async fn update_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
    Extension(repository): Extension<Arc<T>>,
) -> Result<impl IntoResponse, StatusCode> {
    //todo!();
    // コンパイルエラーを通すため暫定でOKを返す
    //Ok(StatusCode::OK)
    let todo = repository
        .update(id, payload)
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn delete_todo<T: TodoRepository>(
    Path(id): Path<i32>,
    Extension(repository): Extension<Arc<T>>,
) -> StatusCode {
    //todo!()
    repository
        .delete(id)
        .map(|_| StatusCode::NO_CONTENT) // Return OK
        .unwrap_or(StatusCode::NOT_FOUND) // ERR: Return 404
}
