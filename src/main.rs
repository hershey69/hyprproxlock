mod bluetooth;
mod config;
mod lock;

use tokio::time;
use tracing::{debug, error, info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tracing_subscriber::fmt::time::LocalTime;
use anyhow::{Result, Context};

use bluetooth::BluetoothManager;
use config::Config;
use lock::LockManager;

struct ProxLock {
    config: Config,
    bluetooth: BluetoothManager,
    lock: LockManager,
}

impl ProxLock {
    async fn new() -> Result<Self> {
        let config = Config::load()?;
        let bluetooth = BluetoothManager::new().await?;
        let lock = LockManager::new();

        Ok(Self {
            config,
            bluetooth,
            lock,
        })
    }

    async fn run(&mut self) -> Result<()> {
        loop {
            let mut any_device_in_range = false;
            let mut all_devices_weak = true;
            let mut any_device_connected = false;

            for device in &self.config.devices {
                if !device.enabled {
                    continue;
                }

                let rssi = self.bluetooth.check_device_rssi(&device.mac_address).await?;
                if rssi > -255 {
                    any_device_connected = true;
                    debug!("Device {} RSSI: {} dBm", device.name, rssi);
                    
                    if rssi >= self.config.thresholds.lock_threshold {
                        all_devices_weak = false;
                        debug!("Device {} signal strong enough to prevent locking", device.name);
                    }

                    if self.lock.is_locked() && rssi > self.config.thresholds.unlock_threshold {
                        any_device_in_range = true;
                        debug!("Device {} signal strong enough for unlocking", device.name);
                    }
                }
            }

            if !any_device_connected {
                all_devices_weak = false;
                debug!("No devices connected, not locking");
            }

            // Update timers and handle lock/unlock
            self.lock.update_timers(
                all_devices_weak,
                any_device_in_range,
                self.config.timings.poll_interval
            );

            if all_devices_weak && !self.lock.is_locked() && 
               self.lock.get_lock_timer() >= self.config.timings.lock_hold_seconds {
                self.lock.lock_screen()?;
            }

            if any_device_in_range && self.lock.is_locked() && 
               self.lock.get_unlock_timer() >= self.config.timings.unlock_hold_seconds {
                self.lock.unlock_screen()?;
            }

            time::sleep(time::Duration::from_secs(self.config.timings.poll_interval)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get the data directory for logs
    let data_dir = dirs::state_dir()
        .context("Failed to get state directory")?
        .join("hyprproxlock")
        .join("logs");
    
    // Create the log directory if it doesn't exist
    std::fs::create_dir_all(&data_dir)
        .context("Failed to create log directory")?;
    
    // Get current date for log file
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let current_log_file = data_dir.join(format!("hyprproxlock.log.{}", current_date));
    
    // Create a file appender with daily rotation
    let file_appender = tracing_appender_localtime::rolling::RollingFileAppender::new(
        tracing_appender_localtime::rolling::Rotation::DAILY,
        &data_dir,
        "hyprproxlock.log"
    );
    
    // Create a file layer
    let file_layer = fmt::layer()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTime::rfc_3339());
    
    // Create a console layer
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTime::rfc_3339());
    
    // Initialize the subscriber with both layers
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env()
            .add_directive(Level::DEBUG.into()))
        .with(file_layer)
        .with(console_layer)
        .init();
    
    info!("Starting hyprproxlock");
    info!("Log file: {}", current_log_file.display());
    
    match ProxLock::new().await {
        Ok(mut proxlock) => proxlock.run().await,
        Err(e) => {
            error!("{}", e);
            eprintln!("\nProgram closed due to an error. Please check the logs at:\n{}\n", current_log_file.display());
            std::process::exit(1);
        }
    }
}
