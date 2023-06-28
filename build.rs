fn main() {
    let search_path = "/usr/lib64";

    println!("cargo:rustc-link-search={search_path}");

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
    println!("cargo:rustc-link-lib=SDL2_ttf");
}
