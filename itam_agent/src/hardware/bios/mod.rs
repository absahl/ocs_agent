#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;

pub fn get_system_uuid() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    return macos::get_system_uuid();

    #[cfg(target_os = "linux")]
    return linux::get_system_uuid();
}

pub fn get_system_model() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    return macos::get_system_model();

    #[cfg(target_os = "linux")]
    return linux::get_system_model();
}

pub fn get_system_model_name() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    return macos::get_system_model_name();

    #[cfg(target_os = "linux")]
    return linux::get_system_model_name();
}

pub fn get_system_serial_number() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    return macos::get_system_serial_number();

    #[cfg(target_os = "linux")]
    return linux::get_system_serial_number();
}

pub fn get_system_firmware_version() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    return macos::get_system_firmware_version();

    #[cfg(target_os = "linux")]
    return linux::get_system_firmware_version();
}
