use std::sync::{PoisonError, RwLock};

use chrono::Utc;
use uuid::Uuid;
use thiserror::Error;
use async_trait::async_trait;

use crate::user::User;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("PoisonError: `{0}`")]
    LockError(String),
    #[error("This entity is already exist")]
    AlreadyExist,
    #[error("This entity does not exist")]
    DoesNotExist,
    #[error("The id format is not valid")]
    InvalidId,
}

impl<T> From<PoisonError<T>> for RepositoryError {
    fn from(poison_error: PoisonError<T>) -> Self{
        RepositoryError::LockError(poison_error.to_string())
    }
}

#[async_trait]
pub trait Repository: Send + Sync + 'static {
    async fn get_user(&self, user_id: &Uuid) ->  Result<User, RepositoryError>;
    async fn create_user(&self, user: &User) ->  Result<User, RepositoryError>;
    async fn update_user(&self, user: &User) ->  Result<User, RepositoryError>;
    async fn delete_user(&self, user_id: &Uuid) ->  Result<Uuid, RepositoryError>;
}

pub struct MemoryRepository {
    users: RwLock<Vec<User>>,
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self {
            users: RwLock::new(vec![]),
        }
    }
}

#[async_trait]
impl Repository for MemoryRepository {
    async fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, RepositoryError> {
        let users = self.users.read()?;
        users
            .iter()
            .find(|u| &u.id == user_id)
            .cloned()
            .ok_or_else(|| RepositoryError::InvalidId)
    }

    async fn create_user(&self, user: &User) ->  Result<User, RepositoryError> {
        if self.get_user(&user.id).await.is_ok() {
            return Err(RepositoryError::AlreadyExist);
        }

        let mut new_user = user.to_owned();
        new_user.created_at = Some(Utc::now());
        let mut users = self.users.write()?;
        users.push(new_user.clone());
        Ok(new_user)
    }

    async fn update_user(&self, user: &User) ->  Result<User, RepositoryError> {
        if !self.get_user(&user.id).await.is_ok() {
            return Err(RepositoryError::DoesNotExist);
        } 

        let mut updated_user = user.to_owned();
        updated_user.updated_at = Some(Utc::now());
        let mut users = self.users.write()?;
        users.retain(|x| x.id != user.id);
        users.push(updated_user.clone());
        Ok(updated_user)
    }

    async fn delete_user(&self, user_id: &Uuid) ->  Result<Uuid, RepositoryError> {
        let mut users = self.users.write()?;
        users.retain(|x| &x.id != user_id);
        Ok(user_id.to_owned())
    }
}