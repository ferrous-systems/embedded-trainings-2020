#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_probe as _; // the panicking behavior

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
