// use async_trait::async_trait;
// use sqlx::MySqlPool;

// use crate::model::user::User;

// #[derive(Clone)]
// pub struct TodoRepository {
//     pub db_connection: MySqlPool,
// }

// #[async_trait]
// pub trait Repository<T> {
//     async fn find(&self, id: u64) -> T;
// }

// #[async_trait]
// impl Repository<User> for TodoRepository {
//     async fn find(&self, id: u64) -> User {
//         sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
//             .fetch_one(&self.db_connection)
//             .await
//             .unwrap()
//     }
// }
