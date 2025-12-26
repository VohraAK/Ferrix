#![no_std]

#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;

const QEMU_IOBASE: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode
{
    Success = 0x10,
    Failure = 0x11
}                   // fun fact: Rust stores enums as tagged unions by default, we need to convert it into a u32 C-sized enum


// this function comands the QEMU instance to close itself
pub fn qemu_close(exit_code: QemuExitCode)
{
    use x86_64::instructions::port::Port;

    unsafe
    {
        let mut port = Port::new(QEMU_IOBASE);
        port.write(exit_code as u32);
    }
}


pub trait Testable 
{
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) 
    {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(_tests: &[&dyn Testable]) { // new
    serial_println!("Running {} tests", _tests.len());

    for test in _tests 
    {
        test.run();
    }

    qemu_close(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! 
{
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);

    qemu_close(QemuExitCode::Failure);
    loop {}
}



// this is the entry point for cargo test
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! 
{
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    test_panic_handler(info)
}