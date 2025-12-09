use std::{
    net::{Ipv4Addr, Ipv6Addr},
    time::Duration,
};

use anyhow::Result;
use log::error;
use serde::Deserialize;
use tokio::time::sleep;

/// URL for the IPify service to get external IP address
const IPIFY_URL: &str = "https://api64.ipify.org?format=json";

/// Maximum number of retry attempts for fetching external IP
const MAX_RETRY_ATTEMPTS: u32 = 5;

/// Delay between retry attempts in seconds
const RETRY_DELAY_SECONDS: u64 = 5;

/// Response structure from IPify API
#[derive(Deserialize)]
struct IpifyResponse {
    ip: String,
}

pub struct ExternalIp {
    pub address: String,
}

/// Makes a single request to the IPify API to get the external IP address
async fn fetch_ip_from_api() -> Result<String> {
    let response = reqwest::get(IPIFY_URL).await?.json::<IpifyResponse>().await?;

    Ok(response.ip)
}

/// Gets the external IP address with automatic retry on failure
///
/// This function will retry up to `MAX_RETRY_ATTEMPTS` times with a delay
/// of `RETRY_DELAY_SECONDS` seconds between attempts.
///
/// # Returns
///
/// Returns the external IP address as a string on success, or an error
/// if all retry attempts fail.
async fn get_external_ip() -> Result<String> {
    let mut last_error = None;

    for attempt in 1..=MAX_RETRY_ATTEMPTS {
        match fetch_ip_from_api().await {
            Ok(ip) => return Ok(ip),
            Err(err) => {
                error!(
                    "Failed to fetch external IP (attempt {}/{}): {}",
                    attempt, MAX_RETRY_ATTEMPTS, err
                );

                last_error = Some(err);

                // Don't sleep after the last attempt
                if attempt < MAX_RETRY_ATTEMPTS {
                    sleep(Duration::from_secs(RETRY_DELAY_SECONDS)).await;
                }
            }
        }
    }

    // Return the last error encountered
    Err(last_error.unwrap())
}

impl ExternalIp {
    pub async fn new() -> Result<Self> {
        let address = get_external_ip().await?;

        Ok(Self { address })
    }

    pub fn is_ipv4(&self) -> bool {
        self.address.parse::<Ipv4Addr>().is_ok()
    }

    pub fn is_ipv6(&self) -> bool {
        self.address.parse::<Ipv6Addr>().is_ok()
    }
}
