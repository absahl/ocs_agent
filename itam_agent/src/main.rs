mod command;
mod hardware;
mod software;
use log;
use flexi_logger::Logger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Logger::try_with_env_or_str("trace")?.log_to_stdout().start()?;

    log::debug!("starting with the name of Allah the most Merciful the most Benovelent");
    log::info!("agent execution started");

    // bios info
    let bios_info = hardware::bios::get_bios_info();
    match bios_info {
        Ok(bios_info) => log::debug!("bios info: {:?}", bios_info),
        Err(e) => log::error!("failed to fetch bios info: {}", e),
    }

    match software::os::get_os_name() {
        Ok(os_name) => log::debug!("os name: {}", os_name),
        Err(e) => log::error!("failed to fetch os name: {}", e),
    }

    match software::os::get_os_version() {
        Ok(os_version) => log::debug!("os version: {}", os_version),
        Err(e) => log::error!("failed to fetch os version: {}", e),
    }

    match software::os::get_os_comments() {
        Ok(os_comments) => log::debug!("os comments: {}", os_comments),
        Err(e) => log::error!("failed to fetch os comments: {}", e),
    }

    log::info!("agent executed successfully");

    Ok(())
}
