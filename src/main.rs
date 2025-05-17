use std::process::Command;
use regex::Regex;
use colored::*;
use std::thread;
use std::time::Duration;
use std::io::{self, Write, BufReader, BufRead};
use std::collections::HashMap;

// Struct to hold parsed USB device information
struct UsbDevice {
    bus: String,
    dev: String,
    iface: String,
    speed: String,
    category: String,
    category_color: Color,
    status: String,
    status_color: Color,
    details: String, // From lsusb -t
    connected_device: String, // From lsusb
}

fn main() {
    loop {
        // Clear screen and move cursor to top-left for refresh
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();

        // Get USB topology
        let output = Command::new("lsusb")
            .arg("-t")
            .output()
            .expect("Failed to execute lsusb -t");
        let lsusb_t = String::from_utf8_lossy(&output.stdout);
        
        // Get standard lsusb output for device descriptions
        let output_lsusb = Command::new("lsusb")
            .output()
            .expect("Failed to execute lsusb");
        let lsusb_output = String::from_utf8_lossy(&output_lsusb.stdout);

        // This regex captures Port, Dev, If, and the rest of the line starting from Class=
        let re = Regex::new(r"^\s*\|__ Port (\d+): Dev (\d+), If (\d+), Class=[^,]*, Driver=[^,]*, (.*)").unwrap();
        // Regex to extract speed (e.g., 480, 5000, 12, 1.5) from the captured details or the whole line
        let re_speed = Regex::new(r"(\d+(?:\.\d+)?)M").unwrap();

        // Modern, simple, and beautiful table style
        let border = "────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────"; // Adjusted length to 120
        println!("\n{}", "USB Ports Status Report (Refreshing every 5s)".bold().underline().blue());
        println!("{}", border.bright_black());
        
        // Updated header with new columns
        println!(
            "{:^5}  {:^5}  {:^5}  {:^10}  {:<15}  {:<8}  {:<8}  {:<50}", // Details width 20->8, Connected Device 30->50
            "Bus".bold(), "Dev".bold(), "If".bold(), "Speed".bold(), "Category".bold(), "Status".bold(), "Details".bold(), "Connected Device".bold()
        );
        println!("{}", border.bright_black());

        // --- Parse lsusb output to get device descriptions ---
        let re_lsusb = Regex::new(r"^Bus\s+(\d+)\s+Device\s+(\d+):\s+(.*)").unwrap();
        let mut description_map: HashMap<(String, String), String> = HashMap::new();

        for line in lsusb_output.lines() {
            if let Some(caps) = re_lsusb.captures(line) {
                let bus = caps[1].to_string();
                let dev = caps[2].to_string();
                let description = caps[3].to_string();
                description_map.insert((bus, dev), description);
            }
        }

        // --- Parse lsusb -t output statefully to get Bus and other details ---
        let re_bus_line = Regex::new(r"^/:\s+Bus\s+(\d+)\.").unwrap();
        let mut current_bus: Option<String> = None;
        let mut devices_to_display: Vec<UsbDevice> = Vec::new();

        // Use BufReader for line-by-line processing which is slightly more robust
        let reader = BufReader::new(lsusb_t.as_bytes());
        let mut any_match = false;
        for line_result in reader.lines() {
            let line = line_result.expect("Failed to read line from lsusb -t output");
            if let Some(caps) = re.captures(&line) {
                any_match = true;
                let _port = &caps[1]; // Renamed to indicate it's intentionally not used further
                let dev = &caps[2];
                let iface = &caps[3];
                let details = &caps[4]; // Capture the rest of the line after Driver=

                // Get the current bus context
                let bus = current_bus.clone().unwrap_or_else(|| "Unknown".to_string());

                // Look up the connected device description
                let connected_device = description_map.get(&(bus.clone(), dev.to_string())).cloned().unwrap_or_else(|| "N/A".to_string());
                // Extract speed (e.g., 480, 5000, 12, 1.5) from the original line
                let speed = re_speed.captures(&line).map(|c| c[1].to_string()).unwrap_or("Unknown".to_string());

                // Determine category based on speed
                let (category, cat_color) = match speed.as_str() {
                    "1.5" => ("USB 1.0 (Low)", Color::Yellow),
                    "12"  => ("USB 1.1 (Full)", Color::Yellow),
                    "480" => ("USB 2.0 (High)", Color::Blue),
                    "5000" => ("USB 3.0 (SS)", Color::Cyan),
                    "10000" => ("USB 3.1 (SS+)", Color::Magenta),
                    _ => ("Unknown", Color::White),
                };

                let (status, status_color) = if dev == "0" {
                    ("Inactive", Color::Red)
                } else {
                    ("Active", Color::Green)
                };
                let speed_display = if speed != "Unknown" {
                    format!("{} Mbps", speed)
                } else {
                    "Unknown".to_string()
                };

                devices_to_display.push(UsbDevice {
                    bus,
                    dev: dev.to_string(),
                    iface: iface.to_string(),
                    speed: speed_display,
                    category: category.to_string(),
                    category_color: cat_color,
                    status: status.to_string(),
                    status_color: status_color,
                    details: details.to_string(),
                    connected_device,
                });
            } else if let Some(caps) = re_bus_line.captures(&line) {
                // Update the current bus context when a bus line is encountered
                current_bus = Some(caps[1].to_string());
            }
        }

        // --- Print the collected device information ---
        for device in devices_to_display {
            println!(
                "{:^5}  {:^5}  {:^5}  {:^10}  {:<15}  {:<8}  {:<8}  {:<50}", // Details width 20->8, Connected Device 30->50
                device.bus.bright_white(),
                device.dev.bright_white(),
                device.iface.bright_white(),
                device.speed.bright_white(),
                device.category.color(device.category_color).bold(),
                device.status.color(device.status_color).bold(),
                device.details.bright_black(),
                device.connected_device.bright_white() // Print the connected device
            );
        }

        if !any_match {
            println!("{:^120}", "No detailed matches found. Showing lines containing 'Port':".yellow()); // Adjusted centering width
            for line in lsusb_t.lines() {
                if line.contains("Port") {
                    println!("{:^120}", line.trim().yellow()); // Adjusted centering width
                }
            };
        }
        println!("{}\n", border.bright_black());
        println!("{}", "Legend:".bold().underline());
        // Updated Legend for new columns and structure
        println!("  {}  - USB Bus number", "Bus".bold());
        println!("  {}  - Device number ({} = no device)", "Dev".bold(), "0".red());
        println!("  {}  - Interface number", "If".bold());
        println!("  {}  - Reported port speed", "Speed".bold());
        println!("  {}  - Specification category:", "Category".bold()); // Category legend remains vertical
        println!("              {}: {}", "USB 1.x (Low/Full)".yellow(), "yellow"); // 1.0 and 1.1 grouped
        println!("              {}: {}", "USB 2.0 (High)".blue(), "blue");
        println!("              {}: {}", "USB 3.0 (SS)".cyan(), "cyan");
        println!("              {}: {}", "USB 3.1 (SS+)".magenta(), "magenta");
        println!("  {}  - {} if a device is connected, {} if not", "Status".bold(), "Active".green(), "Inactive".red());
        println!("  {}  - Additional info from lsusb -t", "Details".bold());
        println!("  {}  - Description from lsusb", "Connected Device".bold());

        // Wait for 5 seconds before the next refresh
        thread::sleep(Duration::from_secs(5));
    }
}