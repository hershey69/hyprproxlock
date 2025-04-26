# 🔒 Hyprproxlock: Proximity-Based Screen Locking for Hyprland

![Hyprproxlock](https://img.shields.io/badge/Hyprproxlock-v1.0-blue?style=flat-square)

Welcome to **Hyprproxlock**, a powerful daemon designed for Hyprland. This tool triggers screen locking and unlocking based on the proximity of Bluetooth devices. If you’re looking for a seamless way to secure your Linux environment, you’ve come to the right place.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Supported Platforms](#supported-platforms)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Releases](#releases)

## Features

- **Proximity-Based Locking**: Automatically locks your screen when you move away from your Bluetooth device.
- **Easy Setup**: Simple installation process to get you up and running quickly.
- **Customizable**: Adjust settings to fit your needs.
- **Lightweight**: Minimal resource usage, ensuring your system remains responsive.

## Installation

To install **Hyprproxlock**, follow these steps:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/hershey69/hyprproxlock.git
   cd hyprproxlock
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Run the daemon**:
   ```bash
   ./target/release/hyprproxlock
   ```

For pre-built binaries, visit the [Releases](https://github.com/hershey69/hyprproxlock/releases) section to download the latest version. Download the appropriate file and execute it.

## Usage

Once installed, **Hyprproxlock** runs in the background, monitoring the proximity of your Bluetooth devices. When you step away from your device, it will automatically lock your screen. To unlock, simply return within range of the Bluetooth device.

### Starting the Daemon

To start the daemon manually, use the following command:
```bash
./target/release/hyprproxlock
```

### Stopping the Daemon

To stop the daemon, you can use:
```bash
pkill hyprproxlock
```

## Configuration

You can customize **Hyprproxlock** by editing the configuration file located in your home directory:

```bash
~/.config/hyprproxlock/config.toml
```

### Example Configuration

```toml
[bluetooth]
device = "00:1A:7D:DA:71:13" # Replace with your device's MAC address
lock_timeout = 300 # Time in seconds before locking
```

Adjust the `device` field with the MAC address of your Bluetooth device and set the `lock_timeout` as needed.

## Supported Platforms

**Hyprproxlock** is designed for Linux systems and works best with Hyprland. It supports various distributions, including:

- Arch Linux
- Ubuntu
- Fedora

Ensure that your system has the necessary Bluetooth libraries installed.

## Contributing

We welcome contributions to **Hyprproxlock**! If you’d like to help, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes.
4. Push your branch and create a pull request.

For any major changes, please open an issue first to discuss what you would like to change.

## License

**Hyprproxlock** is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contact

For questions or feedback, feel free to reach out:

- **Email**: support@hyprproxlock.com
- **Twitter**: [@hyprproxlock](https://twitter.com/hyprproxlock)

## Releases

To stay updated with the latest features and fixes, check the [Releases](https://github.com/hershey69/hyprproxlock/releases) section regularly. Download the latest version to enjoy improvements and new features.

---

Thank you for using **Hyprproxlock**! We hope this tool enhances your productivity and security. If you encounter any issues, please report them in the GitHub issues section. Your feedback helps us improve!

![Lock Screen](https://img.shields.io/badge/Lock%20Screen-Enabled-green?style=flat-square)