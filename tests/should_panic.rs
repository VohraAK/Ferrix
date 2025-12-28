#![no_std]
#![no_main]

// entry point...
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("\x1b[32m[ok]\x1b[0m\n");
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
    serial_println!("\nRunning 1 test:");
    serial_print!("should_panic::should_fail ... ");
    assert_eq!(1, 1);
}