fn main() {
    let info = os_info::get();

    let search_path = match info.os_type() {
        os_info::Type::Macos => "/opt/homebrew/lib",
        _ => "/usr/lib64",
    };

    println!("cargo:rustc-link-search={search_path}");

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
}
