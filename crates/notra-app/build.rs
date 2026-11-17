fn main() {
    println!("cargo:rerun-if-changed=icons/icon.ico");
    println!("cargo:rerun-if-changed=frontend/dist/index.html");
    tauri_build::build();
}
