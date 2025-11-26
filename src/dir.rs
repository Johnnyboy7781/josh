use std::env::current_dir;

use dirs::home_dir;

pub fn get_home() -> String {
    match home_dir() {
        Some(buf) => {
            if let Some(home_str) = buf.to_str() {
                return String::from(home_str);
            } else {
                eprintln!("Home directory path invalid!");
                return String::from("");
            }
        }
        None => {
            eprintln!("Could not determine home directory!");
            return String::from("");
        }
    };
}

pub fn get_curr() -> String {
    match current_dir() {
        Ok(buf) => {
            if let Some(home_str) = buf.to_str() {
                return String::from(home_str);
            } else {
                eprintln!("Current directory path invalid!");
                return String::from("");
            }
        }
        Err(e) => {
            eprintln!("Could not determine current directory: {}", e);
            return String::from("");
        }
    };
}
