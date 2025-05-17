# USB Port Status Utility (usbport)

`usbport` is a command-line utility for Linux that displays detailed information about connected USB devices. It provides a live, color-coded view of USB bus topology, device speeds, categories, and their status, refreshing every 5 seconds.

## Features

* **Comprehensive Device Information:** Displays Bus number, Device ID, Interface, Speed, USB Specification Category, Status (Active/Inactive), and connected device name.
* **Live Refresh:** Automatically updates the displayed information every 5 seconds.
* **Color-Coded Output:** Uses colors for easy identification of USB categories and device status.
* **Detailed Topology:** Leverages `lsusb -t` for tree-like device details.
* **Device Naming:** Fetches device names from the standard `lsusb` output.

## Prerequisites

* **Linux Operating System:** This tool relies on the `lsusb` command, which is standard on most Linux distributions.
* **`lsusb` utility:** Ensure `lsusb` (usually part of the `usbutils` package) is installed. You can typically install it with:

    ```bash
    sudo apt-get install usbutils  # For Debian/Ubuntu based systems
    sudo yum install usbutils      # For Fedora/RHEL based systems
    sudo dnf install usbutils      # For modern Fedora systems
    # Or your distribution's equivalent
    ```

* **Rust Toolchain:** Required to build the project from source. If you don't have Rust, install it from rustup.rs.

## Installation & Building

1. **Clone the repository (if applicable) or navigate to the project directory:**

    ```bash
    # git clone <repository_url>
    # cd usb-port
    ```

2. **Build the project:**

    ```bash
    cargo build --release
    ```

3. The executable will be located at `target/release/usbport`.

## Usage

Run the compiled binary from its location:

```bash
./target/release/usbport
```

For easier access, you can copy it to a directory in your system's `PATH`, for example:

```bash
sudo cp target/release/usbport /usr/local/bin/
```

Then you can run it from anywhere by simply typing:

```bash
usbport
```

Press `Ctrl+C` to exit the utility.

## Example Output

```sh
USB Ports Status Report (Refreshing every 5s)
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
 Bus    Dev    If     Speed       Category         Status    Details   Connected Device
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
 001    005     0     1.5 Mbps   USB 1.0 (Low)    Active    1.5M      ID 093a:2510 Pixart Imaging, Inc. Optical Mouse
 001    002     0     480 Mbps   USB 2.0 (High)   Active    480M      ID 05e3:0608 Genesys Logic, Inc. Hub
 001    004     0     480 Mbps   USB 2.0 (High)   Active    480M      ID 1bcf:2c70 Sunplus Innovation Technology Inc. Integrated Camera
 001    004     1     480 Mbps   USB 2.0 (High)   Active    480M      ID 1bcf:2c70 Sunplus Innovation Technology Inc. Integrated Camera
 002    002     0     480 Mbps   USB 2.0 (High)   Active    480M      ID 8087:8001 Intel Corp. Integrated Hub
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────

Legend:
  Bus   - USB Bus number
  Dev  - Device number (0 = no device)
  If  - Interface number
  Speed  - Reported port speed
  Category  - Specification category:
              USB 1.x (Low/Full): yellow
              USB 2.0 (High): blue
              USB 3.0 (SS): cyan
              USB 3.1 (SS+): magenta
  Status  - Active if a device is connected, Inactive if not
  Details  - Additional info from lsusb -t
  Connected Device - Description from lsusb
```

## Known Issues / Limitations

* **Linux Only:** Due to its reliance on the `lsusb` command, this utility is specific to Linux environments.
* **`lsusb` Dependency:** The `lsusb` command must be installed and accessible in the system's PATH.

## Contributing

Contributions are welcome! If you have suggestions for improvements or find any bugs, please feel free to open an issue or submit a pull request.

## License

This project is currently unlicensed. Consider adding an open-source license like MIT or Apache 2.0 if you plan to share it more broadly.
