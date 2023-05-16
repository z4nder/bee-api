use sqlx::MySqlPool;

use crate::dto::spend_dto::{StoreSpendPayload, UpdateSpendPayload};
use crate::errors::ApiError;
use crate::model::spend::Spend;
use crate::{errors::AppError, model::user::User};

#[derive(Clone)]
pub struct SpendRepository {
    pub db_connection: MySqlPool,
}

impl SpendRepository {
    pub async fn index(&self, owner: User) -> Result<Vec<Spend>, ApiError> {
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
            Err(ApiError::new(AppError::SpendInternalError, None))
        }
    }

    pub async fn find(&self, id: u64, owner: User) -> Result<Spend, ApiError> {
        todo!();
    }

    pub async fn store(&self, payload: StoreSpendPayload, owner: User) -> Result<u64, ApiError> {
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
        .map_err(|error| ApiError::new(AppError::SpendInternalError, Some(error.to_string())))?
        .last_insert_id();

        for tag_id in payload.tags {
            sqlx::query_as!(
                Spend,
                r#"INSERT INTO spends_tags (spend_id, tag_id) VALUES (?, ?)"#,
                insert_id,
                tag_id
            )
            .execute(&self.db_connection)
            .await
            .map_err(|error| ApiError::new(AppError::SpendInternalError, Some(error.to_string())))?
            .last_insert_id();
        }

        Ok(insert_id)
    }

    pub async fn update(
        &self,
        id: u64,
        payload: UpdateSpendPayload,
        owner: User,
    ) -> Result<u64, ApiError> {
        todo!();
    }

    pub async fn destroy(&self, id: u64, owner: User) -> Result<u64, ApiError> {
        todo!();
    }
}
