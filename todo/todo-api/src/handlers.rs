use axum::{
  async_trait,
  extract::{FromRequest, RequestParts},
  http::StatusCode,
  BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

pub mod todo;

#[derive(Debug)]
pub struct ValidatedJson<T>(T);


#[async_trait] // trait で async を実装するとき
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    // where はジェネリック境界を宣言
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
