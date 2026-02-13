// Placeholder for local storage using SQLite

use anyhow::Result;
use sqlx::SqlitePool;

pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init_schema(&self) -> Result<()> {
        // TODO: Create tables for measurements
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS measurements (
                id TEXT PRIMARY KEY,
                timestamp INTEGER NOT NULL,
                measurement_type TEXT NOT NULL,
                target TEXT NOT NULL,
                result TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn store_measurement(&self, _measurement: &crate::measurements::Measurement) -> Result<()> {
        // TODO: Insert measurement into database
        todo!("Store measurement")
    }
}
