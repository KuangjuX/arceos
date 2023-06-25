#![no_std]
#![no_main]

use core::time::Duration;

use alloc::collections::VecDeque;
use libax::{net::recv, thread};

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
    libax::info!("Ixgbe echo");

    let mac_addr = libax::net::get_mac_addr();
    libax::info!("mac addr: {}", mac_addr);

    let stats = libax::net::read_stats();

    libax::info!("stats: {}", stats);

    libax::info!("Start receving packets......");
    loop {
        // echo
        match recv() {
            Ok(buf) => {
                libax::info!("buf: {:?}", buf.as_bytes())
            }
            _ => {}
        }
        libax::info!("sleep 5 seconds");
        thread::sleep(Duration::from_secs(5));
    }
}
