use x86_64::{structures::paging::{mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB}, VirtAddr};
use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0x_4444_4444_0000;    // arbitrary starting address
pub const HEAP_SIZE: usize = 100 * 1024;    // 100 KiB heap

// absolutely low-effort Dummy allocator
pub struct Dummy;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// need to map the heap region before actually using it...
pub fn init_heap(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size4KiB>>
{
    // create a range of pages to be mapped...
    let page_range = 
    {
        // get heap_start and heap_end virtual addrs
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = VirtAddr::new(((HEAP_START + HEAP_SIZE) - 1) as u64);

        // get their pages...
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);

        // set page_range to be an iterator 
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    // iterate over these pages
    for page in page_range
    {
        // for each page, allocate a frame
        let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

        // map page to frame
        unsafe {mapper.map_to(page, frame, flags, frame_allocator)?.flush();}
    };

    // initialise the ALLOCATOR
    unsafe { ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE); }

    Ok(())
}
