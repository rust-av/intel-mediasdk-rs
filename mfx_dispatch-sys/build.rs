extern crate libbindgen;
extern crate metadeps;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn format_write(builder: libbindgen::Builder, output: &str) {
    let s = builder.generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output)
        .unwrap();

    let _ = file.write(s.as_bytes());
}

fn common_builder() -> libbindgen::Builder {
    libbindgen::builder()
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

fn main() {
    let libs = metadeps::probe().unwrap();
/* TODO support the include paths
    let lib  = libs.get("libmfx").unwrap();
    let ref include = lib.include_paths;
*/
    let cuda_builder = common_builder().header("data/mfxvideo.h");

    // Manually fix the comment so rustdoc won't try to pick them
    format_write(cuda_builder, "src/mfxvideo.rs");
}
