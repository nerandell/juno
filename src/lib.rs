#![no_std]
#![no_main]

#![cfg_attr(test, no_main)]
// Define custom test framework
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// Override the function name that test runner creates to
// execute since it cannot be main because of the no_main
// attribute
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

pub mod serial;
pub mod vga_buffer;

#[derive(Debug)]
#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub trait DebugTest {
    fn run(&self);
}

impl<T> DebugTest for T where T: Fn() {
    fn run(&self) {
        serial_print!("Running test {} ", core::any::type_name::<T>());
        self();
        serial_println!("[OK]")
    }
}

// Write to Port mapped IO to exit QEMU.
fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn panic_handler(info: &PanicInfo) -> ! {
    serial_println!("Test failed!");
    serial_println!("Error info: {:?}", info);
    exit_qemu(QemuExitCode::Failure);
    loop {}
}

// Custom test runner
pub fn test_runner(tests: &[&dyn DebugTest]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    panic_handler(info);
}



