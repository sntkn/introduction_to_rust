use super::RepositoryError;
use axum::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[async_trait]
pub trait LabelRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, name: String) -> anyhow::Result<Label>;
    async fn all(&self) -> anyhow::Result<Vec<Label>>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Label {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateLabel {
    id: i32,
    name: String,
}

#[derive(Debug, Clone)]
pub struct LabelRepositoryForDB {
    pool: PgPool,
}

impl LabelRepositoryForDB {
    pub fn new(pool: PgPool) -> Self {
        LabelRepositoryForDB { pool }
    }
}

#[async_trait]
impl LabelRepository for LabelRepositoryForDB {
    async fn create(&self, name: String) -> anyhow::Result<Label> {
        let optional_label = sqlx::query_as::<_, Label>(
            r#"
            select * from labels where name=$1
        "#,
        )
        .bind(name.clone())
        .fetch_optional(&self.pool)
        .await?;

        if let Some(label) = optional_label {
            return Err(RepositoryError::Duplicate(label.id).into());
        }

        let label = sqlx::query_as::<_, Label>(
            r#"
            insert into labels (name)
            values ($1)
            returning *
            "#,
        )
        .bind(name.clone())
        .fetch_one(&self.pool)
        .await?;

        Ok(label)
    }

    async fn all(&self) -> anyhow::Result<Vec<Label>> {
        let label = sqlx::query_as::<_, Label>(
            r#"
            select * from labels order by labels.id asc
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(label)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            delete from labels where id=$1
        "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound(id),
            _ => RepositoryError::Unexpected(e.to_string()),
        })?;

        Ok(())
    }
}

#[cfg(test)]
#[cfg(feature = "database-test")]
mod test {
    use super::*;
    use dotenv::dotenv;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    async fn crud_scenario() {
        dotenv().ok();
        let database_url = &env::var("DATABASE_URL").expect("undefined [DATABASE_URL]");
        let pool = PgPool::connect(database_url)
            .await
            .expect(&format!("fail connect database, url is [{}]", database_url));
        let repository = LabelRepositoryForDB::new(pool);
        let label_text = "test label a";

        // create
        let label = repository
            .create(label_text.to_string())
            .await
            .expect("[create] returned Err");
        assert_eq!(label.name, label_text);

        // all
        let labels = repository.all().await.expect("[all] returned Err");
        let label = labels.last().unwrap(); // todos でラベル作ってる関係？
        assert_eq!(label.name, label_text);

        // delete
        repository
            .delete(label.id)
            .await
            .expect("[delete] returned Err");
    }
}

#[cfg(test)]
pub mod test_utils {
    use axum::async_trait;
    use std::collections::HashMap;

    use super::*;

    impl Label {}

    type LabelDatas = HashMap<i32, Label>;

    #[derive(Debug, Clone)]
    pub struct LabelRepositoryForMemory {}

    impl LabelRepositoryForMemory {
        pub fn new() -> Self {
            LabelRepositoryForMemory {}
        }
    }

    #[async_trait]
    impl LabelRepository for LabelRepositoryForMemory {
        async fn create(&self, _name: String) -> anyhow::Result<Label> {
            todo!()
        }

        async fn all(&self) -> anyhow::Result<Vec<Label>> {
            todo!()
        }

        async fn delete(&self, _id: i32) -> anyhow::Result<()> {
            todo!()
        }
    }
}
