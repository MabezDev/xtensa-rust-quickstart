#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board) 
const CORE_HZ: u32 = 40_000_000;

// GPIO output enable reg
const GPIO_ENABLE_W1TS_REG: u32 = 0x3FF44024;

// gpio output set register
const GPIO_OUT_W1TS_REG: u32 = 0x3FF44008;
// gpio output clear register
const GPIO_OUT_W1TC_REG : u32 = 0x3FF4400C;


const BLINKY_GPIO: u32 = 2; // the GPIO hooked up to the onboard LED

/// GPIO function mode
const GPIO_FUNCX_OUT_BASE: u32 = 0x3FF44530;
const GPIO_FUNCX_OUT_SEL_CFG: u32 = GPIO_FUNCX_OUT_BASE + (BLINKY_GPIO * 4);

// const IO_MUX_GPIO2_REG: u32 = 0x3FF49040;

#[no_mangle]
fn main() -> ! {
    // configure the pin as an output
    unsafe {
        core::ptr::write_volatile(GPIO_ENABLE_W1TS_REG as *mut _, 0x1 << BLINKY_GPIO);
        core::ptr::write_volatile(GPIO_FUNCX_OUT_SEL_CFG as *mut _, 0x100); // 0x100 makes this pin a simple gpio pin - see the technical reference
        
        // if your led pin doesn't default to GPIO you will need to set this register see technical ref for details
        // core::ptr::write_volatile(IO_MUX_GPIO2_REG as *mut _, BLINKY_GPIO); // GPIO2 function 1 is being a gpio port
    }

    loop {
        set_led(BLINKY_GPIO, true);
        delay(CORE_HZ);
        set_led(BLINKY_GPIO, false);
        delay(CORE_HZ);
    }
}

pub fn set_led(idx: u32, val: bool) {
    if val {
        unsafe {
            core::ptr::write_volatile(GPIO_OUT_W1TS_REG as *mut u32, 0x1 << idx);           
        }
    } else {
       unsafe {
            core::ptr::write_volatile(GPIO_OUT_W1TC_REG as *mut u32, 0x1 << idx); // 
        } 
    }
}

/// rough delay - as a guess divide your cycles by 20 (results will differ on opt level)
pub fn delay2(clocks: u32) {
    let dummy_var: u32 = 0;
    for _ in 0..clocks {
        unsafe { core::ptr::read_volatile(&dummy_var) };
    }
}


/// cycle accurate delay using the cycle counter register
pub fn delay(clocks: u32) {
    // NOTE: does not account for rollover
    let target = get_ccount() + clocks;
    loop {
        if get_ccount() > target {
            break;
        }
    }
}

/// Performs a special register read to read the current cycle count.
/// In the future, this can be precompiled to a archive (.a) and linked to so we don't
/// have to require the asm nightly feature - see cortex-m-rt for more details
pub fn get_ccount() -> u32 {
    let x: u32;
    unsafe { asm!("rsr.ccount a2" : "={a2}"(x) ) };
    x
}


/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}