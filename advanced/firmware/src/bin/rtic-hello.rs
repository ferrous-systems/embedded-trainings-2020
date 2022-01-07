#![no_main]
#![no_std]


// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[rtic::app(device = dk, peripherals = true)]
mod app {
    use cortex_m::asm;
    use dk::Peripherals;

    #[local]
    struct MyLocalResources {
    }

    #[shared]
    struct MySharedResources {
        
    }

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources, init::Monotonics) {
        dk::init().unwrap();

        defmt::info!("Hello");
        (MySharedResources {}, MyLocalResources {}, init::Monotonics())
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        defmt::info!("world!");

        loop {
            asm::bkpt();
        }
    }
}