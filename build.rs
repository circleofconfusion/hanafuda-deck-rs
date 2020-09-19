extern crate fs_extra;
use std::env;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  let out_dir = env::var("OUT_DIR").unwrap();
  copy("./res/img", &out_dir, &CopyOptions::new()).unwrap();
}