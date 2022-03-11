#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use juno::{QemuExitCode, exit_qemu, DebugTest, println, serial_println};

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn test_runner(tests: &[&dyn DebugTest]) {
    serial_println!("Running tests {}", tests.len());
    for test in tests {
        test.run();
        serial_println!("Test did not panic!");
        exit_qemu(QemuExitCode::Failure);
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn test_println() {
    assert_eq!(0, 1);
}


