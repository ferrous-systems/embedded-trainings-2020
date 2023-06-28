// this program does not use the standard library to avoid heap allocations.
// only the `core` library functions are available.
#![no_std]
// this program uses a custom entry point instead of `fn main()`
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;

// the custom entry point
// vvvvv
#[entry]
fn main() -> ! {
    //      ˆˆˆ
    //       ! is the 'never' type: this function never returns

    // initializes the peripherals
    dk_bsc::init().unwrap();

    defmt::println!("Hello, world!"); // :wave:

    loop {
        // breakpoint: halts the program's execution
        asm::bkpt();
    }
}
