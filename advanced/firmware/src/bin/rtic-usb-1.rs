#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Event},
};
use panic_log as _; // panic handler

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        usbd: USBD,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        usbd::connect(&board.usbd);

        init::LateResources { usbd: board.usbd }
    }

    #[task(binds = USBD, resources = [usbd])]
    fn usb(cx: usb::Context) {
        let usbd = cx.resources.usbd;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, event)
        }
    }
};

fn on_event(usbd: &USBD, event: Event) {
    log::info!("USB: {:?}", event);

    match event {
        Event::UsbReset => usbd::todo(usbd),

        Event::UsbEp0DataDone => usbd::todo(usbd),

        Event::UsbEp0Setup => usbd::todo(usbd),
    }
}
