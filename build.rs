extern crate bindgen;
use std::io::Write;

use os_info::Type;
use regex::Regex;
use std::env;
use std::fs::File;
use std::path::PathBuf;

fn main() {
    let dir = env::current_dir().unwrap();
    let is_linux = dir.starts_with("/home/");
    println!("OUT: {:#?}", env::var("OUT_DIR"));

    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/usr/lib64");
    let search_path = if is_linux {
        let info = os_info::get();
        let is_ubuntu = info.os_type() == Type::Ubuntu;
        let is_fedora = info.os_type() == Type::Fedora;

        if is_ubuntu {
            // "/mnt/c/Users/kmurp/OneDrive/Documents/compiled_code/sdl2"
            "/usr/lib/x86_64-linux-gnu"
        } else if is_fedora {
            "/usr/lib64"
        } else {
            panic!("not using fedora or ubuntu... are you on arch, btw?")
        }
    } else {
        "/opt/homebrew/lib"
    };

    println!("cargo:rustc-link-search={search_path}");

    // println!("cargo:rustc-link-lib=libSDL2.a");

    // cc::Build::new()
    //     .file("/Users/kmurph/code/SDL/src/SDL.c")
    //     .include("/Users/kmurph/code/SDL/src")
    //     .compile("hello");

    // Tell cargo to tell rustc to link the system sdl2
    // shared library.
    if is_linux {
        println!("cargo:rustc-link-lib=SDL2");
        println!("cargo:rustc-link-lib=SDL2_image");
        println!("cargo:rustc-link-lib=SDL2_ttf");
    } else {
        println!("cargo:rustc-link-lib=sdl2");
        println!("cargo:rustc-link-lib=sdl2_image");
        println!("cargo:rustc-link-lib=sdl2_ttf");
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_path = if is_linux {
        "-I/usr/include/SDL2"
    } else {
        "-I/opt/homebrew/include/SDL2"
    };
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg(include_path)
        // .clang_arg("-I/opt/homebrew/include/SDL2")
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // bindings.
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(".");
    if is_linux {
        let str = bindings.to_string();
        let re = Regex::new(r"^(pub const) FP_.+_bindgen_ty").unwrap();
        let clippy_re = Regex::new(r"(i|u)128").unwrap();
        let lines: Vec<String> = str
            .split('\n')
            .filter_map(|line| {
                if re.is_match(line) {
                    None
                } else if clippy_re.is_match(line) {
                    let line = line.replace("128", "64");
                    Some(line)
                } else {
                    Some(line.to_string())
                }
            })
            .collect();

        let mut output = File::create("bindings.rs").unwrap();

        let line = lines.join("\n");

        write!(output, "{line}").unwrap();
    } else {
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }
}
