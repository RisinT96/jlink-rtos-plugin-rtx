extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // JLINK API
    let jlink_header = "c_headers/jlink/RTOSPlugin.h";
    
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", jlink_header);
    
    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let jlink_bindings = bindgen::Builder::default()
    // The input header we would like to generate bindings for.
    .header(jlink_header)
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate jlink bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    jlink_bindings
    .write_to_file(out_path.join("jlink_bindings.rs"))
    .expect("Couldn't write jlink bindings!");
    
    // RTX API
    let rtx_header = "c_headers/rtx/rtx_os.h";

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", rtx_header);

    // The bindgen::Builder is the main entry point to bindgen,
    // and lets you build up options for the resulting bindings.
    let rtx_bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header(rtx_header)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate rtx bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    rtx_bindings
        .write_to_file(out_path.join("rtx_bindings.rs"))
        .expect("Couldn't write rtx bindings!");
}
