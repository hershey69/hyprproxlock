use anyhow::{Context, Result};
use std::process::Command;
use tracing::debug;

pub struct BluetoothManager {}

impl BluetoothManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub async fn check_device_rssi(&self, mac: &str) -> Result<i16> {
        let output = Command::new("hcitool")
            .args(["rssi", mac])
            .output()
            .context("Failed to run hcitool rssi")?;

        if !output.status.success() {
            debug!("Failed to get RSSI for device {}", mac);
            return Ok(-255);
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let rssi = output_str
            .lines()
            .find(|line| line.contains("RSSI return value:"))
            .and_then(|line| line.split(':').next_back())
            .and_then(|value| value.trim().parse::<i16>().ok())
            .context("Failed to parse RSSI value")?;

        Ok(rssi)
    }
} 