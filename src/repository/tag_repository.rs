use sqlx::MySqlPool;

use crate::dto::tag_dto::{StoreTagPayload, UpdateTagPayload};
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

    pub async fn find(&self, id: u64, owner: User) -> Result<Tag, AppError> {
        let query = sqlx::query_as!(
            Tag,
            r#"SELECT * FROM tags WHERE created_by = ? AND id = ?"#,
            owner.id,
            id
        )
        .fetch_one(&self.db_connection)
        .await;

        if let Ok(tag) = query {
            Ok(tag)
        } else {
            Err(AppError::ResourceNotFound)
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

    pub async fn update(
        &self,
        id: u64,
        payload: UpdateTagPayload,
        owner: User,
    ) -> Result<u64, AppError> {
        let update_id = sqlx::query_as!(
            Tag,
            r#"UPDATE tags SET name = ?, color = ? WHERE id = ? AND created_by = ?"#,
            payload.name,
            payload.color,
            id,
            owner.id
        )
        .execute(&self.db_connection)
        .await
        .map_err(|_| AppError::TagInternalError)?
        .last_insert_id();

        Ok(update_id)
    }

    pub async fn destroy(&self, id: u64, owner: User) -> Result<u64, AppError> {
        let delete_id = sqlx::query_as!(
            Tag,
            r#"DELETE FROM tags WHERE id = ? AND created_by = ?"#,
            id,
            owner.id
        )
        .execute(&self.db_connection)
        .await
        .map_err(|_| AppError::TagInternalError)?
        .last_insert_id();

        Ok(delete_id)
    }
}
