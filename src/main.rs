#![no_std]  // remove std
#![no_main] // remove Rust-level entrypoints
// custom test framework for kernel (since the test crate will not work)
#![feature(custom_test_frameworks)]
#![test_runner(ferrix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ferrix::*;


#[unsafe(no_mangle)]    // DO NOT mangle name of this function!
// entry point for the linker (looks for _start)
pub extern "C" fn _start() -> ! 
{
    vga_buffer::splash_screen();

    // add tests entry point
    #[cfg(test)]
    test_main();
    
    loop {}
}

// panic handlers (test and not test)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    ferrix::test_panic_handler(info)
}
