use x86_64::instructions::port::PortWriteOnly;

/// Shutdown the whole system (in QEMU), including all CPUs.
///
/// See <https://wiki.osdev.org/Shutdown> for more information.
pub fn terminate() -> ! {
    info!("Shutting down...");
    #[cfg(platform = "x86_64-pc-oslab")]
    unsafe {
        PortWriteOnly::new(0x64).write(0xfeu8)
    };

    #[cfg(platform = "x86_64-qemu-q35")]
    unsafe {
        PortWriteOnly::new(0x604).write(0x2000u16)
    };
    crate::arch::halt();
    warn!("It should shutdown!");
    loop {
        crate::arch::halt();
    }
}
