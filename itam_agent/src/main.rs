use std::process::Command;

fn get_system_uuid() -> Result<String, String> {
    let result = Command::new("bash")
        .arg("-c")
        .arg("system_profiler SPHardwareDataType | grep 'Hardware UUID' | awk {'print $3'}")
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

fn main() {
    println!("Starting with the name of Allah the most Merciful the most Benovelent");
    match get_system_uuid() {
        Ok(uuid) => {
            println!("Command executed successfully!");
            println!("Command output: {}", uuid);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
