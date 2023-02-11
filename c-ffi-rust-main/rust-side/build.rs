extern crate bindgen;

use std::path::PathBuf;

use bindgen::CargoCallbacks;

fn main() {
    let libdir_path = dunce::canonicalize(
        PathBuf::from("../c-side")
    )
        .expect("cannot canonicalize path");
    let libdir_path_str = libdir_path.to_str().expect("Path is not a valid string");

    let headers_path = libdir_path.join("mylib.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let obj_path = libdir_path.join("mylib.o");
    let lib_path = libdir_path.join("mylib.lib");

    println!("cargo:rustc-link-search={}", libdir_path_str);
    println!("cargo:rustc-link-lib=mylib");
    println!("cargo:rerun-if-changed={}", libdir_path_str);

    if !std::process::Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join("mylib.c"))
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not compile lib file");
    }

    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        // Panic if the command was not successful.
        panic!("could not emit library file");
    }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src/bindings").join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}