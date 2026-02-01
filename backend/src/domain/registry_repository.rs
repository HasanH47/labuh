use crate::domain::models::RegistryCredential;
use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait RegistryRepository: Send + Sync {
    async fn list_by_team(&self, team_id: &str) -> Result<Vec<RegistryCredential>>;
    async fn find_by_url(&self, team_id: &str, url: &str) -> Result<Option<RegistryCredential>>;
    async fn save(&self, cred: RegistryCredential) -> Result<RegistryCredential>;
    async fn delete(&self, id: &str, team_id: &str) -> Result<()>;
}
