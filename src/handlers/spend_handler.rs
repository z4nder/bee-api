use axum::{
    extract::{Path, State},
    Json,
};
use axum_macros::debug_handler;

use crate::{
    dto::spend_dto::{StoreSpendPayload, UpdateSpendPayload},
    errors::AppError,
    model::{spend::Spend, user::User},
    repository::spend_repository::SpendRepository,
    services::spend_service::SpendService,
};

pub async fn index(
    State(spend_repository): State<SpendRepository>,
    user: User,
) -> Result<Json<Vec<Spend>>, AppError> {
    let spends = SpendService::index(spend_repository, user).await?;

    Ok(Json(spends))
}

pub async fn find(
    Path(id): Path<u64>,
    State(spend_repository): State<SpendRepository>,
    user: User,
) -> Result<Json<Spend>, AppError> {
    todo!();
}

pub async fn store(
    State(spend_repository): State<SpendRepository>,
    user: User,
    Json(payload): Json<StoreSpendPayload>,
) -> Result<Json<u64>, AppError> {
    let id = SpendService::store(payload, spend_repository, user).await?;

    Ok(Json(id))
}

pub async fn update(
    Path(id): Path<u64>,
    State(spend_repository): State<SpendRepository>,
    user: User,
    Json(payload): Json<UpdateSpendPayload>,
) {
    todo!();
}

pub async fn destroy(
    Path(id): Path<u64>,
    State(spend_repository): State<SpendRepository>,
    user: User,
) {
    todo!();
}
