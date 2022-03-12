use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    println!("Breakpoint encountered {:?}", frame)
}

pub fn init_interrupts() {
    IDT.load();
}

#[test_case]
pub fn test_interrupts() {
    x86_64::instructions::interrupts::int3();
}

