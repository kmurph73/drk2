fn main() {
    // linux path: "/usr/lib64";
    let search_path = "/opt/homebrew/lib";

    println!("cargo:rustc-link-search={search_path}");

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
}
