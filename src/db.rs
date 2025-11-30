use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub database: String,
    pub version: String,
}

pub async fn get_database_info(pool: &PgPool) -> anyhow::Result<DatabaseInfo> {
    let mut conn = pool.acquire().await?;

    // Get database name
    let database: String = sqlx::query_scalar("SELECT current_database()")
        .fetch_one(&mut *conn)
        .await?;

    // Get database version
    let version: String = sqlx::query_scalar("SELECT version()")
        .fetch_one(&mut *conn)
        .await?;

    Ok(DatabaseInfo { database, version })
}
