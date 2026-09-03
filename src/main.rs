#![no_std]
#![no_main]

pub mod pfic;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {}
}

#[riscv_rt::entry]
fn main() -> ! {
    loop {}
}
