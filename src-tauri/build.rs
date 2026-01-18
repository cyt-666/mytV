use std::path::Path;

fn main() {
    // 从项目根目录加载 .env 文件（src-tauri 的父目录）
    let env_path = Path::new("../.env");
    if env_path.exists() {
        for item in dotenvy::from_path_iter(env_path).unwrap().flatten() {
            println!("cargo:rustc-env={}={}", item.0, item.1);
        }
        // 当 .env 文件变化时重新构建
        println!("cargo:rerun-if-changed=../.env");
    }

    tauri_build::build()
}
