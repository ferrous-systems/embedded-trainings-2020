// this program does not use the standard library to avoid heap allocations.
// only the `core` library functions are available.
#![no_std]
// this program uses a custom entry point instead of `fn main()`
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use defmt_rtt as _; // global logger
use panic_probe as _; // the panicking behavior

// the custom entry point
// vvvvv
#[entry]
fn main() -> ! {
    //      ˆˆˆ
    //       ! is the 'never' type: this function never returns

    // initializes the peripherals
    dk::init().unwrap();

    defmt::info!("Hello, world!"); // :wave:

    loop {
        // breakpoint: halts the program's execution
        asm::bkpt();
    }
}
