#![no_main]
#![no_std]

use cortex_m::asm;
use cortex_m_rt::entry;
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use apps as _;

#[entry]
fn main() -> ! {
    // uncomment to make logs more verbose
    // log::set_max_level(log::LevelFilter::Trace);

    let board = dk::init().unwrap();

    let mut leds = board.leds;
    leds._1.on();
    leds._2.off();
    leds._3.off();
    leds._4.on();

    // this program does not `exit`; use Ctrl+C to terminate it
    loop {
        asm::nop();
    }
}
