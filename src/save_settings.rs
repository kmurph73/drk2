use std::fs::{self, File};
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
    level: usize,
    speed: usize,
}
pub fn save_settings() {}
