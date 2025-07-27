mod command;
use log;
use flexi_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::try_with_env_or_str("trace")?.log_to_stdout().start()?;

    log::debug!("starting with the name of Allah the most Merciful the most Benovelent");
    log::info!("agent execution started");

    // system UUID
    log::trace!("fetching system UUID");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Hardware UUID' | awk -F ':' {'print $2'}") {
        Ok(system_uuid) => {
            log::debug!("system UUID: {}", system_uuid);
        }
        Err(e) => {
            log::error!("fetching system UUID failed with: {}", e);
        }
    }

    // system model
    log::trace!("fetching system model");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Model Identifier' | awk -F ':' {'print $2'}") {
        Ok(model_identifier) => {
            log::debug!("system model: {}", model_identifier);
        }
        Err(e) => {
            log::error!("fetching system model failed with: {}", e);
        }
    }

    // system serial number
    log::trace!("fetching system serial number");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'Serial Number' | awk -F ':' {'print $2'}") {
        Ok(serial_number) => {
            log::debug!("system serial number: {}", serial_number);
        }
        Err(e) => {
            log::error!("fetching system serial number failed with: {}", e);
        }
    }

    // system firmware version
    log::trace!("fetching system firmware version");
    match command::execute_command("system_profiler SPHardwareDataType | grep 'System Firmware Version' | awk -F ':' {'print $2'}") {
        Ok(firmware_version) => {
            log::debug!("system firmware version: {}", firmware_version);
        }
        Err(e) => {
            log::error!("fetching system firmware version failed with: {}", e);
        }
    }

    log::info!("agent executed successfully");
    Ok(())
}
