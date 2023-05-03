use bcrypt::DEFAULT_COST;

use crate::errors::AppError;

pub async fn hash_password(password: String) -> Result<String, AppError> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::hash(password, DEFAULT_COST).map_err(|err| AppError::NotFound);
        let _ = send.send(result);
    });

    recv.await.map_err(|_| AppError::EncryptError)?
}

pub async fn verify_password(password: String, hash: String) -> Result<bool, AppError> {
    let (send, recv) = tokio::sync::oneshot::channel();
    rayon::spawn(move || {
        let result = bcrypt::verify(password, &hash).map_err(|err| AppError::NotFound);
        let _ = send.send(result);
    });

    recv.await.map_err(|_| AppError::EncryptError)?
}
