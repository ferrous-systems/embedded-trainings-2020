#![no_main]
#![no_std]

use cortex_m::asm;
use panic_log as _; // panic handler

#[rtic::app(device = dk)]
const APP: () = {
    #[init]
    fn init(_cx: init::Context) {
        dk::init().unwrap();

        log::info!("Hello");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        log::info!("world!");

        loop {
            asm::bkpt();
        }
    }
};
