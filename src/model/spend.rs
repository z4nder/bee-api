use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Spend {
    pub id: u64,
    pub name: String,
    pub date: DateTime<Utc>,
    pub value: f64,
    pub created_by: u64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
