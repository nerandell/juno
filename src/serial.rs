use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt::{Arguments, Write};

// To enable directing output to host machine IO
lazy_static! {
    pub static ref SERIAL: Mutex<SerialPort> = {
        let mut serial = unsafe {SerialPort::new(0x3F8) };
        serial.init();
        Mutex::new(serial)
    };
}

pub fn _print(args: Arguments) {
    SERIAL.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt: expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt: expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    }
}
