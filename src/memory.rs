use x86_64::structures::paging::{PageTable, OffsetPageTable, Mapper, Page, PhysFrame, Size4KiB, FrameAllocator};
use x86_64::{VirtAddr, PhysAddr};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub const PAGE_SIZE: usize = 4096;

// implementing an empty frame allocator
pub struct EmptyFrameAllocator;


unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator
{
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> 
    {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;

        frame

    }
}


// proper frame allocator; returns usable frames from bootloader's mem_map
pub struct BootInfoFrameAllocator
{
    mem_map: &'static MemoryMap,
    next: usize
}

impl BootInfoFrameAllocator
{
    // initialise allocator
    pub unsafe fn init(mem_map: &'static MemoryMap) -> Self
    {
        BootInfoFrameAllocator
        {
            mem_map, 
            next: 0
        }
    }

    // return iterator over usable frames in mem_map
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> 
    {
        // get usable regions from mem_map
        let regions = self.mem_map.iter();
        let usable_regions = regions.filter(|reg| reg.region_type == MemoryRegionType::Usable);

        // get address range for each usable region
        let addr_ranges = usable_regions.map(|reg| reg.range.start_addr()..reg.range.end_addr());

        // start_addr and end_addr are page-aligned
        // get every address at page boundary (aligned at PAGE_SIZE)
        let frame_addrs = addr_ranges.flat_map(|reg| reg.step_by(PAGE_SIZE));

        // create PhysFrame types
        frame_addrs.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

}


pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> 
{
    unsafe 
    {
        let level_4_table = active_level_4_table(physical_memory_offset);
        OffsetPageTable::new(level_4_table, physical_memory_offset)
    }
}


// get virtual address of L4 page table
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}


// example mapping function (to VGA base addr)
pub fn create_example_mapping(page: Page, mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) 
{
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe 
    {
        // FIXME: this is not safe, we do it only for testing
        mapper.map_to(page, frame, flags, frame_allocator)
    };

    map_to_result.expect("map_to failed").flush();
}

// pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr>
// {
//     translate_addr_inner(addr, physical_memory_offset)
// }

// fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr>
// {
//     use x86_64::structures::paging::page_table::FrameError;
//     use x86_64::registers::control::Cr3;

//     // read the active level 4 frame from the CR3 register
//     let (level_4_table_frame, _) = Cr3::read();

//     let table_indexes = [addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()];
//     let mut frame = level_4_table_frame;

//     // traverse the multi-level page table
//     for &index in &table_indexes 
//     {
//         // convert the frame into a page table reference
//         let virt = physical_memory_offset + frame.start_address().as_u64();
//         let table_ptr: *const PageTable = virt.as_ptr();
//         let table = unsafe {&*table_ptr};

//         // read the page table entry and update `frame`
//         let entry = &table[index];
//         frame = match entry.frame() 
//         {
//             Ok(frame) => frame,
//             Err(FrameError::FrameNotPresent) => return None,
//             Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
//         };
//     }

//     // calculate the physical address by adding the page offset
//     Some(frame.start_address() + u64::from(addr.page_offset()))
// }