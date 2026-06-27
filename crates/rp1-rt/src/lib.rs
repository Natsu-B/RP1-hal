#![no_std]

#[cfg(target_arch = "arm")]
use core::panic::PanicInfo;

#[cfg(target_arch = "arm")]
unsafe extern "C" {
    fn _stack_start();
}

#[cfg(target_arch = "arm")]
unsafe extern "Rust" {
    fn rp1_entry() -> !;
}

#[cfg(target_arch = "arm")]
unsafe extern "C" {
    static mut __sbss: u8;
    static mut __ebss: u8;
}

#[cfg(target_arch = "arm")]
#[unsafe(link_section = ".vector_table")]
#[used]
pub static VECTOR_TABLE: [unsafe extern "C" fn(); 16] = [
    _stack_start,
    Reset,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
    DefaultHandler,
];

#[cfg(target_arch = "arm")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() {
    unsafe {
        zero_bss();
    }
    unsafe { rp1_entry() }
}

#[cfg(target_arch = "arm")]
unsafe fn zero_bss() {
    let mut ptr = core::ptr::addr_of_mut!(__sbss);
    let end = core::ptr::addr_of_mut!(__ebss);
    while ptr < end {
        unsafe {
            ptr.write_volatile(0);
            ptr = ptr.add(1);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultHandler() {
    loop {
        core::hint::spin_loop();
    }
}

#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
