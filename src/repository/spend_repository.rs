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
        let query = sqlx::query_as!(
            Spend,
            r#"SELECT * FROM spends WHERE created_by = ?"#,
            owner.id
        )
        .fetch_all(&self.db_connection)
        .await;

        if let Ok(spends) = query {
            Ok(spends)
        } else {
            Err(AppError::SpendInternalError)
        }
    }

    pub async fn find(&self, id: u64, owner: User) -> Result<Spend, AppError> {
        todo!();
    }

    pub async fn store(&self, payload: StoreSpendPayload, owner: User) -> Result<u64, AppError> {
        let insert_id = sqlx::query_as!(
            Spend,
            r#"INSERT INTO spends (name, date, value, created_by) VALUES (?, ?, ?, ?)"#,
            payload.name,
            payload.date,
            payload.value,
            owner.id
        )
        .execute(&self.db_connection)
        .await
        .map_err(|_| AppError::SpendInternalError)?
        .last_insert_id();

        Ok(insert_id)
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
