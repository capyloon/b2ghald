/// Timezone & clock functions.
use std::process::Command;

pub enum TimezoneError {
    SetError,
    GetError,
}

pub struct Timezone {}

impl Timezone {
    pub fn set(tz: &str) -> Result<(), TimezoneError> {
        if let Ok(status) = Command::new("timedatectl")
            .arg("set-timezone")
            .arg(tz)
            .status()
        {
            if status.success() {
                return Ok(());
            } else {
                return Err(TimezoneError::SetError);
            }
        }
        Err(TimezoneError::SetError)
    }

    pub fn get() -> Result<String, TimezoneError> {
        if let Ok(output) = Command::new("timedatectl").arg("show").output() {
            let s = String::from_utf8_lossy(&output.stdout);
            for line in s.split('\n') {
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() == 2 && parts[0] == "Timezone" {
                    return Ok(parts[1].to_owned());
                }
            }
        }

        Err(TimezoneError::GetError)
    }
}
