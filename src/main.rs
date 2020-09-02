#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use fos::println;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    println!("Welcome to fos!");

    fos::init();

    #[cfg(test)]
    test_main();
    fos::hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    fos::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fos::test_panic_handler(info)
}
