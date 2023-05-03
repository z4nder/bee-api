use sqlx::MySqlPool;

use crate::{dto::auth_dto::CreateUserData, errors::AppError, model::user::User};

#[derive(Clone)]
pub struct UserRepository<'a> {
    pub db_connection: &'a MySqlPool,
}

impl<'a> UserRepository<'a> {
    pub async fn create(&self, payload: CreateUserData) -> Result<u64, AppError> {
        let insert_id = sqlx::query_as!(
            User,
            r#"INSERT INTO users (name, email, password) VALUES (?, ?, ?)"#,
            payload.name,
            payload.email,
            payload.password
        )
        .execute(self.db_connection)
        .await
        .map_err(|_| AppError::NotFound)?
        .last_insert_id();

        Ok(insert_id)
    }

    pub async fn find_user_by_email(&self, email: &String) -> Result<User, AppError> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(self.db_connection)
            .await;

        if let Ok(user) = query {
            Ok(user)
        } else {
            Err(AppError::NotFound)
        }
    }

    pub async fn find_user_by_id(&self, id: &u64) -> Result<User, AppError> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(self.db_connection)
            .await;

        if let Ok(user) = query {
            Ok(user)
        } else {
            Err(AppError::NotFound)
        }
    }
}
