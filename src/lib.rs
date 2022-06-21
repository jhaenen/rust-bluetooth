#![no_std]

use core::sync::atomic::{AtomicUsize, Ordering};

use defmt_rtt as _; // global logger
use nrf52833_hal as _; // memory layout
pub mod mono;

use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf() // Cancel dubble panic logging
}

static COUNT: AtomicUsize = AtomicUsize::new(0);
defmt::timestamp!("{=usize}", {
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n
});

// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() {
    loop {
        cortex_m::asm::bkpt();
    }
}