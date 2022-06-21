#![no_std]
#![no_main]

use rust_emb as _; // global logger + panicking-behavior + memory layout


#[rtic::app(device = nrf52833_hal::pac, dispatchers = [UARTE1])]
mod app {
    use nrf52833_hal::{
        gpio::{p0::Parts, Level, Output, Pin, PushPull},
        pac::TIMER0,
        prelude::*,
    };
    use rust_emb::mono::{ExtU32, MonoTimer};

    #[monotonic(binds = TIMER0, default = true)]
    type MonoTimer0 = MonoTimer<TIMER0>;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: Pin<Output<PushPull>>
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mono = MonoTimer::new(ctx.device.TIMER0);
        
        let p0 = Parts::new(ctx.device.P0);
        let led = p0.p0_21.into_push_pull_output(Level::High).degrade(); // Create the blinky LED
        p0.p0_11.into_push_pull_output(Level::Low); // Set column to low to allow blinking

        defmt::println!("RTIC blinky example");
        blink::spawn().ok();
        (Shared {}, Local { led }, init::Monotonics(mono))
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(local = [led])]
    fn blink(ctx: blink::Context) {
        defmt::info!("Blink!");
        let led = ctx.local.led;

        if led.is_set_low().unwrap() {
            led.set_high().ok();
        } else {
            led.set_low().ok();
        }

        blink::spawn_after(1.secs()).ok();
    }
}