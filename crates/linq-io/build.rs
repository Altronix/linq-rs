use std::env;

/// NOTE linking app with libusb-1.0 seems only required on Windows
fn main() {
    match env::var("CARGO_CFG_TARGET_OS").as_ref().map(|x| &**x) {
        Ok("linux") => {
            println!("cargo:rustc-link-lib=static=usb-1.0");
        }
        Ok("windows") => {
            println!("cargo:rustc-link-lib=static=libusb-1.0");
        }
        _ => panic!("unsupported host OS!"),
    }
}
