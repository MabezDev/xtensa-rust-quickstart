// build script
// If you are targeting another Xtensa target, you will need to fill in memory.x.in and
// uncomment this build script.


// use std::env;
// use std::fs::File;
// use std::io::Write;
// use std::path::PathBuf;



fn main() {
//     // Put the linker script somewhere the linker can find it
//     let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

//     File::create(out.join("memory.x"))
//         .unwrap()
//         .write_all(include_bytes!("memory.x.in"))
//         .unwrap();

//     println!("cargo:rustc-link-search={}", out.display());
//     // Only re-run the build script when memory.x is changed,
//     // instead of when any part of the source code changes.
//     println!("cargo:rerun-if-changed=memory.x");
}