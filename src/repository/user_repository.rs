use sqlx::MySqlPool;

use crate::{errors::RepositoryError, model::user::User};

#[derive(Clone)]
pub struct UserRepository {
    pub db_connection: MySqlPool,
}

impl UserRepository {
    pub async fn find_user_by_email(&self, email: &String) -> Result<User, RepositoryError> {
        let query = sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.db_connection)
            .await;

        if let Ok(user) = query {
            Ok(user)
        } else {
            Err(RepositoryError::NotFound)
        }
    }
}
