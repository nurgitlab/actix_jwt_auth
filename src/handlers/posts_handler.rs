use crate::{
    errors::posts_errors::PostError,
    models::{auth_models::Claims, posts_models::{CreatePost, DeletePost, GetAllPosts, GetPost, UpdatePost}},
    repositories::posts_repository::PostsRepository,
};
use actix_web::{
    delete, get, post, put, web::{scope, Data, Json, Path, ServiceConfig}, HttpMessage, HttpRequest, HttpResponse, Result
};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};
use sqlx::PgPool;
use validator::Validate;

// Хелпер для извлечения user_id из токена
fn extract_user_id(req: &HttpRequest) -> Result<i32, PostError> {
    req.extensions()
        .get::<Claims>()
        .map(|claims| claims.sub)
        .ok_or(PostError::NotFound)
}

#[post("")]
pub async fn create_post(
    req: HttpRequest,
    post_data: Json<CreatePost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let user_id = extract_user_id(&req)?;
    post_data.validate().map_err(PostError::Validation)?;

    // Проверяем, что пользователь создает пост для себя
    if post_data.user_id != user_id {
        return Err(PostError::NotFound);
    }

    let post = PostsRepository::create(&pool, post_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[get("/all")]
pub async fn get_all_posts(
    req: HttpRequest,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let user_id = extract_user_id(&req)?;
    let posts = PostsRepository::get_all(&pool, user_id).await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{id}")]
pub async fn get_post(
    req: HttpRequest,
    path: Path<i32>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let _user_id = extract_user_id(&req)?; // Проверяем авторизацию
    let post_id = path.into_inner();
    let post = PostsRepository::find_by_id(&pool, post_id).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[put("/{id}")]
pub async fn update_post(
    req: HttpRequest,
    path: Path<i32>,
    post_data: Json<UpdatePost>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let user_id = extract_user_id(&req)?;
    post_data.validate().map_err(PostError::Validation)?;

    // Проверяем, что пользователь обновляет свой пост
    let post_id = path.into_inner();
    let post = PostsRepository::find_by_id(&pool, post_id).await?;
    if post.user_id != user_id {
        return Err(PostError::NotFound);
    }

    let updated_post = PostsRepository::update(&pool, post_id, post_data.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated_post))
}

#[delete("/{id}")]
pub async fn delete_post(
    req: HttpRequest,
    path: Path<i32>,
    pool: Data<PgPool>,
) -> Result<HttpResponse, PostError> {
    let user_id = extract_user_id(&req)?;
    let post_id = path.into_inner();

    // Проверяем, что пользователь удаляет свой пост
    let post = PostsRepository::find_by_id(&pool, post_id).await?;
    if post.user_id != user_id {
        return Err(PostError::NotFound);
    }

    PostsRepository::delete(&pool, post_id).await?;
    Ok(HttpResponse::Ok().json(()))
}

pub fn posts_routes(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(crate::middlewares::auth_middleware::auth_middleware_validator);

    cfg.service(
        scope("/posts")
            .wrap(auth)
            .service(create_post)
            .service(get_all_posts)
            .service(get_post)
            .service(update_post)
            .service(delete_post),
    );
}