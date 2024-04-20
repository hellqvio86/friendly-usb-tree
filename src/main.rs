use log::debug;
use std::collections::HashMap;
use std::process::{Command, Output};

// Struct to hold USB device information
#[derive(Debug)]
struct UsbDeviceInfo {
    vendor_id: String,
    product_id: String,
    friendly_name: String,
}

fn parse_lsusb_output(output: Output) -> HashMap<String, UsbDeviceInfo> {
    // Check if lsusb executed successfully
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error running lsusb: {}", error);
        return HashMap::new();
    }

    // Convert the output to a string
    let lsusb_output = String::from_utf8_lossy(&output.stdout);

    // Create an empty HashMap to store USB device information
    let mut device_info_map: HashMap<String, UsbDeviceInfo> = HashMap::new();

    // Iterate over each line of lsusb output
    for line in lsusb_output.lines() {
        // Split each line by whitespace
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        // Check if there are enough parts in the line
        if parts.len() >= 6 {
            // Extract vendor ID and product ID
            let id_parts: Vec<&str> = parts[5].split(":").collect();
            let vendor_id = id_parts[0].to_string(); // Convert to String
            let product_id = id_parts[1].to_string(); // Convert to String

            // Extract the friendly name
            let friendly_name = parts[6..].join(" ");

            // Construct the key using vendor ID and product ID
            let key = format!("{}:{}", vendor_id, product_id);

            // Create a UsbDeviceInfo struct
            let usb_device_info = UsbDeviceInfo {
                vendor_id,
                product_id,
                friendly_name: friendly_name.to_string(),
            };

            // Debug print device information before adding to the map
            debug!("Adding device: {:?} - {:?}", key, usb_device_info);

            // Insert the key-value pair into the device_info_map
            device_info_map.insert(key, usb_device_info);
        }
    }

    device_info_map
}

fn main() {
    // Execute the lsusb command
    let output = Command::new("lsusb")
        .output()
        .expect("Failed to execute lsusb");

    // Parse lsusb output and store the information in a HashMap
    let device_info_map = parse_lsusb_output(output);

    // Print the HashMap
    println!("{:#?}", device_info_map);
}
