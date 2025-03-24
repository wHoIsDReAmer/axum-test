use std::sync::Arc;

use shaku::HasComponent;
use sqlx::{Pool, Postgres};

use crate::{AuthModule, AuthRepositoryImpl, AuthRepositoryImplParameters, AuthService, DatabaseConfig, DatabaseModule, PostgresConnection, PostgresConnectionImpl, PostgresConnectionImplParameters};

pub async fn setup_pool(config: DatabaseConfig) -> Arc<Pool<Postgres>> {
    let db_module = DatabaseModule::builder()
        .with_component_parameters::<PostgresConnectionImpl>(PostgresConnectionImplParameters {
            config: Arc::new(config),
            pool: tokio::sync::OnceCell::new(),
        })
        .build();
    let conn: &dyn PostgresConnection = db_module.resolve_ref();
    let pool = conn.get_pool().await.expect("Failed to get database pool");

    pool
}

pub async fn setup_auth_service(pool: Arc<Pool<Postgres>>) -> Arc<dyn AuthService> {
    let auth_module = AuthModule::builder()
        .with_component_parameters::<AuthRepositoryImpl>(AuthRepositoryImplParameters {
            db: pool,
        })
        .build();

    auth_module.resolve()
}