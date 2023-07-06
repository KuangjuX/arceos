//! Ixgbe NIC implementation in arceos.

// use alloc::boxed::Box;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
// use alloc::vec::Vec;
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
use ixgbe_driver::RxBuffer;
use ixgbe_driver::TxBuffer;
pub use ixgbe_driver::{INTEL_82599, INTEL_VEND};

use crate::NetDriverOps;
use crate::RxBuf;
use crate::TxBuf;

const RECV_BATCH_SIZE: usize = 64;
// const SEND_BATCH_SIZE: usize = 8;

/// The ixgbe NIC device driver.
///
/// `QS` is the ixgbe queue size.
pub struct IxgbeNic<H: IxgbeHal, const QS: u16> {
    inner: IxgbeDevice<H>,
    mempool: Arc<MemPool>,
    rx_buffer_queue: VecDeque<RxBuffer>,
    // tx_buffer_queue: VecDeque<TxBuffer>,
}

impl<H: IxgbeHal, const QS: u16> IxgbeNic<H, QS> {
    /// Creates a net ixgbe NIC instance and initialize, or returns a error if
    /// any step fails.
    pub fn init(base: usize, len: usize) -> DevResult<Self> {
        let inner = IxgbeDevice::<H>::init(base, len, QS, QS).map_err(|err| {
            error!("Failed to initialize ixgbe device: {:?}", err);
            DevError::BadState
        })?;

        // TODO: Customizable Memory Pool member.
        let mempool = MemPool::allocate::<H>(2048, 4096).unwrap();
        let rx_buffer_queue = VecDeque::new();
        // let tx_buffer_queue = VecDeque::new();
        Ok(Self {
            inner,
            mempool,
            rx_buffer_queue,
            // tx_buffer_queue,
        })
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

    fn fill_rx_buffers(&mut self, _buf_pool: &'a crate::NetBufferPool) -> DevResult {
        Ok(())
    }

    fn prepare_tx_buffer(&self, _tx_buf: &mut crate::NetBuffer, _packet_len: usize) -> DevResult {
        Ok(())
    }

    fn recycle_rx_buffer(&mut self, _rx_buf: crate::NetBufferBox<'a>) -> DevResult {
        Ok(())
    }

    fn receive(&mut self) -> DevResult<RxBuf<'a>> {
        if !self.rx_buffer_queue.is_empty() {
            let rx_buf = self.rx_buffer_queue.pop_front().unwrap();
            Ok(RxBuf::Ixgbe(rx_buf))
        } else {
            match self.inner.receive_packets(0, RECV_BATCH_SIZE) {
                Ok(mut rx_bufs) => {
                    while let Some(rx_buf) = rx_bufs.pop() {
                        self.rx_buffer_queue.push_back(rx_buf)
                    }
                    Ok(RxBuf::Ixgbe(self.rx_buffer_queue.pop_front().unwrap()))
                }
                Err(_) => Err(DevError::Again),
            }
        }
    }

    fn transmit(&mut self, buf: TxBuf) -> DevResult {
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
        // match buf {
        //     TxBuf::Ixgbe(tx_buf) => {
        //         self.tx_buffer_queue.push_back(tx_buf);
        //         if self.tx_buffer_queue.len() >= SEND_BATCH_SIZE {
        //             let mut tx_bufs = VecDeque::new();
        //             for _ in 0..SEND_BATCH_SIZE {
        //                 // TODO: recover tx_buf when transmiting fails.
        //                 tx_bufs.push_back(self.tx_buffer_queue.pop_front().unwrap());
        //             }
        //             match self.inner.send_packets(0, &mut tx_bufs) {
        //                 Ok(_) => {
        //                     while !tx_bufs.is_empty() {
        //                         self.tx_buffer_queue.push_front(tx_bufs.pop_back().unwrap());
        //                     }
        //                     return Ok(());
        //                 }
        //                 Err(err) => match err {
        //                     IxgbeError::QueueFull => return Err(DevError::Again),
        //                     _ => panic!("Unexpected err: {:?}", err),
        //                 },
        //             }
        //         }
        //         Ok(())
        //     }
        //     TxBuf::Virtio(_) => Err(DevError::BadState),
        // }
    }

    fn alloc_tx_buffer(&self, size: usize) -> DevResult<TxBuf<'a>> {
        let tx_buf = TxBuffer::alloc(&self.mempool, size).map_err(|_| DevError::NoMemory)?;
        Ok(TxBuf::Ixgbe(tx_buf))
    }
}

unsafe impl<H: IxgbeHal, const QS: u16> Sync for IxgbeNic<H, QS> {}
unsafe impl<H: IxgbeHal, const QS: u16> Send for IxgbeNic<H, QS> {}
