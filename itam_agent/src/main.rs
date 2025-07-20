use std::process::Command;

fn execute_command(command: &str) -> Result<String, String> {
    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(format!(
                    "{}:{}",
                    output.status,
                    String::from_utf8_lossy(&output.stderr).trim()
                ))
            }
        }
        Err(e) => Err(format!("command execution failed with: {}", e)),
    }
}

fn main() {
    println!("starting with the name of Allah the most Merciful the most Benovelent");

    // system UUID
    println!("fetching system UUID");
    match execute_command("system_profiler SPHardwareDataType | grep 'Hardware UUID' | awk -F ':' {'print $2'}") {
        Ok(system_uuid) => {
            println!("system UUID: {}", system_uuid);
        }
        Err(e) => {
            println!("fetching system UUID failed with: {}", e);
        }
    }

    // system model
    println!("fetching system model");
    match execute_command("system_profiler SPHardwareDataType | grep 'Model Identifier' | awk -F ':' {'print $2'}") {
        Ok(model_identifier) => {
            println!("system model: {}", model_identifier);
        }
        Err(e) => {
            println!("fetching system model failed with: {}", e);
        }
    }

    // system serial number
    println!("fetching system serial number");
    match execute_command("system_profiler SPHardwareDataType | grep 'Serial Number' | awk -F ':' {'print $2'}") {
        Ok(serial_number) => {
            println!("system serial number: {}", serial_number);
        }
        Err(e) => {
            println!("fetching system serial number failed with: {}", e);
        }
    }

    // system firmware version
    println!("fetching system firmware version");
    match execute_command("system_profiler SPHardwareDataType | grep 'System Firmware Version' | awk -F ':' {'print $2'}") {
        Ok(firmware_version) => {
            println!("system firmware version: {}", firmware_version);
        }
        Err(e) => {
            println!("fetching system firmware version failed with: {}", e);
        }
    }
}
