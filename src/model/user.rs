use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct UserProfile {
    pub id: u64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn test() -> User {
        User {
            id: 1,
            name: String::from("John Doe"),
            email: String::from("john.doe@example.com"),
            password: String::from("password123"),
            email_verified_at: None,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        }
    }
}
