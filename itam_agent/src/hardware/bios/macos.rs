use crate::command;
use super::BiosInfo;

pub fn get_bios_info() -> Result<BiosInfo, String> {
    Ok(BiosInfo {
        asset_tag: String::new(),
        system_manufacturer: "Apple Inc".to_string(),
        system_model: command::execute_command("system_profiler SPHardwareDataType | grep 'Model Identifier' | awk -F ':' {'print $2'}")?,
        system_serial: command::execute_command("system_profiler SPHardwareDataType | grep 'Serial Number' | awk -F ':' {'print $2'}")?,
        bios_version: command::execute_command("system_profiler SPHardwareDataType | grep 'System Firmware Version' | awk -F ':' {'print $2'}")?,
        bios_date: String::new(),
        bios_manufacturer: "Apple Inc".to_string(),
        motherboard_manufacturer: "Apple Inc".to_string(),
        motherboard_model: String::new(),
        motherboard_serial: String::new(),
        bios_or_system_type: command::execute_command("system_profiler SPHardwareDataType | grep 'Model Name' | awk -F ':' {'print $2'}")?,
    })
}
