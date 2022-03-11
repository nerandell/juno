#![no_std]
#![no_main]

// Define custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Custom test runner
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn dummy_assertion() {
    println!("Dummy assertion test");
    assert_eq!(1, 2);
    println!("[OK]");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Juno");

    #[cfg(test)]
    test_main();

    loop {}
}

