use anyhow::{Context, Result};
use surge_ping::{Client, Config, PingIdentifier, PingSequence};
use std::time::Duration;
use tokio::time::timeout;

pub struct PingMeasurement {
    target: String,
}

impl PingMeasurement {
    pub fn new(target: String) -> Self {
        Self { target }
    }

    pub async fn measure(&self) -> Result<(f64, f64)> {
        // Send 4 ping packets and calculate statistics
        let count = 4;
        let timeout_duration = Duration::from_secs(2);
        
        let client_v4 = Client::new(&Config::default())
            .context("Failed to create ping client")?;
        
        let mut latencies = Vec::new();
        let mut successful = 0;

        for i in 0..count {
            let payload = [0; 56];
            
            match timeout(
                timeout_duration,
                client_v4.pinger(
                    self.target.parse().context("Invalid IP address")?,
                    PingIdentifier(rand::random()),
                )
                .await
                .ping(PingSequence(i), &payload)
            ).await {
                Ok(Ok((_, duration))) => {
                    latencies.push(duration.as_secs_f64() * 1000.0);
                    successful += 1;
                }
                _ => {
                    // Timeout or error - count as packet loss
                }
            }
        }

        let avg_latency = if !latencies.is_empty() {
            latencies.iter().sum::<f64>() / latencies.len() as f64
        } else {
            0.0
        };

        let packet_loss = ((count - successful) as f64 / count as f64) * 100.0;

        Ok((avg_latency, packet_loss))
    }
}
