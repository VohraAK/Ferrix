#![no_std]
#![no_main]

// entry point...
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[Test did not panic!]");
    qemu_close(QemuExitCode::Success);

    loop {}
}

use core::panic::PanicInfo;
use ferrix::{QemuExitCode, qemu_close, serial_println, serial_print};

#[panic_handler]
fn panic (_info: &PanicInfo) -> !
{
    serial_println!("[ok]");
    qemu_close(QemuExitCode::Success);

    loop {}
}


// ---------- TESTS ----------
fn should_fail()
{
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}