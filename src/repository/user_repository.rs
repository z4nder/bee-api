use sqlx::MySqlPool;

use crate::{
    dto::auth_dto::CreateUserData,
    errors::{ApiError, AppError},
    model::user::User,
};

#[derive(Clone)]
pub struct UserRepository {
    pub db_connection: MySqlPool,
}

impl UserRepository {
    pub async fn create(&self, payload: CreateUserData) -> Result<u64, ApiError> {
        let insert_id = sqlx::query_as!(
            User,
            r#"INSERT INTO users (name, email, password) VALUES (?, ?, ?)"#,
            payload.name,
            payload.email,
            payload.password
        )
        .execute(&self.db_connection)
        .await
        .map_err(|_| ApiError::new(AppError::NotFound, None))?
        .last_insert_id();

        Ok(insert_id)
    }

    pub async fn find_user_by_email(&self, email: &String) -> Result<User, ApiError> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.db_connection)
            .await;

        if let Ok(user) = query {
            Ok(user)
        } else {
            Err(ApiError::new(AppError::NotFound, None))
        }
    }

    pub async fn find_user_by_id(&self, id: &u64) -> Result<User, ApiError> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&self.db_connection)
            .await;

        if let Ok(user) = query {
            Ok(user)
        } else {
            Err(ApiError::new(AppError::NotFound, None))
        }
    }
}
