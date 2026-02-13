use anyhow::Result;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::config::*;
use std::time::Instant;

pub struct DnsMeasurement {
    domain: String,
}

impl DnsMeasurement {
    pub fn new(domain: String) -> Self {
        Self { domain }
    }

    pub async fn measure(&self) -> Result<(f64, bool)> {
        // Create resolver with default configuration
        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        );

        let start = Instant::now();
        
        // Attempt DNS lookup
        let result = resolver.lookup_ip(&self.domain).await;
        
        let duration = start.elapsed();
        let resolution_time_ms = duration.as_secs_f64() * 1000.0;

        match result {
            Ok(response) => {
                // Successfully resolved - check if we got at least one IP
                let success = response.iter().count() > 0;
                Ok((resolution_time_ms, success))
            }
            Err(_) => {
                // DNS resolution failed
                Ok((resolution_time_ms, false))
            }
        }
    }
}
