#![no_std]

pub use rp1_macros::main;

pub mod prelude {
    pub use crate::Peripherals;
}

pub struct Peripherals {
    _private: (),
}

static mut PERIPHERALS_TAKEN: bool = false;

impl Peripherals {
    pub fn take() -> Option<Self> {
        unsafe {
            if PERIPHERALS_TAKEN {
                None
            } else {
                PERIPHERALS_TAKEN = true;
                Some(Self { _private: () })
            }
        }
    }

    pub unsafe fn steal() -> Self {
        unsafe {
            PERIPHERALS_TAKEN = true;
        }
        Self { _private: () }
    }
}

pub fn init() -> Option<Peripherals> {
    Peripherals::take()
}
