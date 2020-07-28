#![no_std]
#![no_main]

use xtensa_lx6_rt::entry;
use xtensa_lx6::timer::delay;

use core::panic::PanicInfo;
use esp32_hal::target as esp32;
use esp32_hal::gpio::GpioExt;
use esp32_hal::hal::digital::v2::OutputPin;

/// The default clock source is the onboard crystal
/// In most cases 40mhz (but can be as low as 2mhz depending on the board) 
const CORE_HZ: u32 = 40_000_000;

const WDT_WKEY_VALUE: u32 = 0x50D83AA1;

#[entry]
fn main() -> ! {
    let dp = unsafe { esp32::Peripherals::steal() };
    
    let mut rtccntl = dp.RTCCNTL;
    let mut timg0 = dp.TIMG0;
    let mut timg1 = dp.TIMG1;

    // (https://github.com/espressif/openocd-esp32/blob/97ba3a6bb9eaa898d91df923bbedddfeaaaf28c9/src/target/esp32.c#L431)
    // openocd disables the wdt's on halt
    // we will do it manually on startup
    disable_timg_wdts(&mut timg0, &mut timg1);
    disable_rtc_wdt(&mut rtccntl);

    let gpios = dp.GPIO.split();
    let mut blinky = gpios.gpio2.into_push_pull_output();

    loop {
        blinky.set_high().unwrap();
        delay(CORE_HZ);
        blinky.set_low().unwrap();
        delay(CORE_HZ);
    }
}

fn disable_rtc_wdt(rtccntl: &mut esp32::RTCCNTL) {
    /* Disables the RTCWDT */
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(WDT_WKEY_VALUE) });
    rtccntl.wdtconfig0.modify(|_, w| unsafe {
        w
        .wdt_stg0()
        .bits(0x0)
        .wdt_stg1()
        .bits(0x0)
        .wdt_stg2()
        .bits(0x0)
        .wdt_stg3()
        .bits(0x0)
        .wdt_flashboot_mod_en()
        .clear_bit()
        .wdt_en()
        .clear_bit()
    });
    rtccntl.wdtwprotect.write(|w| unsafe { w.bits(0x0) });
}

fn disable_timg_wdts(timg0: &mut esp32::TIMG0, timg1: &mut esp32::TIMG1) {
    timg0.wdtwprotect.write(|w| unsafe { w.bits(WDT_WKEY_VALUE)});
    timg1.wdtwprotect.write(|w| unsafe { w.bits(WDT_WKEY_VALUE)});

    timg0.wdtconfig0.write(|w| unsafe{ w.bits(0x0)});
    timg1.wdtconfig0.write(|w| unsafe{ w.bits(0x0)});
}

/// Basic panic handler - just loops
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        continue;
    }
}