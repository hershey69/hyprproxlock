# 🔒 hyprproxlock

A proximity-based daemon for [Hyprland](https://hyprland.org/) that triggers screen locking and unlocking through [hyprlock](https://github.com/hyprwm/hyprlock) based on Bluetooth device proximity. It monitors connected devices' signal strength to automatically control your screen lock state.

## ✨ Features

- 🔵 **Bluetooth Proximity Detection**: Monitors Bluetooth device signal strength to determine when to lock/unlock
- ⚡ **hyprlock Integration**: Uses hyprlock for actual screen locking/unlocking
- 🔄 **Configurable Thresholds**: Customize signal strength thresholds for locking/unlocking
- ⏱️ **Adjustable Timings**: Fine-tune lock/unlock hold times and polling intervals
- 🚀 **Optimized Performance**: Lightweight Rust implementation with minimal dependencies, efficient resource usage, and battery-friendly Bluetooth polling

## 📋 Dependencies

- [Hyprland](https://hyprland.org/)
- [hyprlock](https://github.com/hyprwm/hyprlock) - Required for screen locking functionality
- `bluez-deprecated-tools` package (for `hcitool`)

## 🚀 Get Started

### 1. Installation
You can install hyprproxlock from the AUR:

```bash
yay -S hyprproxlock
```

This will automatically install hyprlock and bluez-deprecated-tools as a dependency.

### 2. Configuration

Create a configuration file at `~/.config/hypr/hyprproxlock.conf`:

```conf
# Device Configuration
device {
    mac_address = "XX:XX:XX:XX:XX:XX"
    name = "My Device"
    enabled = true
}

device {
    mac_address = "XX:XX:XX:XX:XX:XX"
    name = "My Watch"
    enabled = true
}

# Threshold Settings
thresholds {
    lock_threshold = -25
    unlock_threshold = -15
}

# Timing Configuration
timings {
    lock_hold_seconds = 3
    unlock_hold_seconds = 3
    poll_interval = 1
}
```

### 3. Usage

1. Start the daemon:

```bash
hyprproxlock
```

2. The screen will automatically:
   - 🔒 Lock (using hyprlock) when all configured devices are out of range
   - 🔓 Unlock (using hyprlock) when a configured device comes back in range

### 4. Autostart with Hyprland

To automatically start hyprproxlock when Hyprland starts, add it to your Hyprland configuration:

1. Edit `~/.config/hypr/hyprland.conf`:
```ini
exec-once = hyprproxlock
```

## 🤔 How It Works

Let's break down how hyprproxlock works in simple terms:

### 🔵 Bluetooth Connection
- The tool only works when your Bluetooth is turned ON
- Your device (phone, watch, etc.) must be paired and connected
- If Bluetooth is off or no devices are connected, the tool won't do anything

### 📶 Signal Strength (RSSI)
- The tool measures how strong your device's Bluetooth signal is
- Stronger signal = closer to your computer
- Weaker signal = further away from your computer
- Signal strength is measured in dBm (decibel-milliwatts)
  - -15 dBm = very strong signal (very close)
  - -70 dBm = very weak signal (far away)

### ⏱️ Locking Behavior
1. When your device's signal gets weaker than the `lock_threshold`:
   - The tool starts a timer (set by `lock_hold_seconds`)
   - It keeps checking the signal during this time
   - Only triggers hyprlock to lock the screen if the signal stays weak for the full timer duration
   - This prevents accidental locks if you just briefly walk away

2. The screen won't lock if:
   - Bluetooth is turned off
   - Your device is not connected
   - The signal is stronger than `lock_threshold`
   - The timer hasn't completed yet

### 🔓 Unlocking Behavior
1. When your device's signal gets stronger than the `unlock_threshold`:
   - The tool starts a timer (set by `unlock_hold_seconds`)
   - It keeps checking the signal during this time
   - Only triggers hyprlock to unlock the screen if the signal stays strong for the full timer duration
   - This prevents accidental unlocks from brief signal fluctuations

2. The screen won't unlock if:
   - Bluetooth is turned off
   - Your device is not connected
   - The signal is weaker than `unlock_threshold`
   - The timer hasn't completed yet

### 🔄 Continuous Monitoring
- The tool checks your device's signal every `poll_interval` seconds
- This means it's always keeping an eye on your device's position
- You can adjust how often it checks in the configuration

## ⚠️ Note on hcitool

Currently, the project uses `hcitool` from the `bluez-deprecated-tools` package for Bluetooth signal strength measurement. This is a temporary solution as attempts to use the Rust bluez implementation `bluer` did not return valid RSSI values. Future versions will aim to replace this with a more modern solution.

## 📝 License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

- [Hyprland](https://hyprland.org/) for the amazing window manager
- [hyprlock](https://github.com/hyprwm/hyprlock) for the screen locker 