use core::cell::RefCell;

use axdriver::AxNetDevice;
use axsync::Mutex;
use driver_net::{BaseDriverOps, EthernetAddress, NetDriverOps};
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
}

pub fn init(mut net_dev: AxNetDevice) {
    let nic = BareNic::new(net_dev);
    BARE_NIC.init_by(nic);
}

pub fn get_mac_addr() -> EthernetAddress {
    BARE_NIC.get_mac_addr()
}
