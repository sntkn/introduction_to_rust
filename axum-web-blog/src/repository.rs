use ::entity::{
    post,
    post::{ActiveModel, Entity as Post},
};
use sea_orm::*;

pub struct UpdatePost {
    pub title: String,
    pub body: String,
}
pub struct Query;
pub struct Mutation;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<post::Model>, DbErr> {
        Post::find_by_id(id).one(db).await
    }
}

impl Mutation {
    pub async fn update_post_by_id(
        db: &DbConn,
        id: i32,
        payload: UpdatePost,
    ) -> Result<post::Model, DbErr> {
        let mut post: ActiveModel = post::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find post.".to_owned()))?
            .into();
        post.title = Set(payload.title);
        post.body = Set(payload.body);
        post.update(db).await
    }
}
