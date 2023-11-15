use std::fs::{self, File};
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonImg {
    rect: JsonRect,
    pub filename: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRect {
    /// Horizontal position the rectangle begins at.
    pub x: i32,
    /// Vertical position the rectangle begins at.
    pub y: i32,
    /// Width of the rectangle.
    pub w: i32,
    /// Height of the rectangle.
    pub h: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonImgs {
    pub imgs: Vec<JsonImg>,
}

// #[derive(Serialize, Deserialize, Debug)]
pub fn main() {
    let contents = fs::read_to_string("./resources/img_data.json")
        .expect("Should have been able to read the file");

    let p: JsonImgs = serde_json::from_str(&contents).unwrap();

    let mut lines: Vec<String> = Vec::new();

    lines.push(String::from("pub const BASE_DOT_SIZE: i32 = 160;"));

    for img in &p.imgs {
        let file = &img.filename;
        let arr: Vec<&str> = file.split('.').collect();
        let name = arr[0].to_uppercase();
        let JsonRect { x, y, w, h } = img.rect;

        let line = format!(
            "pub const {}_IMG: (i32, i32, i32, i32) = ({}, {}, {}, {});",
            name, x, y, w, h,
        );
        lines.push(line);
    }

    // lines.push(String::from("pub struct Images {"));

    // for img in &p.imgs {
    //     let file = &img.filename;
    //     let arr: Vec<&str> = file.split('.').collect();
    //     let name = arr[0];

    //     lines.push(format!("    pub {}: Image,", name));
    // }

    // lines.push(String::from("}"));

    let mut file = File::create("./src/img_consts.rs");
    match &mut file {
        Ok(file) => {
            let code = lines.join("\n");

            let result = file.write_all(code.as_bytes());

            match result {
                Ok(_) => {
                    println!("img_consts.rs written");
                }
                Err(_) => {
                    println!("img_consts.rs NOT written");
                }
            }
        }
        Err(_) => {}
    }
}
