use anyhow::Result;
use sqlx::{SqlitePool, Row};

pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init_schema(&self) -> Result<()> {
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

    pub async fn store_measurement(&self, measurement: &crate::measurements::Measurement) -> Result<()> {
        let result_json = serde_json::to_string(&measurement.result)?;
        let measurement_type = format!("{:?}", measurement.measurement_type);
        
        sqlx::query(
            r#"
            INSERT INTO measurements (id, timestamp, measurement_type, target, result)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(&measurement.id)
        .bind(measurement.timestamp.timestamp())
        .bind(measurement_type)
        .bind(&measurement.target)
        .bind(result_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_measurements(&self, limit: i32) -> Result<Vec<crate::measurements::Measurement>> {
        let rows = sqlx::query(
            r#"
            SELECT id, timestamp, measurement_type, target, result
            FROM measurements
            ORDER BY timestamp DESC
            LIMIT ?
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let mut measurements = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let timestamp: i64 = row.get("timestamp");
            let measurement_type_str: String = row.get("measurement_type");
            let target: String = row.get("target");
            let result_json: String = row.get("result");

            let measurement_type = match measurement_type_str.as_str() {
                "Ping" => crate::measurements::MeasurementType::Ping,
                "Dns" => crate::measurements::MeasurementType::Dns,
                "Http" => crate::measurements::MeasurementType::Http,
                _ => continue,
            };

            let result: crate::measurements::MeasurementResult = serde_json::from_str(&result_json)?;

            measurements.push(crate::measurements::Measurement {
                id,
                timestamp: chrono::DateTime::from_timestamp(timestamp, 0)
                    .unwrap_or_else(|| chrono::Utc::now()),
                measurement_type,
                target,
                result,
            });
        }

        Ok(measurements)
    }
}
