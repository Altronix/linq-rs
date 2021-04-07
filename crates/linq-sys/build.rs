extern crate bindgen;
extern crate cmake;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new(".")
        .always_configure(true)
        .static_crt(true)
        .very_verbose(true)
        .pic(true)
        .define("LINQ_LOG_LEVEL", "TRACE")
        .define("LINQ_BUILD_APPS", "FALSE")
        .build();
    let out = dst.display();
    match env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        Ok("linux") => {
            println!("cargo:rustc-link-search=native={}/lib", out);
            println!("cargo:rustc-link-lib=static=linq");
            println!("cargo:rustc-link-lib=static=usb-1.0");
            println!("cargo:rustc-link-lib=dylib=udev");
        }
        Ok("windows") => {
            println!("cargo:rustc-link-search=native={}/lib", out);
            println!("cargo:rustc-link-lib=static=linq");
            println!("cargo:rustc-link-lib=static=libusb-1.0");
            println!("cargo:rustc-link-lib=uuid");
            println!("cargo:rustc-link-lib=iphlpapi");
            println!("cargo:rustc-link-lib=Rpcrt4");
        }
        _ => panic!("unsupported host OS!"),
    };

    let bindings = bindgen::Builder::default()
        .header("./src/linq.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    if fs::metadata("./src/bindings.rs").is_ok() {
        fs::remove_file("./src/bindings.rs").unwrap();
    }
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
