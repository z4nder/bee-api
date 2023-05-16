use crate::{
    dto::spend_dto::{StoreSpendPayload, UpdateSpendPayload},
    errors::AppError,
    model::{spend::Spend, user::User},
    repository::spend_repository::SpendRepository,
};

pub struct SpendService;

impl SpendService {
    pub async fn index(
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<Vec<Spend>, AppError> {
        let spends = spend_repository.index(user).await?;

        Ok(spends)
    }

    pub async fn store(
        payload: StoreSpendPayload,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<u64, AppError> {
        let id = spend_repository.store(payload, user).await?;

        Ok(id)
    }

    pub async fn find(
        id: u64,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<Spend, AppError> {
        todo!();
    }

    pub async fn update(
        payload: UpdateSpendPayload,
        id: u64,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<u64, AppError> {
        todo!();
    }

    pub async fn destroy(
        id: u64,
        spend_repository: SpendRepository,
        user: User,
    ) -> Result<u64, AppError> {
        todo!();
    }
}
