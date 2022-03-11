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
mod serial;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

#[derive(Debug)]
#[repr(u32)]
enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("Test failed!");
    serial_println!("Error info: {:?}", info);
    exit_qemu(QemuExitCode::Failure);
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
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn dummy_assertion() {
    serial_println!("Dummy assertion test");
    assert_eq!(1, 2);
    serial_println!("[OK]");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Juno");

    #[cfg(test)]
    test_main();

    loop {}
}

