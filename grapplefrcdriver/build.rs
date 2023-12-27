extern crate cbindgen;
extern crate bindgen;

const SYMBOL_REGEX: &str = r"(HAL_)\w+";

use std::{env, path::PathBuf};
use std::path::Path;
use cbindgen::{Config, Builder};

fn main() {
  let target = env::var("TARGET").unwrap();
  if target.contains("linux") {
    println!("cargo:rustc-link-arg=-Wl,-soname,libgrapplefrcdriver.so");
  }

  // Import bindings from WPI libraries
  println!("cargo:rustc-link-search={}", PathBuf::from(format!("buildlibs/{}/libs", target)).canonicalize().unwrap().to_str().unwrap());
  let profile = std::env::var("PROFILE").unwrap();
  match profile.as_str() {
      "debug" => {
        println!("cargo:rustc-link-lib=wpiHald");
        println!("cargo:rustc-link-lib=wpiutild");
      },
      _ => {
        println!("cargo:rustc-link-lib=wpiHal");
        println!("cargo:rustc-link-lib=wpiutil");
      },
  }
  println!("cargo:rerun-if-changed=HALWrapper.h");

  // Some config copied from first-rust-competition https://github.com/first-rust-competition/first-rust-competition/blob/master/hal-gen/src/main.rs
  let bindings = bindgen::Builder::default()
    .header("HALWrapper.h")
    .derive_default(true)
    .clang_arg(format!("-Ibuildlibs/{}/headers", target))
    .whitelist_type(SYMBOL_REGEX)
    .whitelist_function(SYMBOL_REGEX)
    .whitelist_var(SYMBOL_REGEX)
    .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .clang_args(&[
      format!("--target={}", target)    // See: https://github.com/rust-lang/rust-bindgen/issues/1760
    ])
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  // Export symbols
  let crate_env = env::var("CARGO_MANIFEST_DIR").unwrap();
  let crate_path = Path::new(&crate_env);
  let mut config = Config::from_root_or_default(crate_path);
  config.namespaces = Some(vec!["libgrapplefrc".to_owned(), "ffi".to_owned()]);
  config.pragma_once = true;
  Builder::new().with_crate(crate_path.to_str().unwrap())
      .with_config(config)
      .with_parse_deps(true)
      .with_parse_include(&["grapple-frc-msgs"])
      .generate()
      .expect("Cannot generate header file!")
      .write_to_file("target/headers/libgrapplefrcffi.h");
}