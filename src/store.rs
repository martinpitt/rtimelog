use std::fs::{File};
use std::io::{self, prelude::*};

extern crate dirs;

pub fn read_log() -> String {
    let mut log_path = dirs::home_dir().expect("Cannot determine home directory");
    log_path.push(".gtimelog");
    log_path.push("timelog.txt");

    match File::open(&log_path) {
        Ok(mut f) => {
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect(&format!("Failed to read {}", log_path.display()));
            contents
        },

        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                println!("No existing {}, starting new log", log_path.display());
                String::new()
            } else {
                panic!("Could not open {}: {:?}", log_path.display(), e);
            }
        }
    }
}
