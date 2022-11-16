use crate::error::ApiError;
use crate::schema::*;
use actix_web::web;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize, Deserializer};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    title: String,
    #[serde(deserialize_with = "max200")]
    body: String,
}

// serde のデシリアライズ時に検証するパターン
fn max200<'de, D>(de: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    String::deserialize(de).and_then(|s| {
        if !s.is_empty() && s.len() < 200 {
            Ok(s)
        } else {
            Err(serde::de::Error::custom(
                "string length is 0 or too long",
            ))
        }
    })
}

#[derive(Serialize, Queryable)]
pub struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
}

pub struct Repository {
    pool: DbPool,
}

impl NewPost {
    pub fn validate(&self) -> bool {
        self.title.len() > 0 && self.title.len() <= 100
    }
}

impl Repository {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create a pool.");
        Self { pool }
    }

    pub async fn create_post(&self, new_post: NewPost) -> Result<Post, ApiError> {
        let mut conn = self.pool.get()?;
        let post = web::block(move || {
            // 別スレッドに move するから new_post は使えなくなる
            diesel::insert_into(posts::table)
                .values(new_post)
                .get_result(&mut conn)
        })
        .await??;

        Ok(post)
    }

    pub async fn list_posts(&self) -> Result<Vec<Post>, ApiError> {
        let mut conn = self.pool.get()?;
        let res = web::block(move || {
            posts::table.load(&mut conn)
        })
        .await??;

        Ok(res)
    }

    pub async fn get_post(
        &self,
        id:i32,
    ) -> Result<Post, ApiError> {
        let mut conn = self.pool.get()?;
        let res = web::block(move || {
            posts::table
                .find(id)
                .first(&mut conn)
                .optional()
        })
        .await??
        .ok_or(ApiError::NotFound)?;

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conn() {
        let database_url = std::env::var("DATABASE_URL").unwrap();
        let repo = Repository::new(&database_url);
        assert!(repo.pool.get().is_ok());
    }
}
