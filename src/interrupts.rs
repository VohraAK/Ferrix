use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

// lazy static definition of IDT
lazy_static!
{
    // create a global static reference IDT to the initialised structure below...
    static ref IDT: InterruptDescriptorTable =
    {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

// initialise an IDT
pub fn init_idt() 
{
    // load the idt
    IDT.load();
}

// 
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    // print out the stack frame
    println!("[EXCEPTION]: Breakpoint\n{:#?}", stack_frame);
}



// TEST CASES
#[test_case]
fn test_breakpoint_exception()
{
    x86_64::instructions::interrupts::int3();
}