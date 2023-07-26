//! MSI-X support for PCI devices.

use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::volatile::{volread, volwrite, Volatile};

/// A constant which indicates the region that is reserved for interrupt messages
const MSIX_INTERRUPT_REGION: u32 = 0xFEE << 20;
/// The location in the lower address register where the destination CPU ID is written
const MSIX_DEST_ID_SHIFT: u32 = 12;
/// The bits in the lower address register that need to be cleared and set
const MSIX_ADDRESS_BITS: u32 = 0xFFFF_FFF0;
/// Clear the vector control field to unmask the interrupt
const MSIX_UNMASK_INT: u32 = 0;

/// A memory-mapped array of [`MsixVectorEntry`]
pub struct MsixVectorTable {
    entries: Box<[MsixVectorEntry]>,
}

impl Deref for MsixVectorTable {
    type Target = [MsixVectorEntry];
    fn deref(&self) -> &Self::Target {
        &self.entries
    }
}

impl DerefMut for MsixVectorTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.entries
    }
}

impl MsixVectorTable {
    /// Creates a new MSI-X vector table.
    pub fn new(addr: usize, max_vectors: usize) -> Self {
        let mut entries = Vec::with_capacity(max_vectors);

        for i in 0..max_vectors {
            let entry_addr = addr + i * core::mem::size_of::<MsixVectorEntryMmio>();
            let entry = NonNull::new(entry_addr as *mut MsixVectorEntryMmio).unwrap();
            let entry = MsixVectorEntry { entry };
            entries.push(entry);
        }

        let entries = entries.into_boxed_slice();
        Self { entries }
    }
}

#[repr(C)]
struct MsixVectorEntryMmio {
    /// The lower portion of the address for the memory write transaction.
    /// This part contains the CPU ID which the interrupt will be redirected to.
    msg_lower_addr: Volatile<u32>,
    /// The upper portion of the address for the memory write transaction.
    msg_upper_addr: Volatile<u32>,
    /// The data portion of the msi vector which contains the interrupt number.
    msg_data: Volatile<u32>,
    /// The control portion which contains the interrupt mask bit.
    vector_control: Volatile<u32>,
}

/// A single Message Signaled Interrupt entry.
///
/// This entry contains the interrupt's IRQ vector number
/// and the CPU to which the interrupt will be delivered.
pub struct MsixVectorEntry {
    entry: NonNull<MsixVectorEntryMmio>,
}

impl MsixVectorEntry {
    /// Sets interrupt destination & number for this entry and makes sure the
    /// interrupt is unmasked (PCI Controller side).
    pub fn init(&mut self, cpu_id: usize, int_num: u8) {
        unsafe {
            // unmask the interrupt
            volwrite!(self.entry, vector_control, MSIX_UNMASK_INT);
            let lower_addr = volread!(self.entry, msg_lower_addr);

            // set the CPU to which this interrupt will be delivered.
            let dest_id = (cpu_id as u32) << MSIX_DEST_ID_SHIFT;
            let address = lower_addr & !MSIX_ADDRESS_BITS;
            volwrite!(
                self.entry,
                msg_lower_addr,
                address | MSIX_INTERRUPT_REGION | dest_id
            );

            // write interrupt number
            volwrite!(self.entry, msg_data, int_num as u32);

            if false {
                let control = volread!(self.entry, vector_control);
                log::debug!(
                    "Created MSI vector: control: {}, CPU: {}, int: {}",
                    control,
                    cpu_id,
                    int_num
                );
            }
        }
    }
}
