#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;

#[entry]
fn main() -> ! {
    // initializes the peripherals
    dk::init().unwrap();

    defmt::info!("Hello, world!"); // :wave:

    loop {
        // breakpoint: halts the program's execution
        asm::bkpt();
    }
}
