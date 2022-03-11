#![no_std]
#![no_main]

// Define custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// Override the function name that test runner creates to
// execute since it cannot be main because of the no_main
// attribute
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

#[derive(Debug)]
#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Write to Port mapped IO to exit QEMU.
fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Custom test runner
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn dummy_assertion() {
    println!("Dummy assertion test");
    assert_eq!(1, 1);
    println!("[OK]");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Juno");

    #[cfg(test)]
    test_main();

    loop {}
}

