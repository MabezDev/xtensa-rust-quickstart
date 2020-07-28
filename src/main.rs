#![no_std]
#![no_main]

use xtensa_lx6_rt::entry;
use core::panic::PanicInfo;

#[entry]
fn main() -> ! {
    loop {
        continue;
    }
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        continue;
    }
}