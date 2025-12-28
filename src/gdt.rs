use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::gdt::SegmentSelector;
use lazy_static::lazy_static;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PAGE_SIZE: usize = 4096;

struct Selectors
{
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector
}

// initialise a global TSS
lazy_static!
{
    static ref TSS: TaskStateSegment = 
    {   
        // new instance of tss
        let mut tss : TaskStateSegment = TaskStateSegment::new();

        // set up IST 0 for double-fault
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = 
        {
            const STACK_SIZE: usize = PAGE_SIZE * 5;

            // static, mutable arrays for now, because mem_mgmt is not setup
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;

            // this is the stack pointer the CPU will use in event of a double fault
            stack_end 
        };

        tss
    };
}

// initialise a global GDT
lazy_static!
{
    static ref GDT: (GlobalDescriptorTable, Selectors) = 
    {
        let mut gdt: GlobalDescriptorTable = GlobalDescriptorTable::new();
        
        // initialise selectors

        // add kernel code segment
        let kcode_selector = gdt.add_entry(Descriptor::kernel_code_segment());

        // add TSS
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

        (gdt, Selectors{code_selector:kcode_selector, tss_selector})
    };
}


// init function
pub fn init()
{   
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};

    // init gdt
    GDT.0.load();

    unsafe
    {
        // reload CS in GDT tuple
        CS::set_reg(GDT.1.code_selector);

        // load TSS in GDT tuple
        load_tss(GDT.1.tss_selector);
    };

}

