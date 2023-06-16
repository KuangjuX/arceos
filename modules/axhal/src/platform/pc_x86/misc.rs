use core::time::Duration;

use x86_64::instructions::port::{Port, PortWriteOnly};

/// Shutdown the whole system (in QEMU), including all CPUs.
///
/// See <https://wiki.osdev.org/Shutdown> for more information.
pub fn terminate() -> ! {
    // info!("Shutting down...");
    // unsafe { PortWriteOnly::new(0x604).write(0x2000u16) };
    // crate::arch::halt();
    // warn!("It should shutdown!");
    // loop {
    //     crate::arch::halt();
    // }
    reboot()
}

/// Reboot the whole system, for debugging.
pub fn reboot() -> ! {
    info!("Rebooting OS......");
    crate::time::busy_wait_until(Duration::from_secs(10));
    unsafe {
        Port::new(0x64).write(0xfeu8);
    }
    unreachable!()
}
