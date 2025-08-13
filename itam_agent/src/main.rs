mod command;
mod os;
use log;
use flexi_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::try_with_env_or_str("trace")?.log_to_stdout().start()?;

    log::debug!("starting with the name of Allah the most Merciful the most Benovelent");
    log::info!("agent execution started");


    // system UUID
    log::trace!("fetching system UUID");
    match os::get_system_uuid() {
        Ok(system_uuid) => log::debug!("system UUID: {}", system_uuid.trim()),
        Err(e) => log::error!("failed to fetch system UUID: {}", e),
    }

    // system model
    log::trace!("fetching system model");
    match os::get_system_model() {
        Ok(system_model) => log::debug!("system model: {}", system_model.trim()),
        Err(e) => log::error!("failed to fetch system model: {}", e),
    }

    // system serial number
    log::trace!("fetching system serial number");
    match os::get_system_serial_number() {
        Ok(system_serial_number) => log::debug!("system serial number: {}", system_serial_number.trim()),
        Err(e) => log::error!("failed to fetch system serial number: {}", e),
    }

    // system firmware version
    log::trace!("fetching system firmware version");
    match os::get_system_firmware_version() {
        Ok(system_firmware_version) => log::debug!("system firmware version: {}", system_firmware_version.trim()),
        Err(e) => log::error!("failed to fetch system firmware version: {}", e),
    }

    log::info!("agent executed successfully");

    Ok(())
}
