use crate::{
    errors::posts_errors::PostError,
    models::posts_models::{
        CreatePost, DeletePost, GetAllPosts, GetPost, UpdatePost,
    },
    repositories::posts_repository::PostsRepository,
};
use actix_web::{
    HttpResponse, Result, delete, get, post, put,
    web::{Data, Json, ServiceConfig},
};
use sqlx::PgPool;
use validator::Validate;

#[post("/posts")]
pub async fn create_post(
    post_data: Json<CreatePost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    post_data.validate().map_err(PostError::Validation)?;

    let post = PostsRepository::create(&pool, post_data.into_inner()).await?;

    Ok(HttpResponse::Ok().json(post))
}

#[get("/posts/all")]
pub async fn get_all_posts(
    pool: Data<PgPool>,
    post_data: Json<GetAllPosts>,
) -> Result<HttpResponse, PostError> {
    let posts = PostsRepository::get_all(&pool, post_data.user_id).await?;

    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts")]
pub async fn get_posts(
    post_data: Json<GetPost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let post = PostsRepository::find_by_id(&pool, post_data.id).await?;

    Ok(HttpResponse::Ok().json(post))
}

#[put("/posts")]
pub async fn update_posts(
    post_data: Json<UpdatePost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    post_data.validate().map_err(PostError::Validation)?;

    // Обновление пользователя
    let updated_post =
        PostsRepository::update(&pool, post_data.id, post_data.into_inner())
            .await?;

    Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/posts")]
async fn delete_post(
    post_data: Json<DeletePost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    PostsRepository::delete(&pool, post_data.id).await?;
    Ok(HttpResponse::Ok().json(()))
}

pub fn posts_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_post)
        .service(get_all_posts)
        .service(update_posts)
        .service(get_posts)
        .service(delete_post);
}
