use sqlx::PgPool;
use anyhow::Result;

use crate::config::database_config::DatabaseConfig;

pub(crate) async fn create_connection(config: &DatabaseConfig) -> Result<PgPool> {
    let connection_string = format!(
        "postgresql://{}:{}@{}:{}/{}",
        config.user,
        config.password,
        config.host,
        config.port,
        config.database
    );

    Ok(PgPool::connect(&connection_string).await?)
}

#[cfg(test)]
mod tests {
    use super::*;   
    #[tokio::test]
    async fn test_create_connection() {
        let config = DatabaseConfig::new(
            "localhost".to_string(),
            5432,
            "postgres".to_string(),
            "1234".to_string(),
            "postgres".to_string(),
        );
        let pool = create_connection(&config).await;
        assert!(pool.is_ok());
    }
}