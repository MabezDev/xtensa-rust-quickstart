#![no_std]
#![no_main]

use xtensa_lx6_rt as _;
use xtensa_lx6_rt::entry;
use panic_halt as _;

// NOTE: building this will require you to fill out the memory.x file, see build.rs for info
#[entry]
fn main() -> ! {
    loop {}
}
