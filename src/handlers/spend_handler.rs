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
};

#[debug_handler]
pub async fn index(
    State(spend_repository): State<SpendRepository>,
    user: User,
) -> Result<Json<Vec<Spend>>, AppError> {
    todo!();
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
) {
    todo!();
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
