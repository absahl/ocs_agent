use crate::command;

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

pub fn get_os_name() -> Result<String, String> {
    log::trace!("fetching OS name");
    match command::execute_command("system_profiler SPSoftwareDataType | grep 'System Version' | awk -F ':' {' print $2 '} | cut -d' ' -f2") {
        Ok(os_name) => {
            let trimmed = os_name.trim().to_string();
            log::debug!("fetched OS name: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch OS name: {}", e);
            Err(e)
        }
    }
}

pub fn get_os_version() -> Result<String, String> {
    log::trace!("fetching OS version");
    match command::execute_command("system_profiler SPSoftwareDataType | grep 'System Version' | awk -F ':' {' print $2 '} | cut -d' ' -f3-") {
        Ok(os_version) => {
            let trimmed = os_version.trim().to_string();
            log::debug!("fetched OS version: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch OS version: {}", e);
            Err(e)
        }
    }
}

pub fn get_os_comments() -> Result<String, String> {
    log::trace!("fetching OS comments");
    match command::execute_command("uname -v") {
        Ok(os_comments) => {
            let trimmed = os_comments.trim().to_string();
            log::debug!("fetched OS comments: {}", trimmed);
            Ok(trimmed)
        }
        Err(e) => {
            log::error!("failed to fetch OS comments: {}", e);
            Err(e)
        }
    }
}
