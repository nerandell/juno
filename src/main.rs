#![no_std]
#![no_main]

// Define custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(juno::test_runner)]
// Override the function name that test runner creates to
// execute since it cannot be main because of the no_main
// attribute
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

use core::panic::PanicInfo;


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    juno::panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Juno");

    juno::init();
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn dummy_assertion() {
    assert_eq!(1, 1);
}

