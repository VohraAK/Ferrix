#![no_std]  // remove std
#![no_main] // remove Rust-level entrypoints
// custom test framework for kernel (since the test crate will not work)
#![feature(custom_test_frameworks)]
#![test_runner(ferrix::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ferrix::*;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


#[unsafe(no_mangle)]    // DO NOT mangle name of this function!
// entry point for the linker (looks for _start)
fn kernel_main(boot_info: &'static BootInfo) -> ! 
{
    use ferrix::memory;
    use ferrix::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};

    vga_buffer::splash_screen();

    // initialise IDT
    init();

    // BREAKPOINT TEST (trigger int3)
    // x86_64::instructions::interrupts::int3();

    // let's trigger a page fault
    // unsafe
    // {
    //     *(0xfeedbeef as *mut u8) = 69;
    // }

    // triggering a stack overflow triple fault
    // fn overflow()
    // {
        // overflow();
    // }
    // overflow();

    // this creates a triple fault:
    // 1) after multiple stack pushes, the guard page is accessed -> stack overflows into guard page -> page fault
    // 2) page fault handler is accessed and tries to push interrupt stack frame -> stack is still invalid due to guard page -> double page fault
    // 3) page fault + page fault -> triple fault -> system reboot loop
    
    // need to ensure the stack stays valid in this instance -> need a known-good stack to switch to on faults
    // solved this issue by creating an Interrupt Stack Table...
    // a static stack was assigned to IST[0]; the CPU switches to this known-good stack for the double fault handler
    // GDT init with TSS entry, which stores the IST
    // solves the triple-fault-and-reboot issue

    // print deadlock
    // loop
    // {
        // use ferrix::print;
        // print!("-");
    // }

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&(boot_info.memory_map))};

    // map an unused page
    let page: Page = Page::containing_address(VirtAddr::new(0xdeadbeef as u64));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write a test string to the screen through this new mapping
    let page_ptr: *mut usize = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e); }

    // TESTS ENTRY POINT
    #[cfg(test)]
    test_main();
    
    hlt_loop();
}

// panic handlers (test and not test)
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
    ferrix::test_panic_handler(info)
}
