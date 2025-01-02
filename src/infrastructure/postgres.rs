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