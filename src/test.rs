use axum::{Router, routing::get};
use axum_test::TestServer;
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

use crate::db::{DatabaseInfo, get_database_info};
use crate::routing::{AppState, create_router, database_test};

struct TestContext {
    _container: ContainerAsync<GenericImage>,
    pool: PgPool,
    server: TestServer,
}

impl TestContext {
    async fn new() -> anyhow::Result<Self> {
        // Start PostgreSQL container
        let container = GenericImage::new("postgres", "18-alpine")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "testuser")
            .with_env_var("POSTGRES_PASSWORD", "testpass")
            .with_env_var("POSTGRES_DB", "testdb")
            .with_mapped_port(0, 5432.tcp())
            .start()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to start container: {e}"))?;

        // Get host and port dynamically
        let host = container
            .get_host()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get host: {e}"))?;
        let port = container
            .get_host_port_ipv4(5432)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to get port: {e}"))?;

        // Build connection string
        let connection_string = format!("postgres://testuser:testpass@{host}:{port}/testdb");

        // Create connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database: {e}"))?;

        let app = create_router(AppState { db: pool.clone() });
        let server = TestServer::new(app)?;

        Ok(Self {
            _container: container,
            pool,
            server,
        })
    }
}

#[tokio::test]
async fn test_health_check() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let response = ctx.server.get("/health").await;
    response.assert_status_ok();
    response.assert_text("Healthy");

    Ok(())
}

#[tokio::test]
async fn test_database_endpoint() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let response = ctx.server.get("/test").await;
    response.assert_status_ok();

    let db_info: DatabaseInfo = response.json();
    assert_eq!(db_info.database, "testdb");
    assert!(db_info.version.contains("PostgreSQL"));

    Ok(())
}

#[tokio::test]
async fn test_get_database_info() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    let db_info = get_database_info(&ctx.pool).await?;
    assert_eq!(db_info.database, "testdb");
    assert!(db_info.version.contains("PostgreSQL"));

    Ok(())
}

// Test database error handling
#[tokio::test]
async fn test_database_error_handling() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;

    // Close all connections in the pool to simulate database failure
    ctx.pool.close().await;

    let app = Router::new()
        .route("/test", get(database_test))
        .with_state(AppState { db: ctx.pool });
    let server = TestServer::new(app)?;

    // Should return 500 error when database is unavailable
    let response = server.get("/test").await;
    response.assert_status_internal_server_error();
    response.assert_json(&json!({"error": "Internal server error"}));

    Ok(())
}

// Test struct serialization/deserialization
#[test]
fn test_database_info_serde() -> anyhow::Result<()> {
    let info = DatabaseInfo {
        database: "test_db".to_string(),
        version: "PostgreSQL 13.0".to_string(),
    };

    // Test serialization
    let json = serde_json::to_string(&info)?;

    // Test deserialization
    let deserialized: DatabaseInfo = serde_json::from_str(&json)?;
    assert_eq!(deserialized.database, info.database);
    assert_eq!(deserialized.version, info.version);

    Ok(())
}

// Test AppState clone
#[tokio::test]
async fn test_app_state_clone() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;
    let state = AppState {
        db: ctx.pool.clone(),
    };

    // Test that AppState can be cloned
    let cloned = state.clone();

    // Both should be able to get database info
    let info1 = get_database_info(&state.db).await?;
    let info2 = get_database_info(&cloned.db).await?;
    assert_eq!(info1.database, info2.database);

    Ok(())
}
