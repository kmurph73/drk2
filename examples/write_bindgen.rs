use std::path::PathBuf;

use os_info::Type;

extern crate bindgen;
pub fn main() {
    let info = os_info::get();

    let is_linux = match info.os_type() {
        Type::Ubuntu => true,
        Type::Fedora => true,
        Type::Macos => false,
        _ => panic!("on unknown OS, plz add it to match clause"),
    };

    let include_path = if is_linux {
        "-I/usr/include/SDL2"
    } else {
        "-I/opt/homebrew/include/SDL2"
    };

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

    let out_path = PathBuf::from(".");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
