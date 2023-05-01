extern crate glob;
extern crate image;
extern crate texture_packer;

use glob::glob;
use std::io::Write;

use serde::{Deserialize, Serialize};
use std::fs::File;
use texture_packer::Rect;
use texture_packer::{
    exporter::ImageExporter, importer::ImageImporter, texture::Texture, TexturePacker,
    TexturePackerConfig,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRect {
    /// Horizontal position the rectangle begins at.
    pub x: u32,
    /// Vertical position the rectangle begins at.
    pub y: u32,
    /// Width of the rectangle.
    pub w: u32,
    /// Height of the rectangle.
    pub h: u32,
}

#[derive(Serialize, Deserialize)]
struct Img {
    rect: JsonRect,
    filename: String,
}

#[derive(Serialize, Deserialize)]
struct Imgs {
    imgs: Vec<Img>,
}

fn main() {
    // let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // d.push("assets");

    // let paths = fs::read_dir("./assets").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }

    work();
}

fn work() {
    //
    // Perform texture packing
    //
    let config = TexturePackerConfig {
        max_width: 20000,
        max_height: 20000,
        allow_rotation: false,
        texture_outlines: false,
        border_padding: 0,
        trim: false,
        ..Default::default()
    };

    let mut packer = TexturePacker::new_skyline(config);

    let entries = glob("./imgs/*.png").unwrap();

    for path in entries {
        let path = path.unwrap();
        println!("path: {:#?}", path);
        let texture = ImageImporter::import_from_file(&path)
            .expect("Unable to import file. Run this example with --features=\"png\"");

        let filename = path.file_name().unwrap().to_owned();
        println!("pack_own: {:#?}", filename);
        packer.pack_own(filename, texture).unwrap();
    }

    //
    // Print the information
    //
    println!("Dimensions : {}x{}", packer.width(), packer.height());
    let mut imgs: Vec<Img> = Vec::new();
    for (name, frame) in packer.get_frames() {
        println!("  {:#?} : {:#?}", name, frame);

        let Rect { x, y, h, w } = frame.frame;

        let filename: String = name.to_string_lossy().to_string();

        let rect = JsonRect { x, y, w, h };

        let img = Img { filename, rect };

        imgs.push(img);
    }

    //
    // Save the result
    //
    let exporter = ImageExporter::export(&packer).unwrap();
    let mut file = File::create("resources/skyline-packer-output.png").unwrap();
    exporter
        .write_to(&mut file, image::ImageFormat::Png)
        .unwrap();

    println!("Output texture stored in {:?}", file);

    let imgs = Imgs { imgs };
    let serialized = serde_json::to_string(&imgs).unwrap();

    let path = "./resources/img_data.json";
    let mut output = File::create(path).unwrap();
    write!(output, "{}", serialized).unwrap();
}
