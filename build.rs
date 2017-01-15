extern crate glob;
extern crate serde_codegen;

use std::env;
use std::fs;
use glob::glob;
use std::path::Path;
use std::ffi::OsString;

fn gen_types(out_dir: &OsString) {
    let types_dir = Path::new(&out_dir).join("types");

    fs::create_dir_all(&types_dir).unwrap();

    for entry in glob("src/types/*.in.rs").unwrap() {
        if let Ok(src_filename) = entry {
            let src = Path::new(&src_filename);
            let type_ = src
                .file_name().unwrap()
                .to_str().unwrap()
                .split(".")
                .collect::<Vec<&str>>()[0];

            let dst_filename = format!("{}.rs", type_);
            let dst = types_dir.join(dst_filename);

            serde_codegen::expand(&src, &dst).unwrap();
        }
    }
}

fn gen_events(out_dir: &OsString) {
    let src = Path::new("src/events.in.rs");
    let dst = Path::new(&out_dir).join("events.rs");

    serde_codegen::expand(&src, &dst).unwrap();
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    gen_types(&out_dir);
    gen_events(&out_dir);
}
