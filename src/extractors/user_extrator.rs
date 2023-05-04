use axum::{
    async_trait,
    extract::{Extension, FromRequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use sqlx::MySqlPool;

use crate::{model::user::User, repository::user_repository::UserRepository, utils::jwt};

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|err| err.into_response())?;

        let Extension(pool) = parts
            .extract::<Extension<MySqlPool>>()
            .await
            .map_err(|err| err.into_response())?;

        let user_repository = UserRepository {
            db_connection: pool,
        };

        let claims = jwt::verify(bearer.token());

        Ok(user_repository
            .find_user_by_id(&claims.sub)
            .await
            .map_err(|err| err.into_response())?)
    }
}
