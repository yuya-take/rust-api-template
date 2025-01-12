use std::sync::Arc;

use agent_adapter::{
    modules::{RepositoriesModule, RepositoriesModuleExt},
    persistence::{
        dynamodb::{init_client, DynamoDB},
        postgres::PostgresConn,
    },
};
use agent_app::usecase::health_check::HealthCheckUseCase;

pub struct Modules {
    health_check_use_case: HealthCheckUseCase<RepositoriesModule>,
}

pub trait ModulesExt {
    type RepositoriesModule: RepositoriesModuleExt;

    fn health_check_use_case(&self) -> &HealthCheckUseCase<Self::RepositoriesModule>;
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn health_check_use_case(&self) -> &HealthCheckUseCase<Self::RepositoriesModule> {
        &self.health_check_use_case
    }
}

impl Modules {
    pub async fn new() -> Modules {
        let postgres_conn = PostgresConn::new().await;
        let client = init_client().await;
        let dynamodb_conn = DynamoDB::new(client);

        let repositories_module = Arc::new(RepositoriesModule::new(postgres_conn, dynamodb_conn));
        let health_check_use_case = HealthCheckUseCase::new(repositories_module.clone());
        Self {
            health_check_use_case,
        }
    }
}
