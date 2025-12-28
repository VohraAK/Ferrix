use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::{println, gdt, print};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
pub const PS2_KBD_DATA_PORT: u16 = 0x60;

// def PICs
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});

// def interrupt index enum
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex 
{
    Timer = PIC_1_OFFSET,   // 32
    Keyboard,               // 33
}

impl InterruptIndex 
{
    fn as_u8(self) -> u8 
    {
        self as u8
    }

    fn as_usize(self) -> usize 
    {
        usize::from(self.as_u8())
    }
}



// lazy static definition of IDT
lazy_static!
{
    // create a global static reference IDT to the initialised structure below...
    static ref IDT: InterruptDescriptorTable =
    {
        let mut idt = InterruptDescriptorTable::new();
        
        // set breakpoint handler
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        unsafe
        {
            // set double fault handler
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // set timer interrupt handler
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);

        // set keyboard interrupt handler
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

// initialise an IDT
pub fn init_idt() 
{
    // load the idt
    IDT.load();
}

// initialise PIC
pub fn init_pics()
{
    unsafe 
    {
        PICS.lock().initialize();
    }
}

// enable interrupts
pub fn enable_interrupts()
{
    x86_64::instructions::interrupts::enable();
}



// ---------- HANDLERS ----------
// breakpoint handler
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame)
{
    // print out the stack frame
    println!("[EXCEPTION]: Breakpoint\n{:#?}", stack_frame);
}

// double fault handler
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! 
{
    // panic and print out the stack frame
    panic!("[EXCEPTION]: Double Fault\n{:#?}", stack_frame);
}

// timer interrupt handler
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame)
{
    // print!(".");

    // send EOI
    unsafe
    {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame)
{
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;


    // kbd instance
    lazy_static! 
    {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    // read scancode
    let scancode: u8 = unsafe { port.read() };

    // interpret keyevent
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) 
    {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key 
            {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }


    // send EOI
    unsafe 
    {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}


// --------- TEST CASES ----------
#[test_case]
fn test_breakpoint_exception()
{
    x86_64::instructions::interrupts::int3();
}