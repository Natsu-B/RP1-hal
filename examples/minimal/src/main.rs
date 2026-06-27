#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

#[cfg(target_arch = "arm")]
use rp1_hal::prelude::*;
#[cfg(target_arch = "arm")]
use rp1_rt as _;

#[cfg(target_arch = "arm")]
#[rp1_hal::main]
fn main(_p: Peripherals) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(not(target_arch = "arm"))]
fn main() {}
