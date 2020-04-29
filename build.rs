extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // JLINK GDB Server API
    let jlink_header = "c_headers/jlink/RTOSPlugin.h";
    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed={}", jlink_header);

    let jlink_bindings = bindgen::Builder::default()
        .header(jlink_header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate jlink bindings");

    jlink_bindings
        .write_to_file(out_path.join("jlink_bindings.rs"))
        .expect("Couldn't write jlink bindings!");
}
