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
