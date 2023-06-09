use chrono::Utc;

use crate::{
    dto::auth_dto::{CreateUserData, LoginInput, RegisterInput},
    errors::{ApiError, AppError},
    model::user::User,
    repository::user_repository::UserRepository,
    utils::encryption,
};

pub struct AuthService;

impl<'a> AuthService {
    pub async fn login(
        input: LoginInput,
        user_repository: UserRepository,
    ) -> Result<User, ApiError> {
        let user = user_repository.find_user_by_email(&input.email).await?;

        if encryption::verify_password(input.password, user.password.to_owned()).await? {
            Ok(user)
        } else {
            Err(ApiError::new(AppError::WrongCredentials, None))
        }
    }

    pub async fn register(
        input: RegisterInput,
        user_repository: UserRepository,
    ) -> Result<u64, ApiError> {
        if user_repository
            .find_user_by_email(&input.email)
            .await
            .is_ok()
        {
            return Err(ApiError::new(AppError::DuplicateUserEmail, None));
        }

        let data = CreateUserData {
            name: input.name,
            email: input.email,
            password: encryption::hash_password(input.password).await?,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        Ok(user_repository.create(data).await?)
    }
}
