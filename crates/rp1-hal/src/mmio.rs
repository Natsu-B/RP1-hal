use core::marker::PhantomData;
use core::ptr::{read_volatile, write_volatile};

#[repr(transparent)]
pub struct Reg<T> {
    addr: *mut T,
    _not_send_sync: PhantomData<*mut ()>,
}

impl<T> Reg<T> {
    /// # Safety
    ///
    /// The caller must ensure `addr` is valid for volatile accesses of `T` and
    /// that aliasing follows the target peripheral's register semantics.
    pub const unsafe fn new(addr: usize) -> Self {
        Self {
            addr: addr as *mut T,
            _not_send_sync: PhantomData,
        }
    }
}

impl Reg<u32> {
    #[inline(always)]
    pub fn read(&self) -> u32 {
        unsafe { read_volatile(self.addr) }
    }

    #[inline(always)]
    pub fn write(&self, value: u32) {
        unsafe { write_volatile(self.addr, value) }
    }

    #[inline(always)]
    pub fn modify(&self, f: impl FnOnce(u32) -> u32) {
        let value = self.read();
        self.write(f(value));
    }
}
