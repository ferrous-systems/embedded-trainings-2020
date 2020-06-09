#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Event},
};
use panic_log as _; // panic handler
use usb::{Descriptor, Request};

#[rtfm::app(device = dk)]
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
    fn on_usbd(cx: on_usbd::Context) {
        let usbd = cx.resources.usbd;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, event)
        }
    }
};

fn on_event(usbd: &USBD, event: Event) {
    log::info!("USB: {:?}", event);

    match event {
        Event::UsbReset => {
            // nothing to do here at the moment
        }

        Event::UsbEp0DataDone => usbd::todo(usbd),

        Event::UsbEp0Setup => {
            let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
            let brequest = usbd.brequest.read().brequest().bits();
            let wlength = usbd::wlength(usbd);
            let windex = usbd::windex(usbd);
            let wvalue = usbd::wvalue(usbd);

            log::info!(
                "bmrequesttype: {}, brequest: {}, wlength: {}, windex: {}, wvalue: {}",
                bmrequesttype,
                brequest,
                wlength,
                windex,
                wvalue
            );

            if let Ok(Request::GetDescriptor { descriptor, length }) =
                Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)
            {
                match descriptor {
                    Descriptor::Device => {
                        // GOAL
                        log::info!("GET_DESCRIPTOR Device [length={}]", length);

                        usbd::todo(usbd)
                    }

                    _ => usbd::todo(usbd),
                }
            } else {
                usbd::todo(usbd)
            }
        }
    }
}
