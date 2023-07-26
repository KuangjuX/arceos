use core::ptr::NonNull;

use axalloc::global_allocator;
use axhal::irq::alloc_and_register_handler;
use axhal::mem::{phys_to_virt, virt_to_phys};
use driver_net::ixgbe::{IxgbeHal, PhysAddr as IxgbePhysAddr};

use crate::AxNetDevice;

/// The number of msi-x vectors this device can have.
/// It can be set from PCI space, but we took the value from the data sheet.
pub const IXGBE_MAX_MSIX_VECTORS: usize = 64;

pub struct IxgbeHalImpl;

unsafe impl IxgbeHal for IxgbeHalImpl {
    fn dma_alloc(size: usize) -> (IxgbePhysAddr, NonNull<u8>) {
        let vaddr = if let Ok(vaddr) = global_allocator().alloc(size, 2) {
            vaddr
        } else {
            return (0, NonNull::dangling());
        };
        let paddr = virt_to_phys(vaddr.into());
        let ptr = NonNull::new(vaddr as _).unwrap();
        (paddr.as_usize(), ptr)
    }

    unsafe fn dma_dealloc(_paddr: IxgbePhysAddr, vaddr: NonNull<u8>, size: usize) -> i32 {
        global_allocator().dealloc(vaddr.as_ptr() as usize, size, 2);
        0
    }

    unsafe fn mmio_phys_to_virt(paddr: IxgbePhysAddr, _size: usize) -> NonNull<u8> {
        NonNull::new(phys_to_virt(paddr.into()).as_mut_ptr()).unwrap()
    }

    unsafe fn mmio_virt_to_phys(vaddr: NonNull<u8>, _size: usize) -> IxgbePhysAddr {
        virt_to_phys((vaddr.as_ptr() as usize).into()).into()
    }

    fn wait_until(duration: core::time::Duration) -> Result<(), &'static str> {
        axhal::time::busy_wait_until(duration);
        Ok(())
    }
}

pub fn pci_probe_ixgbe(
    pci_root: &mut driver_pci::PciRoot,
    dev_func: driver_pci::DeviceFunction,
    dev_info: &driver_pci::DeviceFunctionInfo,
) -> Option<AxNetDevice> {
    use driver_net::ixgbe::{IxgbeNic, INTEL_82599, INTEL_VEND};
    if dev_info.vendor_id == INTEL_VEND && dev_info.device_id == INTEL_82599 {
        // Intel 10Gb Network
        info!("ixgbe PCI device found at {:?}", dev_func);

        // Initialize the device
        // These can be changed according to the requirments specified in the ixgbe init function.
        const QN: u16 = 1;
        const QS: usize = 1024;
        let bar_info = pci_root.bar_info(dev_func, 0).unwrap();
        match bar_info {
            driver_pci::BarInfo::Memory { address, size, .. } => {
                // map the msi-x vector table to an address found from the pci space.
                let mut vector_table = pci_root
                    .pci_mem_map_msix(
                        dev_func,
                        IXGBE_MAX_MSIX_VECTORS,
                        |phys_addr: usize| -> usize { phys_to_virt(phys_addr.into()).into() },
                    )
                    .ok()?;
                // enable msi-x interrupts if required and return the assigned interrupt number.
                pci_root.pci_enable_msix(dev_func).ok()?;

                // ixgbe enable initialize & enable msi-x interrupts.
                let ixgbe_nic = IxgbeNic::<IxgbeHalImpl, QS, QN>::init(
                    phys_to_virt((address as usize).into()).into(),
                    size as usize,
                )
                .expect("failed to initialize ixgbe device");

                // Initialize msi vectors
                let msi_int_num =
                    alloc_and_register_handler(|| todo!("ixgbe: msi interrupt handler"))?;
                vector_table[0].init(0, msi_int_num);
                return Some(ixgbe_nic);
            }
            driver_pci::BarInfo::IO { .. } => {
                error!("ixgbe: BAR0 is of I/O type");
                return None;
            }
        }
    }
    None
}
