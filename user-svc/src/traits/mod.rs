use crate::api::User;
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
/// NoSQL Document store interface
pub trait UserStore: Sync + Send {
    async fn insert_user(&self, user: User) -> Result<(), Box<dyn std::error::Error>>;
    async fn query_user_name(&self, name: String) -> Result<(), Box<dyn std::error::Error>>;
    async fn query_user_id(&self, id: String) -> Result<User, Box<dyn std::error::Error>>;
}
