use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{println, gdt};
use lazy_static::lazy_static;

// lazy static definition of IDT
lazy_static!
{
    // create a global static reference IDT to the initialised structure below...
    static ref IDT: InterruptDescriptorTable =
    {
        let mut idt = InterruptDescriptorTable::new();
        
        // set breakpoint handler
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe
        {
            // set double fault handler
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        
        idt
    };
}

// initialise an IDT
pub fn init_idt() 
{
    // load the idt
    IDT.load();
}


// breakpoint handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    // print out the stack frame
    println!("[EXCEPTION]: Breakpoint\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! 
{
    // panic and print out the stack frame
    panic!("[EXCEPTION]: Double Fault\n{:#?}", stack_frame);
}


// TEST CASES
#[test_case]
fn test_breakpoint_exception()
{
    x86_64::instructions::interrupts::int3();
}