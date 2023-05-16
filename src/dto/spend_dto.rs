use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct StoreSpendPayload {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub date: String,
    #[validate(range(min = 0.1))]
    pub value: f64,
    pub tags: Vec<u64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSpendPayload {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub date: String,
    #[validate(range(min = 0.1))]
    pub value: f64,

    pub tags: Vec<u64>,
}
