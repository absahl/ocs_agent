use crate::command;

// perl equivalent: platform_UUID
// sh equivalent: system_profiler SPHardwareDataType | grep 'Hardware UUID'
// result: B4569DDD-051D-5670-8150-AC13E67E38CF
pub fn get_system_uuid() -> Result<String, String> {
    log::trace!("fetching system UUID");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Hardware UUID' | awk -F ':' {'print $2'}") {
        Ok(system_uuid) => {
            let trimmed = system_uuid.trim().to_string();
            log::debug!("fetched system UUID: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch system UUID: {}", e);
            Err(e)
        }
    }
}

// perl equivalent: machine_model
// sh equivalent: system_profiler SPHardwareDataType | grep 'Model Identifier'
// result: Mac15,6
pub fn get_system_model() -> Result<String, String> {
    log::trace!("fetching system model");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Model Identifier' | awk -F ':' {'print $2'}") {
        Ok(model_identifier) => {
            let trimmed = model_identifier.trim().to_string();
            log::debug!("fetched system model: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch system model: {}", e);
            Err(e)
        }
    }
}

// perl equivalent: machine_name
// sh equivalent: system_profiler SPHardwareDataType | grep 'Model Name'
// result: MacBook Pro
pub fn get_system_model_name() -> Result<String, String> {
    log::trace!("fetching system model name");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Model Name' | awk -F ':' {'print $2'}") {
        Ok(model_name) => {
            let trimmed = model_name.trim().to_string();
            log::debug!("fetched system model name: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch system model name: {}", e);
            Err(e)
        }
    }
}

// perl equivalent: serial_number
// sh equivalent: system_profiler SPHardwareDataType | grep 'Serial Number (system)'
// result: JJ97CDGJ9N
pub fn get_system_serial_number() -> Result<String, String> {
    log::trace!("fetching system serial number");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Serial Number' | awk -F ':' {'print $2'}") {
        Ok(serial_number) => {
            let trimmed = serial_number.trim().to_string();
            log::debug!("fetched system serial number: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch system serial number: {}", e);
            Err(e)
        }
    }
}

// perl equivalent: boot_rom_version
// sh equivalent: system_profiler SPHardwareDataType | grep 'System Firmware Version'
// result: 11881.140.96
pub fn get_system_firmware_version() -> Result<String, String> {
    log::trace!("fetching system firmware version");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'System Firmware Version' | awk -F ':' {'print $2'}") {
        Ok(firmware_version) => {
            let trimmed = firmware_version.trim().to_string();
            log::debug!("fetched system firmware version: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch system firmware version: {}", e);
            Err(e)
        }
    }
}
