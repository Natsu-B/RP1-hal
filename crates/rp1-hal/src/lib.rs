#![no_std]

pub use rp1_macros::main;

pub mod addr;
pub mod gpio;
pub mod mailbox;
pub mod mmio;
pub mod owner;
pub mod peripherals;
pub mod prelude;

pub use peripherals::Peripherals;

pub fn init() -> Option<Peripherals> {
    Peripherals::take()
}
