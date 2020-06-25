#![no_main]
#![no_std]

use cortex_m::asm;
use dk::peripheral::POWER;
use panic_log as _; // panic handler

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        power: POWER,
        counter: usize, // <- new resource
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        let power = board.power;

        power.intenset.write(|w| w.usbdetected().set_bit());

        log::info!("USBDETECTED interrupt enabled");

        init::LateResources {
            power,
            counter: 0, // <- initialize the new resource
        }
    }

    #[idle]
    fn main(_cx: main::Context) -> ! {
        loop {
            log::info!("idle: going to sleep");
            asm::wfi();
            log::info!("idle: woke up");
        }
    }

    #[task(binds = POWER_CLOCK, resources = [power, counter])]
    //                                              ^^^^^^^ we want to access the resource from here
    fn on_power_event(cx: on_power_event::Context) {
        log::debug!("POWER event occurred");

        let power = cx.resources.power;
        let counter = cx.resources.counter;

        *counter += 1;
        let n = *counter;
        log::info!(
            "on_power_event: cable connected {} time{}",
            n,
            if n != 1 { "s" } else { "" }
        );

        // clear the interrupt flag; otherwise this task will run again after it returns
        power.events_usbdetected.reset();
    }
};
