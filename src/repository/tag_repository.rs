use sqlx::MySqlPool;

use crate::dto::tag_dto::StoreTagPayload;
use crate::model::tag::Tag;
use crate::{errors::AppError, model::user::User};

#[derive(Clone)]
pub struct TagRepository {
    pub db_connection: MySqlPool,
}

impl TagRepository {
    pub async fn index(&self, owner: User) -> Result<Vec<Tag>, AppError> {
        let query = sqlx::query_as!(Tag, r#"SELECT * FROM tags WHERE created_by = ?"#, owner.id)
            .fetch_all(&self.db_connection)
            .await;

        if let Ok(tags) = query {
            Ok(tags)
        } else {
            Err(AppError::TagInternalError)
        }
    }

    pub async fn store(&self, payload: StoreTagPayload, owner: User) -> Result<u64, AppError> {
        let insert_id = sqlx::query_as!(
            Tag,
            r#"INSERT INTO tags (name, color, created_by) VALUES (?, ?, ?)"#,
            payload.name,
            payload.color,
            owner.id
        )
        .execute(&self.db_connection)
        .await
        .map_err(|_| AppError::TagInternalError)?
        .last_insert_id();

        Ok(insert_id)
    }
}
