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

        // initialize the USBD peripheral
        // NOTE this will block if the USB cable is not connected to port J3
        usbd::init(board.power, &board.usbd);

        defmt::info!("USBD initialized");

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
    defmt::info!("USB: {:?} @ {:?}", event, dk::uptime());

    match event {
        Event::UsbReset => todo!(),

        Event::UsbEp0DataDone => todo!(),

        Event::UsbEp0Setup => {
            defmt::info!("goal reached; move to the next section");
            dk::exit()
        }
    }
}
