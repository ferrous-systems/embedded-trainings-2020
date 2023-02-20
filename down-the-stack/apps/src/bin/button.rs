#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
use core::fmt::Write;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;

#[entry]
fn main() -> ! {
    // to enable more verbose logs, go to your `Cargo.toml` and set defmt logging levels
    // to `defmt-trace` by changing the `default = []` entry in `[features]`

    let board = dk_bsc::init().unwrap();

    let mut led = board.leds;
    let button_1 = board.buttons.b_1;

    loop {
        if button_1.is_pushed() {
            led.led_1.on();
        } else {
            led.led_1.off();
        }
    }

    // this program does not `exit`; use Ctrl+C to terminate it
    loop {
        asm::nop();
    }
}
