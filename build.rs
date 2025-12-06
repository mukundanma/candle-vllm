use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("../../../examples");
    fs::create_dir_all(&dest_path).unwrap();
    fs_extra::dir::copy("examples", &dest_path, &fs_extra::dir::CopyOptions::new()).unwrap();
}
