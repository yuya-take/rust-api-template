use crate::persistence::dynamodb::DynamoDB;
use agent_domain::repository::health_check::HealthCheckRepository;

use crate::{
    persistence::postgres::PostgresConn, repository::health_check::HealthCheckRepositoryImpl,
};

pub struct RepositoriesModule {
    heakth_check_repository: HealthCheckRepositoryImpl,
}

pub trait RepositoriesModuleExt {
    type HealthCheckRepo: HealthCheckRepository;
    fn health_check_repository(&self) -> &Self::HealthCheckRepo;
}

impl RepositoriesModuleExt for RepositoriesModule {
    type HealthCheckRepo = HealthCheckRepositoryImpl;
    fn health_check_repository(&self) -> &Self::HealthCheckRepo {
        &self.heakth_check_repository
    }
}

impl RepositoriesModule {
    pub fn new(postgres: PostgresConn, dynamodb: DynamoDB) -> Self {
        Self {
            heakth_check_repository: HealthCheckRepositoryImpl::new(
                postgres.clone(),
                dynamodb.clone(),
            ),
        }
    }
}
