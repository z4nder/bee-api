use crate::{
    dto::tag_dto::{StoreTagPayload, UpdateTagPayload},
    errors::{ApiError, AppError},
    model::{tag::Tag, user::User},
    repository::tag_repository::TagRepository,
};

pub struct TagService;

impl TagService {
    pub async fn index(tag_repository: TagRepository, user: User) -> Result<Vec<Tag>, ApiError> {
        let tags = tag_repository.index(user).await?;

        Ok(tags)
    }

    pub async fn store(
        payload: StoreTagPayload,
        tag_repository: TagRepository,
        user: User,
    ) -> Result<u64, ApiError> {
        let created_tag_id = tag_repository.store(payload, user).await?;

        Ok(created_tag_id)
    }

    pub async fn find(id: u64, tag_repository: TagRepository, user: User) -> Result<Tag, ApiError> {
        tag_repository.find(id, user).await
    }

    pub async fn update(
        payload: UpdateTagPayload,
        id: u64,
        tag_repository: TagRepository,
        user: User,
    ) -> Result<u64, ApiError> {
        tag_repository.update(id, payload, user).await
    }

    pub async fn destroy(
        id: u64,
        tag_repository: TagRepository,
        user: User,
    ) -> Result<u64, ApiError> {
        tag_repository.destroy(id, user).await
    }
}
