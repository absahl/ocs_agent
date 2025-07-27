use std::process::Command;
use log;

pub fn execute_command(command: &str) -> Result<String, String> {
    log::trace!("executing command [{}]", command);
    let result = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match result {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(format!(
                    "{}:{}",
                    output.status,
                    String::from_utf8_lossy(&output.stderr).trim()
                ))
            }
        }
        Err(e) => Err(format!("command execution failed with: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_command() {
        let result = execute_command("system_profiler SPHardwareDataType | grep 'Hardware UUID' | awk -F ':' {'print $2'}");
        match result {
            Ok(system_uuid) => assert_eq!(system_uuid, "B4569DDD-051D-5670-8150-AC13E67E38CF"),
            Err(_) => panic!(""),
        }
    }
}
