// Placeholder for measurement modules
// This will contain ping, DNS, and HTTP measurement implementations

pub mod dns;
pub mod http;
pub mod ping;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub measurement_type: MeasurementType,
    pub target: String,
    pub result: MeasurementResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementType {
    Ping,
    Dns,
    Http,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementResult {
    Ping {
        latency_ms: f64,
        packet_loss: f64,
    },
    Dns {
        resolution_time_ms: f64,
        success: bool,
    },
    Http {
        response_time_ms: f64,
        status_code: u16,
        success: bool,
    },
}
