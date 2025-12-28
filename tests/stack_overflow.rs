#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use ferrix::{QemuExitCode, qemu_close, serial_print, serial_println};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};


// no test harness
// testing stack overflow with GDT enabled

// we need to test the double fault handler
// if IDT is init normally, then the default double fault handler will be used
// since we need to run the test, we need a dummy double fault handler which returns success to QEMU
// the test IDT will therefore have a dummy fault handler, with stack index pointing to our implementation of IST[0]

lazy_static!
{
    static ref TEST_IDT: InterruptDescriptorTable = 
    {
        let mut test_idt = InterruptDescriptorTable::new();
        unsafe
        {
            test_idt.double_fault
                    .set_handler_fn(test_double_fault_handler)
                    .set_stack_index(ferrix::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        test_idt
    };
}

pub fn init_test()
{
    ferrix::gdt::init();
    TEST_IDT.load();
}


extern "x86-interrupt" fn test_double_fault_handler(_stack_frame: InterruptStackFrame, _error_code: u64) -> ! 
{
    serial_println!("[ok]");
    qemu_close(QemuExitCode::Success);
    loop {}
}



#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! 
{
    serial_print!("stack_overflow::stack_overflow...\t");

    init_test();

    stack_overflow();

    panic!("OVERFLOW TEST FAILED!");
    
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    ferrix::test_panic_handler(info)
}


#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // prevent tail recursion optimizations
}