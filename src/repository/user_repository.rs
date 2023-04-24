use sqlx::MySqlPool;

use crate::model::user::User;

#[derive(Clone)]
pub struct UserRepository {
    pub db_connection: MySqlPool,
}

impl UserRepository {
    pub async fn find_user_by_email(&self, email: &String) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
            .fetch_one(&self.db_connection)
            .await
    }
}
