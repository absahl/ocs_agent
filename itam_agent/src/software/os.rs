use crate::command;

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
