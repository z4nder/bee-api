use axum::{
    async_trait,
    extract::{Extension, FromRequest, FromRequestParts, State, TypedHeader},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt,
};

use crate::{
    errors::AppError,
    model::user::User,
    repository::user_repository::{self, UserRepository},
    utils::jwt,
};

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

        let Extension(user_repository) = parts
            .extract::<Extension<UserRepository>>()
            .await
            .map_err(|err| err.into_response())?;

        let claims = jwt::verify(bearer.token());

        Ok(user_repository.find_user_by_id(&claims.sub).await.unwrap())
    }
}
