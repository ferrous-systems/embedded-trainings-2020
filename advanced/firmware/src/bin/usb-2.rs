#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Event},
};
use panic_log as _; // panic handler
use usb::{Descriptor, Request};

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
    log::info!("USB: {:?} @ {:?}", event, dk::uptime());

    match event {
        Event::UsbReset => {
            // nothing to do here at the moment
        }

        Event::UsbEp0DataDone => todo!(),

        Event::UsbEp0Setup => {
            // TODO read USBD registers
            let bmrequesttype = 0;
            let brequest = 0;
            let wlength = 0;
            let windex = 0;
            let wvalue = 0;

            log::info!(
                "SETUP: bmrequesttype: {}, brequest: {}, wlength: {}, windex: {}, wvalue: {}",
                bmrequesttype,
                brequest,
                wlength,
                windex,
                wvalue
            );

            let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)
                .expect("Error parsing request");
            match request {
                Request::GetDescriptor { descriptor, length }
                    if descriptor == Descriptor::Device =>
                {
                    // TODO modify `Request::parse()` in `advanced/common/usb/lib.rs`
                    // so that this branch is reached

                    log::info!("GET_DESCRIPTOR Device [length={}]", length);

                    log::info!("Goal reached; move to the next section");
                    dk::exit()
                }
                Request::SetAddress { .. } => {
                    // On Mac OS you'll get this request before the GET_DESCRIPTOR request so we
                    // need to catch it here. We'll properly handle this request later
                    // but for now it's OK to do nothing.
                }
                _ => unreachable!(), // we don't handle any other Requests
            }
        }
    }
}
