#![no_std]  // remove std
#![no_main] // remove Rust-level entrypoints

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]    // function called on panic
fn panic(_info: &PanicInfo) -> !
{
    println!("{}", _info);
    loop{}     
}

#[unsafe(no_mangle)]    // DO NOT mangle name of this function!
// entry point for the linker (looks for _start)

pub extern "C" fn _start() -> ! 
{
    vga_buffer::splash_screen();
    
    loop {}
}