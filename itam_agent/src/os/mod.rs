mod macos;


pub fn get_system_uuid() -> Result<String, String> {
    macos::get_system_uuid()
}

pub fn get_system_model() -> Result<String, String> {
    macos::get_system_model()
}

pub fn get_system_serial_number() -> Result<String, String> {
    macos::get_system_serial_number()
}

pub fn get_system_firmware_version() -> Result<String, String> {
    macos::get_system_firmware_version()
}

pub fn get_os_name() -> Result<String, String> {
    macos::get_os_name()
}

pub fn get_os_version() -> Result<String, String> {
    macos::get_os_version()
}

pub fn get_os_comments() -> Result<String, String> {
    macos::get_os_comments()
}
