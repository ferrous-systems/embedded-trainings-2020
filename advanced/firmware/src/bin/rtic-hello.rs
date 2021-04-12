#![no_main]
#![no_std]

use cortex_m::asm;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[rtic::app(device = dk)]
const APP: () = {
    #[init]
    fn init(_cx: init::Context) {
        dk::init().unwrap();

        defmt::info!("Hello");
    }

    #[idle]
    fn main(_cx: main::Context) -> ! {
        defmt::info!("world!");

        loop {
            asm::bkpt();
        }
    }
};
