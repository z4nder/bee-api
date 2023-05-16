use bcrypt::DEFAULT_COST;

use crate::errors::{ApiError, AppError};

pub async fn hash_password(password: String) -> Result<String, ApiError> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(password, DEFAULT_COST)
            .map_err(|_| ApiError::new(AppError::NotFound, None));
        let _ = send.send(result);
    });

    recv.await
        .map_err(|_| ApiError::new(AppError::EncryptError, None))?
}

pub async fn verify_password(password: String, hash: String) -> Result<bool, ApiError> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result =
            bcrypt::verify(password, &hash).map_err(|_| ApiError::new(AppError::NotFound, None));
        let _ = send.send(result);
    });

    recv.await
        .map_err(|_| ApiError::new(AppError::EncryptError, None))?
}
