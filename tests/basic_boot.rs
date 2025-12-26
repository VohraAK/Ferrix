#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ferrix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use ferrix::println;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! 
{
    println!("TESTING BASIC BOOT");
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! 
{
    loop {}
}


// ---------- TESTS ----------
#[test_case]
fn test_println()
{
    println!("test_println output...");
}