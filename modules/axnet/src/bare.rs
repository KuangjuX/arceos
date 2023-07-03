use core::cell::RefCell;

use alloc::boxed::Box;
use axdriver::AxNetDevice;
use axerrno::{AxError, AxResult};
use axsync::Mutex;
pub use driver_net::DeviceStats;
use driver_net::{BaseDriverOps, DevError, EthernetAddress, NetDriverOps, RxBuf};
use lazy_init::LazyInit;

static BARE_NIC: LazyInit<BareNic> = LazyInit::new();

pub struct BareNic {
    inner: Mutex<RefCell<AxNetDevice>>,
}

impl BareNic {
    /// Create a new `BareNic` instance.
    pub fn new(inner: AxNetDevice) -> Self {
        Self {
            inner: Mutex::new(RefCell::new(inner)),
        }
    }

    /// Get the MAC address of the NIC.
    pub fn get_mac_addr(&self) -> EthernetAddress {
        self.inner.lock().borrow().mac_address()
    }

    pub fn reset_stats(&self) {
        self.inner.lock().borrow_mut().reset_stats()
    }

    pub fn read_stats(&self) -> DeviceStats {
        self.inner.lock().borrow().read_stats()
    }

    pub fn recv(&self) -> AxResult<Box<dyn RxBuf>> {
        match self.inner.lock().borrow_mut().recv() {
            Ok(rx_buf) => Ok(rx_buf),
            Err(err) => match err {
                DevError::Again => Err(AxError::WouldBlock),
                _ => panic!("Unexpected error"),
            },
        }
    }

    pub fn send(&self, buf: &[u8]) -> AxResult {
        // match self.inner.lock().borrow_mut().send(buf) {
        //     Ok(_) => Ok(()),
        //     Err(err) => match err {
        //         DevError::Again => Err(AxError::WouldBlock),
        //         DevError::NoMemory => Err(AxError::NoMemory),
        //         _ => panic!("Unexpected error"),
        //     },
        // }
        todo!()
    }
}

pub fn init(net_dev: AxNetDevice) {
    let nic = BareNic::new(net_dev);
    BARE_NIC.init_by(nic);
}

pub fn get_mac_addr() -> EthernetAddress {
    BARE_NIC.get_mac_addr()
}

pub fn reset_stats() {
    BARE_NIC.reset_stats()
}

pub fn read_stats() -> DeviceStats {
    BARE_NIC.read_stats()
}

pub fn recv() -> AxResult<Box<dyn RxBuf>> {
    BARE_NIC.recv()
}

pub fn send(buf: &[u8]) -> AxResult {
    BARE_NIC.send(buf)
}
