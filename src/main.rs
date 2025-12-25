#![no_std]  // remove std
#![no_main] // remove main

use core::panic::PanicInfo;

#[panic_handler]    // function called on panic
fn panic(_info: &PanicInfo) -> !
{
    loop{}     
}

#[unsafe(no_mangle)]    // DO NOT mangle name of this function!
// entry point for the linker (looks for _start)
pub extern "C" fn _start() -> ! 
{
    loop {}
}