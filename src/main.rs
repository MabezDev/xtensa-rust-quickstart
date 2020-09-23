#![no_std]
#![no_main]

use xtensa_lx_rt as _;
use xtensa_lx_rt::entry;
use panic_halt as _;

/// NOTE: These are instructions for building a custom Xtensa target, for the esp32 and esp8266
/// see the examples/ folder. 
///
/// Building for a custom target:
///     - fill out the `memory.x` file
///     - Uncomment the build.rs
///     - Choose a `xtensa-lx` and `xtensa-lx-rt` cpu feature
#[entry]
fn main() -> ! {
    loop {}
}
