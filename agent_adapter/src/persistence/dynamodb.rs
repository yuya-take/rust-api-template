use std::sync::Arc;

use aws_config::load_from_env;
use aws_sdk_dynamodb::config::Builder;
use aws_sdk_dynamodb::Client;
use dotenv::dotenv;

#[derive(Clone)]
pub struct DynamoDB {
    pub(crate) client: Arc<Client>,
}

// クライアントの初期化
pub async fn init_client() -> Client {
    let config = load_from_env().await;
    let dynamodb_config = {
        #[cfg(debug_assertions)]
        {
            dotenv().ok();
            let database_url = std::env::var("DYNAMO_LOCAL_ENDPOINT").expect(
                "DYNAMO_LOCAL_ENDPOINT is not set. Please set it via environment variable or .env file.",
            );
            Builder::from(&config).endpoint_url(database_url).build()
        }
        #[cfg(not(debug_assertions))]
        {
            Builder::from(&config).build()
        }
    };
    let dynamodb = Client::from_conf(dynamodb_config);

    #[cfg(debug_assertions)]
    println!("DynamoDB client initialized for local development");
    #[cfg(not(debug_assertions))]
    println!("DynamoDB client initialized for production");
    dynamodb
}

impl DynamoDB {
    // DynamoDBインスタンスを作成
    pub fn new(client: Client) -> DynamoDB {
        DynamoDB {
            client: Arc::new(client),
        }
    }

    pub async fn list_tables(&self) -> anyhow::Result<Option<Vec<String>>> {
        let res = self.client.list_tables().send().await?;
        Ok(res.table_names)
    }
}

#[cfg(test)]
mod test {
    use super::{init_client, DynamoDB};

    #[tokio::test]
    #[ignore]
    async fn test_list_table() {
        let client = init_client().await;
        let dynamodb = DynamoDB::new(client);
        let list_table = dynamodb.list_tables().await;
        println!("tables: {:?}", list_table);
    }
}
