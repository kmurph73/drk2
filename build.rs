fn main() {
    let search_path = "/opt/homebrew/lib";

    println!("cargo:rustc-link-search={search_path}");

    println!("cargo:rustc-link-lib=sdl2");
    println!("cargo:rustc-link-lib=sdl2_image");
    println!("cargo:rustc-link-lib=sdl2_ttf");
}
