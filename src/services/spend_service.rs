use crate::{
    dto::spend_dto::{StoreSpendPayload, UpdateSpendPayload},
    errors::{ApiError, AppError},
    model::{spend::Spend, user::User},
    repository::spend_repository::SpendRepository,
};

pub struct SpendService;

impl SpendService {
    pub async fn index(
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<Vec<Spend>, ApiError> {
        let spends = spend_repository.index(user).await?;

        Ok(spends)
    }

    pub async fn store(
        payload: StoreSpendPayload,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<u64, ApiError> {
        let id = spend_repository.store(payload, user).await?;

        Ok(id)
    }

    pub async fn find(
        id: u64,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<Spend, ApiError> {
        todo!();
    }

    pub async fn update(
        payload: UpdateSpendPayload,
        id: u64,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<u64, ApiError> {
        todo!();
    }

    pub async fn destroy(
        id: u64,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<u64, ApiError> {
        todo!();
    }
}
