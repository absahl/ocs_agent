use crate::command;

pub fn get_system_uuid() -> Result<String, String> {
    log::debug!("fetching system UUID");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Hardware UUID' | awk -F ':' {'print $2'}") {
        Ok(system_uuid) => {
            log::debug!("system UUID: {}", system_uuid);
            Ok(system_uuid)
        }
        Err(e) => {
            log::error!("fetching system UUID failed with: {}", e);
            Err(e)
        }
    }
}

pub fn get_system_model() -> Result<String, String> {
    log::debug!("fetching system model");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Model Identifier' | awk -F ':' {'print $2'}") {
        Ok(model_identifier) => {
            log::debug!("system model: {}", model_identifier);
            Ok(model_identifier)
        }
        Err(e) => {
            log::error!("fetching system model failed with: {}", e);
            Err(e)
        }
    }
}

pub fn get_system_serial_number() -> Result<String, String> {
    log::debug!("fetching system serial number");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Serial Number' | awk -F ':' {'print $2'}") {
        Ok(serial_number) => {
            log::debug!("system serial number: {}", serial_number);
            Ok(serial_number)
        }
        Err(e) => {
            log::error!("fetching system serial number failed with: {}", e);
            Err(e)
        }
    }
}

pub fn get_system_firmware_version() -> Result<String, String> {
    log::debug!("fetching system firmware version");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'System Firmware Version' | awk -F ':' {'print $2'}") {
        Ok(firmware_version) => {
            log::debug!("system firmware version: {}", firmware_version);
            Ok(firmware_version)
        }
        Err(e) => {
            log::error!("fetching system firmware version failed with: {}", e);
            Err(e)
        }
    }
}
