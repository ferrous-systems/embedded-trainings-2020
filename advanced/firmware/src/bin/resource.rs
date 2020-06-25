#![no_main]
#![no_std]

use cortex_m::asm;
use dk::peripheral::POWER;
use panic_log as _; // panic handler

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        power: POWER, // <- resource declaration
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        let power = board.power;

        power.intenset.write(|w| w.usbdetected().set_bit());

        log::info!("USBDETECTED interrupt enabled");

        init::LateResources {
            power, // <- resource initialization
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

    #[task(binds = POWER_CLOCK, resources = [power])]
    //                                      ^^^^^^^ resource access list
    fn on_power_event(cx: on_power_event::Context) {
        log::info!("POWER event occurred");

        // resources available to this task
        let resources = cx.resources;

        // the POWER peripheral can be accessed through a reference
        let power: &mut POWER = resources.power;

        // clear the interrupt flag; otherwise this task will run again after it returns
        power.events_usbdetected.reset();
    }
};
