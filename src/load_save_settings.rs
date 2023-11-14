use std::fs::{self, File};
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::prelude::LEVEL_DEFAULT;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub level: usize,
}

pub fn load_settings(settings_path: &str) -> Settings {
    let results = fs::read_to_string(settings_path);

    match results {
        Ok(contents) => {
            let settings: Settings = serde_json::from_str(&contents).unwrap();

            settings
        }
        Err(_) => {
            Settings {
                level: LEVEL_DEFAULT,
            }

            // let json = serde_json::to_string(&settings).unwrap();

            // let mut result = File::create(settings_path);
            // match &mut result {
            //     Ok(file) => {
            //         file.write_all(json.as_bytes());
            //     }
            //     Err(_) => {
            //     }

            // }

            // settings
        }
    }
}

pub fn save_settings(settings: &Settings, settings_path: &str) {
    let json = serde_json::to_string(settings).unwrap();

    let mut results = File::create(settings_path);

    match &mut results {
        Ok(file) => {
            let _ = file.write_all(json.as_bytes());
        }
        Err(_) => {}
    }
}
