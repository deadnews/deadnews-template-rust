use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, FromRow)]
pub struct DatabaseInfo {
    pub database: String,
    pub version: String,
}

pub async fn get_database_info(pool: &PgPool) -> anyhow::Result<DatabaseInfo> {
    let info = sqlx::query_as("SELECT current_database() AS database, version() AS version")
        .fetch_one(pool)
        .await?;

    Ok(info)
}
