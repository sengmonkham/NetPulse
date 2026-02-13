use anyhow::Result;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<()> {
    println!("NetPulse - Recent Measurements Viewer");
    println!("======================================\n");

    // Connect to database
    let pool = SqlitePool::connect("sqlite:agent.db").await?;

    // Query recent measurements
    let rows = sqlx::query(
        r#"
        SELECT id, timestamp, measurement_type, target, result
        FROM measurements
        ORDER BY timestamp DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&pool)
    .await?;

    if rows.is_empty() {
        println!("No measurements found in database.");
        println!("Run the agent first to collect measurements.");
        return Ok(());
    }

    println!("Found {} measurements:\n", rows.len());

    for row in rows {
        let id: String = row.get("id");
        let timestamp: i64 = row.get("timestamp");
        let measurement_type: String = row.get("measurement_type");
        let target: String = row.get("target");
        let result_json: String = row.get("result");

        let datetime = chrono::DateTime::from_timestamp(timestamp, 0)
            .unwrap_or_else(|| chrono::Utc::now());

        println!("─────────────────────────────────────────────");
        println!("Type: {} | Target: {}", measurement_type, target);
        println!("Time: {}", datetime.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("Result: {}", result_json);
        println!();
    }

    Ok(())
}
