#![no_std]
#![no_main]

mod vga_buffer;

extern crate rlibc;

use core::panic::PanicInfo;

// Function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to fos!");

    loop {}
}
