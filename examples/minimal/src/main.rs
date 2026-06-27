#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]

#[cfg(target_arch = "arm")]
use rp1_hal::prelude::*;
#[cfg(target_arch = "arm")]
use rp1_rt as _;

#[cfg(target_arch = "arm")]
#[rp1_hal::main]
fn main(mut p: Peripherals) -> ! {
    let mut pin = p.gpio.pin::<0>().into_output();

    loop {
        pin.toggle();
        for _ in 0..10_000 {
            core::hint::spin_loop();
        }
        rp1_hal::mailbox::poll();
    }
}

#[cfg(not(target_arch = "arm"))]
fn main() {}
