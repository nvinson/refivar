use std::env;
use std::fs;
use std::path;

fn main() {
    let prefix = env::var("OUT_DIR").unwrap();
    let sys_conf_dir = env::var("SYSCONFDIR").unwrap_or("share".to_string());

    let target_dir = path::Path::new(&sys_conf_dir).join("refivar");
    let target_path = path::Path::new(&target_dir).join("guids.json");
    let dest_dir = path::Path::new(&prefix).join(&target_dir);
    let dest_path = path::Path::new(&prefix).join(&target_path);
    let src_path = path::Path::new(&std::env::current_dir().unwrap())
        .join("src")
        .join("lib")
        .join("efivar")
        .join("guids.json");

    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir).unwrap();
    }
    fs::copy(src_path, dest_path).unwrap();

    print!("cargo:rustc-env=GUIDS_LIST_PATH={}", target_path.display());
}
