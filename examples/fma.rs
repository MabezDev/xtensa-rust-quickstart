#![no_std]
#![no_main]

#![feature(core_intrinsics)]

use xtensa_lx_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let x = CustomF32(1.0);
    let a = lerp(x, CustomF32(1.0), CustomF32(2.0));

    loop {
        
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CustomF32(f32);

impl core::ops::Neg for CustomF32 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        CustomF32(self.0.neg())
    }
}

impl CustomF32 {
    pub fn mul_add(&self, a: CustomF32, b: CustomF32) -> CustomF32 {
        CustomF32(unsafe { core::intrinsics::fmaf32(self.0, a.0, b.0) })
    }
}

pub fn lerp(s: CustomF32, start: CustomF32, end: CustomF32) -> CustomF32 {
    // consistent
    if start == end {
        start

    // exact/monotonic
    } else {
        s.mul_add(end, (-s).mul_add(start, start))
    }
}
