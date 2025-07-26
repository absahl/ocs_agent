mod command;

fn main() {
    println!("starting with the name of Allah the most Merciful the most Benovelent");

    // system UUID
    println!("fetching system UUID");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Hardware UUID' | awk -F ':' {'print $2'}") {
        Ok(system_uuid) => {
            println!("system UUID: {}", system_uuid);
        }
        Err(e) => {
            println!("fetching system UUID failed with: {}", e);
        }
    }

    // system model
    println!("fetching system model");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Model Identifier' | awk -F ':' {'print $2'}") {
        Ok(model_identifier) => {
            println!("system model: {}", model_identifier);
        }
        Err(e) => {
            println!("fetching system model failed with: {}", e);
        }
    }

    // system serial number
    println!("fetching system serial number");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Serial Number' | awk -F ':' {'print $2'}") {
        Ok(serial_number) => {
            println!("system serial number: {}", serial_number);
        }
        Err(e) => {
            println!("fetching system serial number failed with: {}", e);
        }
    }

    // system firmware version
    println!("fetching system firmware version");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'System Firmware Version' | awk -F ':' {'print $2'}") {
        Ok(firmware_version) => {
            println!("system firmware version: {}", firmware_version);
        }
        Err(e) => {
            println!("fetching system firmware version failed with: {}", e);
        }
    }
}
