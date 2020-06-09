#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_log as _; // the panicking behavior

#[entry]
fn main() -> ! {
    // initializes the peripherals
    dk::init().unwrap();

    log::info!("Hello, world!"); // :wave:

    loop {
        // breakpoint: halts the program's execution
        asm::bkpt();
    }
}
