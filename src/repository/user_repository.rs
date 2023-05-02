use sqlx::MySqlPool;

use crate::{errors::AppError, model::user::User};

#[derive(Clone)]
pub struct UserRepository {
    pub db_connection: MySqlPool,
}

impl UserRepository {
    pub async fn find_user_by_email(&self, email: &String) -> Result<User, AppError> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.db_connection)
            .await;

        if let Ok(user) = query {
            Ok(user)
        } else {
            Err(AppError::NotFound)
        }
    }

    pub async fn find_user_by_id(&self, id: &u64) -> Result<User, AppError> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(&self.db_connection)
            .await;

        if let Ok(user) = query {
            Ok(user)
        } else {
            Err(AppError::NotFound)
        }
    }
}
