#![no_main]
#![no_std]

// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use cortex_m::asm;
    use dk::peripheral::POWER;

    #[local]
    struct MyLocalResources {
        power: POWER,
        counter: usize,
    }

    #[shared]
    struct MySharedResources { 
    }

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources, init::Monotonics) {
        let board = dk::init().unwrap();

        let power = board.power;

        power.intenset.write(|w| w.usbdetected().set_bit());

        defmt::println!("USBDETECTED interrupt enabled");

        (
            MySharedResources {}, 
            MyLocalResources {
                power,
                counter: 0, // <- initialize the new resource
                }, 
            init::Monotonics()
        )
    }
   

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            defmt::println!("idle: going to sleep");
            asm::wfi();
            defmt::println!("idle: woke up");
        }
    }

    #[task(binds = POWER_CLOCK, local = [power, counter])]
    //                                          ^^^^^^^ we want to access the resource from here
    fn on_power_event(cx: on_power_event::Context) {
        defmt::debug!("POWER event occurred");

        let power = cx.local.power;
        let counter = cx.local.counter;

        *counter += 1;
        let n = *counter;
        defmt::println!(
            "on_power_event: cable connected {} time{}",
            n,
            if n != 1 { "s" } else { "" }
        );

        // clear the interrupt flag; otherwise this task will run again after it returns
        power.events_usbdetected.reset();
    }
}
