use core::sync::atomic::{AtomicBool, Ordering};

static PERIPHERALS_TAKEN: AtomicBool = AtomicBool::new(false);

pub struct Peripherals {
    pub gpio: crate::gpio::Gpio,
    _private: (),
}

impl Peripherals {
    pub fn take() -> Option<Self> {
        if PERIPHERALS_TAKEN
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return None;
        }
        Some(unsafe { Self::new() })
    }

    /// # Safety
    ///
    /// This bypasses singleton ownership. Callers must ensure no other
    /// `Peripherals` instance is alive.
    pub unsafe fn steal() -> Self {
        PERIPHERALS_TAKEN.store(true, Ordering::Release);
        unsafe { Self::new() }
    }

    unsafe fn new() -> Self {
        Self {
            gpio: unsafe { crate::gpio::Gpio::new() },
            _private: (),
        }
    }
}
