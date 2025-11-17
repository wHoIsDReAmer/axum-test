use anyhow::Result;
use shaku::{Component, Interface};
use sqlx::PgPool;
use std::sync::Arc;

use crate::config::database_config::DatabaseConfig;

use async_trait::async_trait;

#[async_trait]
pub(crate) trait PostgresConnection: Interface {
    async fn get_pool(&self) -> Result<Arc<PgPool>>;
}

#[derive(Component)]
#[shaku(interface = PostgresConnection)]
pub(crate) struct PostgresConnectionImpl {
    config: Arc<DatabaseConfig>,
    pool: tokio::sync::OnceCell<Arc<PgPool>>,
}

#[async_trait]
impl PostgresConnection for PostgresConnectionImpl {
    async fn get_pool(&self) -> Result<Arc<PgPool>> {
        let pool = self
            .pool
            .get_or_init(|| async {
                let connection_string = format!(
                    "postgresql://{}:{}@{}:{}/{}",
                    self.config.user,
                    self.config.password,
                    self.config.host,
                    self.config.port,
                    self.config.database
                );

                match PgPool::connect(&connection_string).await {
                    Ok(pool) => Arc::new(pool),
                    Err(e) => panic!("Failed to connect to database: {}", e),
                }
            })
            .await;

        Ok(pool.clone())
    }
}

// pub(crate) async fn create_connection(config: &DatabaseConfig) -> Result<Arc<PgPool>> {
//     let connection_string = format!(
//         "postgresql://{}:{}@{}:{}/{}",
//         config.user,
//         config.password,
//         config.host,
//         config.port,
//         config.database
//     );
//     Ok(Arc::new(PgPool::connect(&connection_string).await?))
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[tokio::test]
//     async fn test_create_connection() {
//         let config = DatabaseConfig::new(
//             "localhost".to_string(),
//             5432,
//             "postgres".to_string(),
//             "1234".to_string(),
//             "postgres".to_string(),
//         );
//         let pool = create_connection(&config).await;
//         assert!(pool.is_ok());
//     }
// }
