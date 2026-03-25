use anyhow::Context;
use axum_test::TestServer;
use serde_json::json;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

use crate::app::App;
use crate::db::{DatabaseInfo, get_database_info};

struct TestContext {
    _container: ContainerAsync<GenericImage>,
    pool: sqlx::PgPool,
    server: TestServer,
}

impl TestContext {
    async fn new() -> anyhow::Result<Self> {
        let container = GenericImage::new("postgres", "18-alpine")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "testuser")
            .with_env_var("POSTGRES_PASSWORD", "testpass")
            .with_env_var("POSTGRES_DB", "testdb")
            .start()
            .await
            .context("Failed to start container")?;

        let host = container.get_host().await.context("Failed to get host")?;
        let port = container
            .get_host_port_ipv4(5432)
            .await
            .context("Failed to get port")?;

        let database_url = format!("postgres://testuser:testpass@{host}:{port}/testdb");

        let config = crate::config::Config { database_url };
        let app = App::new(&config).await.context("Failed to create app")?;
        let pool = app.db.clone();
        let server = TestServer::new(app.into_router());

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
    response.assert_text("Healthy\n");

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

#[tokio::test]
async fn test_database_error_handling() -> anyhow::Result<()> {
    let ctx = TestContext::new().await?;
    ctx.pool.close().await;

    let response = ctx.server.get("/test").await;
    response.assert_status_internal_server_error();
    response.assert_json(&json!({"error": "Internal server error"}));

    Ok(())
}

#[test]
fn test_database_info_serde() -> anyhow::Result<()> {
    let info = DatabaseInfo {
        database: "test_db".to_string(),
        version: "PostgreSQL 13.0".to_string(),
    };

    let json = serde_json::to_string(&info)?;
    let deserialized: DatabaseInfo = serde_json::from_str(&json)?;
    assert_eq!(deserialized.database, info.database);
    assert_eq!(deserialized.version, info.version);

    Ok(())
}
