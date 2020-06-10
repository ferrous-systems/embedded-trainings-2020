#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_log as _; // panic handler

#[entry]
fn main() -> ! {
    // board initialization
    dk::init().unwrap();

    log::info!("Hello, world!");

    loop {
        asm::bkpt();
    }
}
