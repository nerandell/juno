use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use crate::gdt::DOUBLE_FAULT_STACK_INDEX;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                            .set_stack_index(DOUBLE_FAULT_STACK_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    println!("Breakpoint encountered {:#?}", frame)
}

extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame,
                                               _error_code: u64) -> ! {
    panic!("Double fault encountered {:#?}", frame);
}

pub fn init_interrupts() {
    IDT.load();
}

#[test_case]
pub fn test_interrupts() {
    x86_64::instructions::interrupts::int3();
}

