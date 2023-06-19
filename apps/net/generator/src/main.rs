#![no_std]
#![no_main]

use alloc::collections::VecDeque;

#[macro_use]
extern crate libax;
extern crate alloc;

// number of packets sent simultaneously by our driver
const BATCH_SIZE: usize = 32;
// number of packets in our mempool
const NUM_PACKETS: usize = 2048;
// size of our packets
const PACKET_SIZE: usize = 60;

#[no_mangle]
fn main() {
    libax::info!("Ixgbe generator test");
    #[rustfmt::skip]
    let mut pkt_data = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06,         // dst MAC
        0x10, 0x10, 0x10, 0x10, 0x10, 0x10,         // src MAC
        0x08, 0x00,                                 // ether type: IPv4
        0x45, 0x00,                                 // Version, IHL, TOS
        ((PACKET_SIZE - 14) >> 8) as u8,            // ip len excluding ethernet, high byte
        ((PACKET_SIZE - 14) & 0xFF) as u8,          // ip len excluding ethernet, low byte
        0x00, 0x00, 0x00, 0x00,                     // id, flags, fragmentation
        0x40, 0x11, 0x00, 0x00,                     // TTL (64), protocol (UDP), checksum
        0x0A, 0x00, 0x00, 0x01,                     // src ip (10.0.0.1)
        0x0A, 0x00, 0x00, 0x02,                     // dst ip (10.0.0.2)
        0x00, 0x2A, 0x05, 0x39,                     // src and dst ports (42 -> 1337)
        ((PACKET_SIZE - 20 - 14) >> 8) as u8,       // udp len excluding ip & ethernet, high byte
        ((PACKET_SIZE - 20 - 14) & 0xFF) as u8,     // udp len excluding ip & ethernet, low byte
        0x00, 0x00,                                 // udp checksum, optional
        b'i', b'x', b'g', b'b', b'e'                // payload
        // rest of the payload is zero-filled because mempools guarantee empty bufs
    ];

    let mac_addr = libax::net::get_mac_addr();
    libax::info!("mac address: {}", mac_addr);

    // src MAC must be MAC of the device(spoof check of PF)
    pkt_data[6..12].copy_from_slice(&mac_addr.0);

    // let mut buffer: VecDeque<>
}
