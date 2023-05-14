use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use reqwest::StatusCode;

use crate::{
    dto::tag_dto::{StoreTagPayload, UpdateTagPayload},
    errors::AppError,
    model::{tag::Tag, user::User},
    repository::tag_repository::{self, TagRepository},
    services::tag_services::TagService,
};

#[debug_handler]
pub async fn index(
    State(tag_repository): State<TagRepository>,
    user: User,
) -> Result<Json<Vec<Tag>>, AppError> {
    let tags = TagService::index(tag_repository, user).await?;

    Ok(Json(tags))
}

pub async fn find(
    Path(id): Path<u64>,
    State(tag_repository): State<TagRepository>,
    user: User,
) -> Result<Json<Tag>, AppError> {
    let tag = TagService::find(id, tag_repository, user).await?;

    Ok(Json(tag))
}

pub async fn store(
    State(tag_repository): State<TagRepository>,
    user: User,
    Json(payload): Json<StoreTagPayload>,
) -> Result<impl IntoResponse, AppError> {
    let created_tag_id = TagService::store(payload, tag_repository, user)
        .await
        .map_err(|_| AppError::TagInternalError)?;

    Ok((StatusCode::CREATED, Json(created_tag_id)))
}

pub async fn update(
    Path(id): Path<u64>,
    State(tag_repository): State<TagRepository>,
    user: User,
    Json(payload): Json<UpdateTagPayload>,
) -> Result<impl IntoResponse, AppError> {
    let updated_id = TagService::update(payload, id, tag_repository, user)
        .await
        .map_err(|_| AppError::TagInternalError)?;

    Ok((StatusCode::OK, Json(updated_id)))
}

pub async fn destroy(
    Path(id): Path<u64>,
    State(tag_repository): State<TagRepository>,
    user: User,
) -> Result<impl IntoResponse, AppError> {
    let deleted_id = TagService::destroy(id, tag_repository, user)
        .await
        .map_err(|_| AppError::TagInternalError)?;

    Ok((StatusCode::OK, Json(deleted_id)))
}
