#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[entry]
fn main() -> ! {
    // board initialization
    dk::init().unwrap();

    defmt::info!("Hello, world!");

    loop {
        asm::bkpt();
    }
}
