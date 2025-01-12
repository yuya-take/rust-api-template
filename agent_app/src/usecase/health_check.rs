use std::sync::Arc;

use derive_new::new;

use agent_adapter::modules::RepositoriesModuleExt;
use agent_domain::repository::health_check::HealthCheckRepository;

#[derive(new)]
pub struct HealthCheckUseCase<R: RepositoriesModuleExt> {
    repository: Arc<R>,
}

impl<R: RepositoriesModuleExt> HealthCheckUseCase<R> {
    pub async fn diagnose_postgres_conn(&self) -> anyhow::Result<()> {
        let repository = self.repository.health_check_repository();
        repository.check_postgres_conn().await
    }
    pub async fn diagnose_dynamodb_conn(&self) -> anyhow::Result<()> {
        let repository = self.repository.health_check_repository();
        repository.check_dynamodb_conn().await
    }
}
