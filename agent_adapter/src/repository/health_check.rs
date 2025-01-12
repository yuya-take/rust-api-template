use agent_domain::repository::health_check::HealthCheckRepository;
use async_trait::async_trait;
use std::sync::Arc;

use anyhow::anyhow;

use crate::persistence::dynamodb::DynamoDB;
use crate::persistence::postgres::PostgresConn;

pub struct HealthCheckRepositoryImpl {
    postgres_conn: Arc<PostgresConn>,
    dynamodb_conn: Arc<DynamoDB>,
}

impl HealthCheckRepositoryImpl {
    pub fn new(postgres_conn: PostgresConn, dynamodb_conn: DynamoDB) -> Self {
        Self {
            postgres_conn: Arc::new(postgres_conn),
            dynamodb_conn: Arc::new(dynamodb_conn),
        }
    }
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_postgres_conn(&self) -> anyhow::Result<()> {
        let pool = self.postgres_conn.0.clone();

        pool.try_acquire()
            .map(|_| ())
            .ok_or(anyhow!("Failed to acquire connection"))
    }
    async fn check_dynamodb_conn(&self) -> anyhow::Result<()> {
        let table_name = self.dynamodb_conn.list_tables().await?;
        println!("DynamoDB is healthy");
        println!("Table name: {:?}", table_name);
        Ok(())
    }
}
