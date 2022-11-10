use std::{path::PathBuf, env};

use pkg_config;

fn main() {
    //pkg_config::probe_library("libdlt").unwrap();

    println!("cargo:rustc-link-lib=dlt");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .detect_include_paths(true)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("dlt_.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

        std::fs::copy(PathBuf::from(out_path), PathBuf::from("src/bindings.rs")).unwrap();
}