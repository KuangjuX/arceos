use alloc::sync::Arc;
use driver_common::BaseDriverOps;
use driver_common::DevError;
use driver_common::DevResult;
pub use ixgbe_driver::BufferDirection;
pub use ixgbe_driver::DeviceStats;
pub use ixgbe_driver::IxgbeDevice;
use ixgbe_driver::IxgbeError;
pub use ixgbe_driver::IxgbeHal;
use ixgbe_driver::MemPool;
use ixgbe_driver::NicDevice;
pub use ixgbe_driver::PhysAddr;
use ixgbe_driver::TxBuffer;
pub use ixgbe_driver::{INTEL_82599, INTEL_VEND};

use crate::NetDriverOps;
use crate::RxBuf;
use crate::TxBuf;
use alloc::boxed::Box;

pub struct IxgbeNic<H: IxgbeHal, const QS: u16> {
    inner: IxgbeDevice<H>,
    mempool: Arc<MemPool>,
}

impl<H: IxgbeHal, const QS: u16> IxgbeNic<H, QS> {
    pub fn init(base: usize, len: usize) -> DevResult<Self> {
        let inner = IxgbeDevice::<H>::init(base, len, QS, QS).map_err(|err| {
            error!("Failed to initialize ixgbe device: {:?}", err);
            DevError::BadState
        })?;

        // TODO: Customizable Memory Pool member.
        let mempool = MemPool::allocate::<H>(2048, 4096).unwrap();
        Ok(Self { inner, mempool })
    }
}

impl<H: IxgbeHal, const QS: u16> BaseDriverOps for IxgbeNic<H, QS> {
    fn device_name(&self) -> &str {
        self.inner.get_driver_name()
    }

    fn device_type(&self) -> driver_common::DeviceType {
        driver_common::DeviceType::Net
    }
}

impl<'a, H: IxgbeHal + 'static, const QS: u16> NetDriverOps<'a> for IxgbeNic<H, QS> {
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
        self.inner.can_receive(0).unwrap()
    }

    fn can_transmit(&self) -> bool {
        self.inner.can_send(0).unwrap()
    }

    // fn fill_rx_buffers(&mut self, buf_pool: &'a crate::NetBufferPool) -> DevResult {
    //     todo!()
    // }

    // fn prepare_tx_buffer(&self, tx_buf: &mut crate::NetBuffer, packet_len: usize) -> DevResult {
    //     todo!()
    // }

    // fn receive(&mut self) -> DevResult<crate::NetBufferBox<'a>> {
    //     todo!()
    // }

    // fn recycle_rx_buffer(&mut self, rx_buf: crate::NetBufferBox<'a>) -> DevResult {
    //     todo!()
    // }

    // fn transmit(&mut self, tx_buf: &crate::NetBuffer) -> DevResult {
    //     todo!()
    // }

    fn recv(&mut self) -> DevResult<RxBuf<'a>> {
        // TODO: configurable param
        match self.inner.receive(0) {
            Ok(rx_buf) => Ok(RxBuf::Ixgbe(rx_buf)),
            Err(err) => match err {
                IxgbeError::NotReady => Err(DevError::Again),
                _ => panic!("Unexpected err: {:?}", err),
            },
        }
    }

    fn send(&mut self, buf: TxBuf) -> DevResult {
        // let len = buf.len();
        // if let Ok(mut tx_buf) = TxBuffer::alloc(&self.mempool, len) {
        //     // TODO: zero copy
        //     unsafe {
        //         core::ptr::copy(buf.as_ptr(), tx_buf.packet_mut().as_mut_ptr(), len);
        //     }
        //     match self.inner.send(0, tx_buf) {
        //         Ok(_) => return Ok(()),
        //         Err(err) => match err {
        //             IxgbeError::QueueFull => return Err(DevError::Again),
        //             _ => panic!("Unexpected err: {:?}", err),
        //         },
        //     }
        // }
        // Err(DevError::NoMemory)

        match buf {
            TxBuf::Ixgbe(tx_buf) => match self.inner.send(0, tx_buf) {
                Ok(_) => Ok(()),
                Err(err) => match err {
                    IxgbeError::QueueFull => Err(DevError::Again),
                    _ => panic!("Unexpected err: {:?}", err),
                },
            },
            TxBuf::Virtio(_) => Err(DevError::BadState),
        }
    }

    fn alloc_tx_buffer(&self, size: usize) -> DevResult<TxBuf<'a>> {
        let tx_buf = TxBuffer::alloc(&self.mempool, size).map_err(|_| DevError::NoMemory)?;
        Ok(TxBuf::Ixgbe(tx_buf))
    }

    fn reset_stats(&mut self) {
        self.inner.reset_stats()
    }

    fn read_stats(&self) -> ixgbe_driver::DeviceStats {
        let mut stats = DeviceStats::default();
        self.inner.read_stats(&mut stats);
        stats
    }
}
