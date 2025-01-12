use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct PostgresConn(pub(crate) Arc<Pool<Postgres>>);

impl PostgresConn {
    pub async fn new() -> PostgresConn {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect(
            "DATABASE_URL is not set. Please set it via environment variable or .env file.",
        );
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap_or_else(|_| {
                panic!("Cannot connect to the database. Please check your configuration.")
            });
        PostgresConn(Arc::new(pool))
    }
}
