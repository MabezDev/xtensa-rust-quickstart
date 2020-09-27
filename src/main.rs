//! NOTE: The following instructions are for building a custom Xtensa target, for the esp32 and esp8266
//! see the examples/ folder. 
//!
//! Building for a custom target:
//!   - fill out a `memory.x` file
//!   - Create a build.rs
//!   - Choose a `xtensa-lx` and `xtensa-lx-rt` cpu feature
//! 
//! Example `memory.x` (also see the ESP hals)
//! ```
//! /* 
//! Specify main memory areas
//!     These are generic ones, which will need to be filled in for your
//!     device.
//! 
//!     The espXX-hal crates will handle this for you. Only use this if you're targeting
//!     another xtensa target.
//! */
//! MEMORY
//! {
//!   vectors_seg ( RX )     : ORIGIN = 0x40080000, len =  1k /* SRAM0 */
//!   iram_seg ( RX )        : ORIGIN = 0x40080400, len = 128k-0x400 /* SRAM0 */
//!   dram_seg ( RW )        : ORIGIN = 0x3FFB0000, len = 176k
//! }
//! 
//! REGION_ALIAS("ROTEXT", iram_seg); /* these alias' link up with `xtensa-lx-rt` */
//! REGION_ALIAS("RWTEXT", iram_seg);
//! REGION_ALIAS("RODATA", dram_seg);
//! REGION_ALIAS("RWDATA", dram_seg);
//! ```
//! 
//! Example `build.rs`
//! ```
//! use std::env;
//! use std::fs::File;
//! use std::io::Write;
//! use std::path::PathBuf;
//! 
//! 
//! 
//! fn main() {
//!     // Put the linker script somewhere the linker can find it
//!     let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
//! 
//!     File::create(out.join("memory.x"))
//!         .unwrap()
//!         .write_all(include_bytes!("memory.x.in"))
//!         .unwrap();
//! 
//!     println!("cargo:rustc-link-search={}", out.display());
//!     // Only re-run the build script when memory.x is changed,
//!     // instead of when any part of the source code changes.
//!     println!("cargo:rerun-if-changed=memory.x");
//! }
//! ```

#![no_std]
#![no_main]

use xtensa_lx_rt as _;
use xtensa_lx_rt::entry;
use panic_halt as _;


#[entry]
fn main() -> ! {
    loop {}
}
