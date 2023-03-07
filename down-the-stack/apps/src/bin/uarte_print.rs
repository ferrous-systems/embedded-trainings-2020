#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use core::fmt::Write;

// this imports `down-the-stack/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;

#[entry]
fn main() -> ! {
    // to enable more verbose logs, go to your `Cargo.toml` and set defmt logging levels
    // to `defmt-trace` by changing the `default = []` entry in `[features]`

    let board = dk_bsc::init().unwrap();
    let mut uarte = board.uarte;

    let tx_buffer = "Hello, World!\n";
    uarte.write_str(tx_buffer).unwrap();
    
    // this program does not `exit`; use Ctrl+C to terminate it
    loop {
        asm::nop();
    }
}
