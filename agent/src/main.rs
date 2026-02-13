use chrono::Utc;
use clap::Parser;
use tokio::time::{interval, Duration};
use tracing::{error, info, warn};
use tracing_subscriber;
use uuid::Uuid;

mod config;
mod measurements;
mod p2p;
mod privacy;
mod storage;

use config::AgentConfig;
use measurements::*;
use privacy::PrivacyLayer;
use storage::Storage;

#[derive(Parser, Debug)]
#[command(name = "network-health-agent")]
#[command(about = "Decentralized network health monitoring agent", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "agent.toml")]
    config: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt().with_env_filter(log_level).init();

    info!("Starting Network Health Monitoring Agent");
    info!("Loading configuration from: {}", args.config);

    // Load configuration
    let config = AgentConfig::load(&args.config)?;
    info!("Configuration loaded successfully");
    info!(
        "Measurement interval: {} seconds",
        config.measurements.interval
    );
    info!("Targets: {:?}", config.measurements.targets);

    // Initialize storage
    let db_path = "sqlite://agent.db";
    info!("Database path: {}", db_path);
    let storage = Storage::new(db_path).await?;
    storage.init_schema().await?;
    info!("Storage initialized");

    // Initialize privacy layer
    let privacy = PrivacyLayer::new(config.privacy.epsilon, 1.0);
    info!(
        "Privacy layer initialized (epsilon: {})",
        config.privacy.epsilon
    );

    info!("Agent initialized successfully");
    info!("Starting measurement loop...");

    // Create measurement interval
    let mut ticker = interval(Duration::from_secs(config.measurements.interval));

    // Main event loop
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                info!("Running measurements...");

                for target in &config.measurements.targets {
                    // Determine measurement type based on target format
                    if target.starts_with("http://") || target.starts_with("https://") {
                        // HTTP measurement
                        let http_measure = http::HttpMeasurement::new(target.clone());
                        match http_measure.measure().await {
                            Ok((response_time, status_code, success)) => {
                                let privatized_time = privacy.privatize(response_time);

                                let measurement = Measurement {
                                    id: Uuid::new_v4().to_string(),
                                    timestamp: Utc::now(),
                                    measurement_type: MeasurementType::Http,
                                    target: target.clone(),
                                    result: MeasurementResult::Http {
                                        response_time_ms: privatized_time,
                                        status_code,
                                        success,
                                    },
                                };

                                if let Err(e) = storage.store_measurement(&measurement).await {
                                    error!("Failed to store HTTP measurement: {}", e);
                                } else {
                                    info!("HTTP {} - {}ms (status: {})", target, privatized_time.round(), status_code);
                                }
                            }
                            Err(e) => warn!("HTTP measurement failed for {}: {}", target, e),
                        }
                    } else if target.contains('.') && !target.contains('/') {
                        // Could be domain (DNS) or IP (ping)
                        // Try DNS first
                        let dns_measure = dns::DnsMeasurement::new(target.clone());
                        match dns_measure.measure().await {
                            Ok((resolution_time, success)) => {
                                let privatized_time = privacy.privatize(resolution_time);

                                let measurement = Measurement {
                                    id: Uuid::new_v4().to_string(),
                                    timestamp: Utc::now(),
                                    measurement_type: MeasurementType::Dns,
                                    target: target.clone(),
                                    result: MeasurementResult::Dns {
                                        resolution_time_ms: privatized_time,
                                        success,
                                    },
                                };

                                if let Err(e) = storage.store_measurement(&measurement).await {
                                    error!("Failed to store DNS measurement: {}", e);
                                } else {
                                    info!("DNS {} - {}ms (success: {})", target, privatized_time.round(), success);
                                }
                            }
                            Err(e) => warn!("DNS measurement failed for {}: {}", target, e),
                        }

                        // Also try ping
                        let ping_measure = ping::PingMeasurement::new(target.clone());
                        match ping_measure.measure().await {
                            Ok((latency, packet_loss)) => {
                                let privatized_latency = privacy.privatize(latency);

                                let measurement = Measurement {
                                    id: Uuid::new_v4().to_string(),
                                    timestamp: Utc::now(),
                                    measurement_type: MeasurementType::Ping,
                                    target: target.clone(),
                                    result: MeasurementResult::Ping {
                                        latency_ms: privatized_latency,
                                        packet_loss,
                                    },
                                };

                                if let Err(e) = storage.store_measurement(&measurement).await {
                                    error!("Failed to store ping measurement: {}", e);
                                } else {
                                    info!("PING {} - {}ms (loss: {}%)", target, privatized_latency.round(), packet_loss);
                                }
                            }
                            Err(e) => warn!("Ping measurement failed for {}: {}", target, e),
                        }
                    }
                }

                info!("Measurement cycle complete");
            }

            _ = tokio::signal::ctrl_c() => {
                info!("Shutting down...");
                break;
            }
        }
    }

    Ok(())
}
