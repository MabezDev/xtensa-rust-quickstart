#![no_std]
#![no_main]
#![feature(asm)]

use xtensa_lx6_rt as _;

use core::panic::PanicInfo;
use esp32;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board) 
const CORE_HZ: u32 = 40_000_000;

const BLINKY_GPIO: u32 = 2; // the GPIO hooked up to the onboard LED

const RTC_CNTL_WDT_WKEY_VALUE: u32 = 0x50D83AA1;

#[no_mangle]
fn main() -> ! {
    let dp = unsafe { esp32::Peripherals::steal() };
    
    let mut gpio = dp.GPIO;
    let mut rtccntl = dp.RTCCNTL;
    disable_wdt(&mut rtccntl);

    configure_pin_as_output(&mut gpio, BLINKY_GPIO);
    loop {
        set_led(&mut gpio, BLINKY_GPIO, true);
        rtccntl.rtc_cntl_wdtfeed_reg.write(|w| w.rtc_cntl_wdt_feed().set_bit());
        // delay(CORE_HZ);
        // set_led(&mut gpio, BLINKY_GPIO, false);
        // delay(CORE_HZ);
    }
}

fn disable_wdt(rtccntl: &mut esp32::RTCCNTL) {
    rtccntl.rtc_cntl_wdtwprotect_reg.write(|w| unsafe { w.bits(RTC_CNTL_WDT_WKEY_VALUE) });
    rtccntl.rtc_cntl_wdtfeed_reg.modify(|_, w| w.rtc_cntl_wdt_feed().set_bit());
    rtccntl.rtc_cntl_wdtconfig0_reg.modify(|_, w| unsafe {
        w
        .rtc_cntl_wdt_stg0()
        .bits(0x0)
        .rtc_cntl_wdt_stg1()
        .bits(0x0)
        .rtc_cntl_wdt_stg2()
        .bits(0x0)
        .rtc_cntl_wdt_stg3()
        .bits(0x0)
        .rtc_cntl_wdt_flashboot_mod_en()
        .clear_bit()
        .rtc_cntl_wdt_en()
        .clear_bit()
    });
    rtccntl.rtc_cntl_wdtwprotect_reg.write(|w| unsafe { w.bits(0x0) });
}

pub fn set_led(reg: &mut esp32::GPIO, idx: u32, val: bool) {
    if val {
        reg.gpio_out_w1ts_reg.write(|w| unsafe { w.bits(0x1 << idx) });
    } else {
       reg.gpio_out_w1tc_reg.write(|w| unsafe { w.bits(0x1 << idx) });
    }
}

/// Configure the pin as an output
pub fn configure_pin_as_output(reg: &mut esp32::GPIO, gpio: u32){
    reg.gpio_enable_w1ts_reg.write(|w| unsafe  { w.bits(0x1 << gpio) });
    reg.gpio_func2_out_sel_cfg_reg.write(|w| unsafe { w.bits(0x100) });
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