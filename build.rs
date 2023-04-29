use os_info::Type;

fn main() {
    let info = os_info::get();

    let (search_path, is_linux) = match info.os_type() {
        Type::Ubuntu => ("/usr/lib/x86_64-linux-gnu", true),
        Type::Fedora => ("/usr/lib64", true),
        Type::Macos => ("/opt/homebrew/lib", false),
        _ => panic!("on unknown OS, plz add it to match clause"),
    };

    println!("cargo:rustc-link-search={search_path}");

    if is_linux {
        println!("cargo:rustc-link-lib=SDL2");
        println!("cargo:rustc-link-lib=SDL2_image");
        println!("cargo:rustc-link-lib=SDL2_ttf");
    } else {
        println!("cargo:rustc-link-lib=sdl2");
        println!("cargo:rustc-link-lib=sdl2_image");
        println!("cargo:rustc-link-lib=sdl2_ttf");
    }
}
