use driver_common::BaseDriverOps;
use driver_common::DevError;
use driver_common::DevResult;
pub use ixgbe_driver::BufferDirection;
pub use ixgbe_driver::IxgbeDevice;
pub use ixgbe_driver::IxgbeHal;
use ixgbe_driver::NicDevice;
pub use ixgbe_driver::PhysAddr;
pub use ixgbe_driver::{INTEL_82599, INTEL_VEND};

use crate::NetDriverOps;

pub struct IxgbeNic<H: IxgbeHal> {
    inner: IxgbeDevice<H>,
}

impl<H: IxgbeHal> IxgbeNic<H> {
    pub fn init(
        base: usize,
        len: usize,
        num_rx_queues: u16,
        num_tx_queues: u16,
    ) -> DevResult<Self> {
        let inner =
            IxgbeDevice::<H>::init(base, len, num_rx_queues, num_tx_queues).map_err(|err| {
                error!("Failed to initialize ixgbe device: {:?}", err);
                DevError::BadState
            })?;
        Ok(Self { inner })
    }
}

impl<H: IxgbeHal> BaseDriverOps for IxgbeNic<H> {
    fn device_name(&self) -> &str {
        self.inner.get_driver_name()
    }

    fn device_type(&self) -> driver_common::DeviceType {
        driver_common::DeviceType::Net
    }
}

impl<'a, H: IxgbeHal> NetDriverOps<'a> for IxgbeNic<H> {
    fn mac_address(&self) -> crate::EthernetAddress {
        crate::EthernetAddress(self.inner.get_mac_addr())
    }

    fn tx_queue_size(&self) -> usize {
        self.inner.num_tx_queues() as usize
    }

    fn rx_queue_size(&self) -> usize {
        self.inner.num_rx_queues() as usize
    }

    fn can_receive(&self) -> bool {
        false
    }

    fn can_transmit(&self) -> bool {
        false
    }

    fn fill_rx_buffers(&mut self, buf_pool: &'a crate::NetBufferPool) -> driver_common::DevResult {
        todo!()
    }

    fn prepare_tx_buffer(
        &self,
        tx_buf: &mut crate::NetBuffer,
        packet_len: usize,
    ) -> driver_common::DevResult {
        todo!()
    }

    fn receive(&mut self) -> driver_common::DevResult<crate::NetBufferBox<'a>> {
        todo!()
    }

    fn recycle_rx_buffer(&mut self, rx_buf: crate::NetBufferBox<'a>) -> driver_common::DevResult {
        todo!()
    }

    fn transmit(&mut self, tx_buf: &crate::NetBuffer) -> driver_common::DevResult {
        todo!()
    }
}
