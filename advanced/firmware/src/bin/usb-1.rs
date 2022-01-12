#![no_main]
#![no_std]

// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use cortex_m::asm;
    use dk::{
        peripheral::USBD,
        usbd::{self, Event},
    };

    #[local]
    struct MyLocalResources {
        usbd: USBD,
    }

    #[shared]
    struct MySharedResources {  
    }

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources, init::Monotonics) {
        let board = dk::init().unwrap();

        // initialize the USBD peripheral
        // NOTE this will block if the USB cable is not connected to port J3
        usbd::init(board.power, &board.usbd);

        defmt::println!("USBD initialized");

        (MySharedResources {}, MyLocalResources { usbd: board.usbd }, init::Monotonics())
    }

    #[task(binds = USBD, local = [usbd])]
    fn main(cx: main::Context) {
        let usbd = cx.local.usbd;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, event)
        }
    }

    fn on_event(_usbd: &USBD, event: Event) {
        defmt::println!("USB: {} @ {}", event, dk::uptime());
    
        match event {
            Event::UsbReset => todo!(),
    
            Event::UsbEp0DataDone => todo!(),
            // leave this at it is for now.
            Event::UsbEp0Setup => {
                defmt::println!("goal reached; move to the next section");
                dk::exit();
            }
        }
    }
}


