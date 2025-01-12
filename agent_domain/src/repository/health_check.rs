use async_trait::async_trait;

#[async_trait]
pub trait HealthCheckRepository: Sync + Send {
    async fn check_postgres_conn(&self) -> anyhow::Result<()>;
    async fn check_dynamodb_conn(&self) -> anyhow::Result<()>;
}
