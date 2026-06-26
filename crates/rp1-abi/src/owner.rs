pub const DEV_GPIO: u8 = 0;
pub const DEV_UART0: u8 = 1;
pub const DEV_UART1: u8 = 2;
pub const DEV_I2C0: u8 = 3;
pub const DEV_I2C1: u8 = 4;
pub const DEV_SPI0: u8 = 5;
pub const DEV_PIO0: u8 = 6;
pub const DEV_PIO1: u8 = 7;
pub const DEV_DMA: u8 = 8;
pub const DEV_TIMER: u8 = 9;

pub const fn bit(dev: u8) -> u64 {
    1u64 << dev
}
