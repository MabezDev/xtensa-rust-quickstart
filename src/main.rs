#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::sync::atomic::{self, Ordering};

use panic_ramdump as _;

#[no_mangle]
fn main() {
    
}

#[doc(hidden)]
#[no_mangle]
#[link_section = ".iram0.vectors"]
pub unsafe extern "C" fn Reset() -> ! {
    // CURRENTLY DOES NOTHING
    loop {
        
    }
}

#[doc(hidden)]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // CURRENTLY DOES NOTHING
    loop {
        
    }
}



