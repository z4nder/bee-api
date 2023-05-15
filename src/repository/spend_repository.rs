use sqlx::MySqlPool;

use crate::dto::spend_dto::{StoreSpendPayload, UpdateSpendPayload};
use crate::model::spend::Spend;
use crate::{errors::AppError, model::user::User};

#[derive(Clone)]
pub struct SpendRepository {
    pub db_connection: MySqlPool,
}

impl SpendRepository {
    pub async fn index(&self, owner: User) -> Result<Vec<Spend>, AppError> {
        todo!();
    }

    pub async fn find(&self, id: u64, owner: User) -> Result<Spend, AppError> {
        todo!();
    }

    pub async fn store(&self, payload: StoreSpendPayload, owner: User) -> Result<u64, AppError> {
        todo!();
    }

    pub async fn update(
        &self,
        id: u64,
        payload: UpdateSpendPayload,
        owner: User,
    ) -> Result<u64, AppError> {
        todo!();
    }

    pub async fn destroy(&self, id: u64, owner: User) -> Result<u64, AppError> {
        todo!();
    }
}
