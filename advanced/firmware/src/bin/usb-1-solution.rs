#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Event},
};
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        usbd: USBD,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        init::LateResources { usbd: board.usbd }
    }

    #[task(binds = USBD, resources = [usbd])]
    fn main(cx: main::Context) {
        let usbd = cx.resources.usbd;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, event)
        }
    }
};

fn on_event(_usbd: &USBD, event: Event) {
    defmt::info!("USB: {:?}", event);

    match event {
        Event::UsbReset => {
            // going from the Default state to the Default state is a no-operation
            defmt::info!("returning to the Default state");
        }

        Event::UsbEp0DataDone => todo!(),

        Event::UsbEp0Setup => {
            defmt::info!("goal reached; move to the next section");
            dk::exit()
        }
    }
}
