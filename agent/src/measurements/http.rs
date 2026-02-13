use anyhow::{Context, Result};
use reqwest::Client;
use std::time::{Duration, Instant};

pub struct HttpMeasurement {
    url: String,
}

impl HttpMeasurement {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn measure(&self) -> Result<(f64, u16, bool)> {
        // Create HTTP client with timeout
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .context("Failed to create HTTP client")?;

        let start = Instant::now();
        
        // Send GET request
        let result = client.get(&self.url).send().await;
        
        let duration = start.elapsed();
        let response_time_ms = duration.as_secs_f64() * 1000.0;

        match result {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let success = response.status().is_success();
                Ok((response_time_ms, status_code, success))
            }
            Err(_) => {
                // Request failed - return 0 status code
                Ok((response_time_ms, 0, false))
            }
        }
    }
}
