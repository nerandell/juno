#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;
use juno::{QemuExitCode, exit_qemu, DebugTest, println, serial_println};

fn should_fail() {
    assert_eq!(0, 1);
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("Test did not panic!");
    exit_qemu(QemuExitCode::Failure);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);

    loop {}
}




