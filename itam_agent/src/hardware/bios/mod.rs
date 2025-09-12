// define shared struct
#[derive(Debug)]
pub struct BiosInfo {
    asset_tag: String,
    system_manufacturer: String,
    system_model: String,
    system_serial: String,
    bios_version: String,
    bios_date: String,
    bios_manufacturer: String,
    motherboard_manufacturer: String,
    motherboard_model: String,
    motherboard_serial: String,
    bios_or_system_type: String,
}

// declare os specific modules
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;

// re-export the selected os as alias
#[cfg(target_os = "macos")]
use macos as os;
#[cfg(target_os = "linux")]
use linux as os;

// rename this to new
// implement the shared function
pub fn get_bios_info() -> Result<BiosInfo, String> {
    os::get_bios_info()
}
