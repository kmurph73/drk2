use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::prelude::{LEVEL_DEFAULT, SETTINGS_PATH, SPEED_DEFAULT};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub level: usize,
    pub speed: usize,
}

pub fn load_settings() -> Settings {
    Settings {
        level: LEVEL_DEFAULT,
        speed: SPEED_DEFAULT,
    }
    // let results = fs::read_to_string(SETTINGS_PATH);

    // match results {
    //     Ok(contents) => {
    //         let settings: Settings = serde_json::from_str(&contents).unwrap();

    //         settings
    //     }
    //     Err(_) => {
    //         let settings = Settings {
    //             level: LEVEL_DEFAULT,
    //             speed: SPEED_DEFAULT,
    //         };

    //         let json = serde_json::to_string(&settings).unwrap();

    //         let mut output = File::create(SETTINGS_PATH).unwrap();
    //         write!(output, "{json}").unwrap();

    //         settings
    //     }
    // }
}

pub fn save_settings(settings: &Settings) {
    let json = serde_json::to_string(settings).unwrap();

    let mut output = File::create(SETTINGS_PATH).unwrap();
    write!(output, "{json}").unwrap();
}
