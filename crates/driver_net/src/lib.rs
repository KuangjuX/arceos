//! Common traits and types for network device (NIC) drivers.

#![no_std]
#![feature(const_mut_refs)]
#![feature(const_slice_from_raw_parts_mut)]

#[cfg(feature = "ixgbe")]
pub mod ixgbe;
mod net_buf;

#[macro_use]
extern crate log;
extern crate alloc;

#[doc(no_inline)]
pub use driver_common::{BaseDriverOps, DevError, DevResult, DeviceType};
pub use ixgbe_driver::DeviceStats;
use ixgbe_driver::{RxBuffer, TxBuffer};

pub use self::net_buf::{NetBuffer, NetBufferBox, NetBufferPool};

/// The ethernet address of the NIC (MAC address).
pub struct EthernetAddress(pub [u8; 6]);

impl core::fmt::Display for EthernetAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5],
        )
    }
}

/// Operations that require a network device (NIC) driver to implement.
///
/// `'a` indicates the lifetime of the network buffers.
pub trait NetDriverOps<'a>: BaseDriverOps {
    /// The ethernet address of the NIC.
    fn mac_address(&self) -> EthernetAddress;

    /// Whether can transmit packets.
    fn can_transmit(&self) -> bool;

    /// Whether can receive packets.
    fn can_receive(&self) -> bool;

    /// Size of the receive queue.
    fn rx_queue_size(&self) -> usize;

    /// Size of the transmit queue.
    fn tx_queue_size(&self) -> usize;

    // /// Fills the receive queue with buffers.
    // ///
    // /// It should be called once when the driver is initialized.
    // fn fill_rx_buffers(&mut self, buf_pool: &'a NetBufferPool) -> DevResult;

    // /// Prepares a buffer for transmitting.
    // ///
    // /// e.g., fill the header of the packet.
    // fn prepare_tx_buffer(&self, tx_buf: &mut NetBuffer, packet_len: usize) -> DevResult;

    // /// Gives back the `rx_buf` to the receive queue for later receiving.
    // ///
    // /// `rx_buf` should be the same as the one returned by
    // /// [`NetDriverOps::receive`].
    // fn recycle_rx_buffer(&mut self, rx_buf: NetBufferBox<'a>) -> DevResult;

    // /// Transmits a packet in the buffer to the network, and blocks until the
    // /// request completed.
    // ///
    // /// `tx_buf` should be initialized by [`NetDriverOps::prepare_tx_buffer`].
    // fn transmit(&mut self, tx_buf: &NetBuffer) -> DevResult;

    // /// Receives a packet from the network and store it in the [`NetBuffer`],
    // /// returns the buffer.
    // ///
    // /// Before receiving, the driver should have already populated some buffers
    // /// in the receive queue by [`NetDriverOps::fill_rx_buffers`] or
    // /// [`NetDriverOps::recycle_rx_buffer`].
    // ///
    // /// If currently no incomming packets, returns an error with type
    // /// [`DevError::Again`].
    // fn receive(&mut self) -> DevResult<NetBufferBox<'a>>;

    /// Receive a packet from the network and store in the [`Box<dyn RxBuf>`],
    /// returns the buffer.
    fn recv(&mut self) -> DevResult<RxBuf<'a>>;

    /// Send a packet to network and write it into buffer,
    /// returns [`DevResult`].
    fn send(&mut self, tx_buf: TxBuf) -> DevResult;

    /// Allocate a memory buffer of a specified size for network transmission,
    /// returns [`DevResult`]
    fn alloc_tx_buffer(&self, size: usize) -> DevResult<TxBuf<'a>>;

    /// Reset network card states.
    fn reset_stats(&mut self);

    /// Read network card states.
    fn read_stats(&self) -> DeviceStats;
}

pub struct RxBufWrapper {
    pub inner: RxBuffer,
}

// pub struct TxBufWrapper {
//     pub inner: TxBuffer,
// }

// pub trait RxBuf {
//     /// Returns packet in the buffer, not including the header.
//     fn packet(&self) -> &[u8];

//     /// Returns packet in the buffer, not including the header.
//     fn packet_mut(&mut self) -> &mut [u8];
// }

// impl RxBuf for RxBufWrapper {
//     fn packet(&self) -> &[u8] {
//         self.inner.packet()
//     }

//     fn packet_mut(&mut self) -> &mut [u8] {
//         self.inner.packet_mut()
//     }
// }

// pub trait TxBuf {
//     /// Returns allocated packet buffer data.
//     fn packet(&self) -> &[u8];

//     /// Returns allocated mutuable buffer data.
//     fn packet_mut(&mut self) -> &mut [u8];

//     fn as_any(self: Box<Self>) -> Box<dyn core::any::Any>;
// }

// impl TxBuf for TxBufWrapper {
//     /// Returns allocated packet buffer data.
//     fn packet(&self) -> &[u8] {
//         self.inner.packet()
//     }

//     /// Returns allocated mutuable buffer data.
//     fn packet_mut(&mut self) -> &mut [u8] {
//         self.inner.packet_mut()
//     }

//     fn as_any(self: Box<Self>) -> Box<dyn core::any::Any> {
//         Box::new(self)
//     }
// }

pub enum TxBuf<'a> {
    Ixgbe(TxBuffer),
    Virtio(NetBufferBox<'a>),
}

impl<'a> TxBuf<'a> {
    pub fn packet(&self) -> &[u8] {
        match self {
            Self::Ixgbe(tx_buf) => tx_buf.packet(),
            Self::Virtio(tx_buf) => tx_buf.packet(),
        }
    }

    pub fn packet_mut(&mut self) -> &mut [u8] {
        match self {
            Self::Ixgbe(tx_buf) => tx_buf.packet_mut(),
            Self::Virtio(tx_buf) => tx_buf.packet_mut(),
        }
    }
}

pub enum RxBuf<'a> {
    Ixgbe(RxBuffer),
    Virtio(NetBufferBox<'a>),
}

impl<'a> RxBuf<'a> {
    pub fn packet(&self) -> &[u8] {
        match self {
            Self::Ixgbe(rx_buf) => rx_buf.packet(),
            Self::Virtio(rx_buf) => rx_buf.packet(),
        }
    }

    pub fn packet_mut(&mut self) -> &mut [u8] {
        match self {
            Self::Ixgbe(rx_buf) => rx_buf.packet_mut(),
            Self::Virtio(rx_buf) => rx_buf.packet_mut(),
        }
    }
}
