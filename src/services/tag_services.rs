use crate::{
    dto::tag_dto::StoreTagPayload,
    errors::AppError,
    model::{tag::Tag, user::User},
    repository::tag_repository::TagRepository,
};

pub struct TagService;

impl TagService {
    pub async fn index(tag_repository: TagRepository, user: User) -> Result<Vec<Tag>, AppError> {
        let tags = tag_repository.index(user).await?;

        Ok(tags)
    }

    pub async fn store(
        payload: StoreTagPayload,
        tag_repository: TagRepository,
        user: User,
    ) -> Result<u64, AppError> {
        let created_tag_id = tag_repository.store(payload, user).await?;

        Ok(created_tag_id)
    }

    pub async fn find(id: u64, tag_repository: TagRepository, user: User) -> Result<Tag, AppError> {
        tag_repository.find(id, user).await
    }

    pub async fn update(
        payload: StoreTagPayload,
        id: u64,
        tag_repository: TagRepository,
        user: User,
    ) -> Result<u64, AppError> {
        todo!();
    }

    pub async fn delete(
        id: u64,
        tag_repository: TagRepository,
        user: User,
    ) -> Result<bool, AppError> {
        todo!();
    }
}
