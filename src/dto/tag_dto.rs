use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct StoreTagPayload {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub color: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTagPayload {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub color: String,
}
