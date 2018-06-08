extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use bindgen::callbacks;

#[derive(Debug)]
struct MacroTyper;

impl callbacks::ParseCallbacks for MacroTyper {
    fn int_macro(self: &Self, name: &str, value: i64) -> Option<callbacks::IntKind> {
        if name.starts_with("KERN_") || name.starts_with("KMOD_INFO_VERSION") {
            Some(callbacks::IntKind::I32)
        } else {
            None
        }
    }
}


fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    println!("cargo:rerun-if-changed=wrapper.h");
    let mut sdk_path = String::from_utf8(Command::new("xcrun")
        .args(&["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun failed")
        .stdout
    ).expect("xcrun failed harder");
    sdk_path.pop();
    let macros = Box::new(MacroTyper);
    let bindings = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("c_types")
        .rust_target(bindgen::RustTarget::Nightly)
        .blacklist_type("kmod_info_t")
        .clang_arg("--target=x86_64-apple-darwin")
        .clang_arg(format!("-I{}/System/Library/Frameworks/Kernel.framework/Versions/Current/Headers/", sdk_path))
        .parse_callbacks(macros)
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
