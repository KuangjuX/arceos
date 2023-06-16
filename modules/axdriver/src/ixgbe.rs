use axalloc::global_allocator;
use axhal::mem::{phys_to_virt, virt_to_phys};
use core::ptr::NonNull;
use driver_net::ixgbe::{BufferDirection, IxgbeHal, PhysAddr};

pub struct IxgbehalImpl;

unsafe impl IxgbeHal for IxgbehalImpl {
    fn dma_alloc(pages: usize, _direction: BufferDirection) -> (PhysAddr, NonNull<u8>) {
        let vaddr = if let Ok(vaddr) = global_allocator().alloc_pages(pages, 0x1000) {
            vaddr
        } else {
            return (0, NonNull::dangling());
        };
        let paddr = virt_to_phys(vaddr.into());
        let ptr = NonNull::new(vaddr as _).unwrap();
        (paddr.as_usize(), ptr)
    }

    unsafe fn dma_dealloc(_paddr: PhysAddr, vaddr: NonNull<u8>, pages: usize) -> i32 {
        global_allocator().dealloc_pages(vaddr.as_ptr() as usize, pages);
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: PhysAddr, size: usize) -> NonNull<u8> {
        NonNull::new(phys_to_virt(paddr.into()).as_mut_ptr()).unwrap()
    }

    unsafe fn mmio_virt_to_phys(vaddr: NonNull<u8>, size: usize) -> PhysAddr {
        virt_to_phys((vaddr.as_ptr() as usize).into()).into()
    }

    fn get_tsc_frequency() -> u64 {
        axhal::time::TIMER_FREQUENCY as u64
    }

    fn wait_until(duration: core::time::Duration) -> Result<(), &'static str> {
        axhal::time::busy_wait_until(duration);
        Ok(())
    }
}
