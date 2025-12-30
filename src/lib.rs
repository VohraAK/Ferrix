#![no_std]
#![feature(abi_x86_interrupt)]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// =========================
// Imports and Modules
// =========================
use core::panic::PanicInfo;
pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;

extern crate alloc;

#[cfg(test)]
use bootloader::{BootInfo, entry_point};
#[cfg(test)]
entry_point!(test_kernel_main);

// =========================
// Constants and Enums
// =========================
const QEMU_IOBASE: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode 
{
    Success = 0x10,
    Failure = 0x11,
} // Rust stores enums as tagged unions by default, we need to convert it into a u32 C-sized enum

// =========================
// Lib Functions
// =========================
pub fn qemu_close(exit_code: QemuExitCode) 
{
    use x86_64::instructions::port::Port;
    unsafe 
    {
        let mut port = Port::new(QEMU_IOBASE);
        port.write(exit_code as u32);
    }
}

// hlt
pub fn hlt_loop() -> !
{
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init()
{
    // init PICs
    interrupts::init_pics();

    // init idt
    interrupts::init_idt();

    // init gdt (with tss)
    gdt::init();

    // enable interrupts
    interrupts::enable_interrupts();

    println!("   [OK] Interrupts enabled")

}

// =========================
// Test Framework
// =========================
pub trait Testable {
    fn run(&self, width: usize);
    fn name(&self) -> &'static str;
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self, width: usize) {
        let name = core::any::type_name::<T>();
        serial_print!("{:<width$} ... ", name, width = width);
        self();
        serial_println!("\x1b[32m[ok]\x1b[0m");
    }
    fn name(&self) -> &'static str {
        core::any::type_name::<T>()
    }
}

pub fn test_runner(_tests: &[&dyn Testable]) 
{
    if _tests.len() <= 0
    {
        serial_print!("\n[No tests detected]\n\n");
        qemu_close(QemuExitCode::Success);
    }

    serial_println!("\nRunning {} tests", _tests.len());
    // Find the maximum test name length
    let max_width = _tests.iter().map(|t| t.name().len()).max().unwrap_or(0);
    for test in _tests {
        test.run(max_width);
    }
    serial_print!("\n");
    qemu_close(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! 
{
    serial_println!("\x1b[31m[failed]\x1b[0m\n");
    serial_println!("Error: {}\n", info);
    qemu_close(QemuExitCode::Failure);
    hlt_loop();
}

// =========================
// Test Entry Points
// =========================


#[cfg(test)]
#[unsafe(no_mangle)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! 
{
    // intialise IDT
    init();

    test_main();
    
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    test_panic_handler(info)
}