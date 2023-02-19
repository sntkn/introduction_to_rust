use ::entity::{
    post,
    post::{ActiveModel, Entity as Post},
};
use sea_orm::*;

pub struct PostUpdate {
    pub title: String,
    pub body: String,
}

pub struct PostCreate {
    pub title: String,
    pub body: String,
}

pub struct PostQuery;
pub struct PostMutation;

impl PostQuery {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }

    pub async fn find_all_posts(db: &DbConn) -> Result<Vec<post::Model>, DbErr> {
        post::Entity::find().all(db).await
    }
}

impl PostMutation {
    pub async fn create_post(db: &DbConn, payload: PostCreate) -> Result<post::Model, DbErr> {
        let post = ActiveModel {
            id: ActiveValue::NotSet,
            title: ActiveValue::set(payload.title.to_string()),
            body: ActiveValue::set(payload.body.to_string()),
            published: ActiveValue::set(false),
        };

        post.insert(db).await
    }

    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        payload: PostUpdate,
    ) -> Result<post::Model, DbErr> {
        let mut post: ActiveModel = Post::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))?
            .into();
        post.title = Set(payload.title);
        post.body = Set(payload.body);

        post.update(db).await
    }

    pub async fn delete_post_by_id(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        post::Entity::delete_by_id(id).exec(db).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[tokio::test]
    async fn find_post_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[post::Model {
                id: 15,
                title: "Test Post".to_owned(),
                body: "This is a test post".to_owned(),
                published: false,
            }]])
            .into_connection();

        let post = PostQuery::find_post_by_id(&db, 15)
            .await
            .expect("[find] returned Err")
            .unwrap();

        assert_eq!("Test Post".to_owned(), post.title);
        assert_eq!("This is a test post".to_owned(), post.body);
    }

    #[tokio::test]
    async fn find_all_posts() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[post::Model {
                id: 15,
                title: "Test Post".to_owned(),
                body: "This is a test post".to_owned(),
                published: false,
            }]])
            .into_connection();

        let posts = PostQuery::find_all_posts(&db)
            .await
            .expect("[all] returned Err");
        assert_eq!("Test Post".to_owned(), posts[0].title);
        assert_eq!("This is a test post".to_owned(), posts[0].body);
    }
    #[tokio::test]

    async fn test_create_post() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[post::Model {
                id: 15,
                title: "Test Post".to_owned(),
                body: "This is a test post".to_owned(),
                published: false,
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 15,
                rows_affected: 1,
            }])
            .into_connection();

        let payload = PostCreate {
            title: "Test Post".to_owned(),
            body: "This is a test post".to_owned(),
        };

        let created = PostMutation::create_post(&db, payload)
            .await
            .expect("[create] returned Err");
        assert_eq!("Test Post".to_owned(), created.title);
        assert_eq!("This is a test post".to_owned(), created.body);
    }

    #[tokio::test]
    async fn test_update_post() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_query_results([[post::Model {
                id: 15,
                title: "Test Post".to_owned(),
                body: "This is a test post".to_owned(),
                published: false,
            }]])
            .append_query_results([[post::Model {
                id: 15,
                title: "Updated Test Post".to_owned(),
                body: "This is an updated test post".to_owned(),
                published: false,
            }]])
            .append_exec_results([MockExecResult {
                last_insert_id: 15,
                rows_affected: 1,
            }])
            .into_connection();

        let payload = PostUpdate {
            title: "Updated Test Post".to_owned(),
            body: "This is an updated test post".to_owned(),
        };

        let updated = PostMutation::update_post_by_id(&db, 15, payload)
            .await
            .expect("[update] returned Err");
        assert_eq!("Updated Test Post".to_owned(), updated.title);
        assert_eq!("This is an updated test post".to_owned(), updated.body);
    }

    #[tokio::test]
    async fn test_delete_post() {
        let db = MockDatabase::new(DatabaseBackend::Sqlite)
            .append_exec_results([MockExecResult {
                last_insert_id: 15,
                rows_affected: 1,
            }])
            .into_connection();

        let deleted = PostMutation::delete_post_by_id(&db, 15)
            .await
            .expect("[delete] returned Err");
        assert_eq!(1, deleted.rows_affected);
    }
}
