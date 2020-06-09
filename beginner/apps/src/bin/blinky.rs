#![no_main]
#![no_std]

use core::time::Duration;

use cortex_m_rt::entry;
use panic_log as _; // panic handler

#[entry]
fn main() -> ! {
    // uncomment to enable more verbose logs
    // log::set_max_level(log::LevelFilter::Trace);

    let board = dk::init().unwrap();

    let mut led = board.leds._1;
    let mut timer = board.timer;

    for _ in 0..10 {
        led.toggle();
        timer.wait(Duration::from_secs(1));
        log::info!("LED toggled at {:?}", dk::uptime());
    }

    dk::exit()
}
