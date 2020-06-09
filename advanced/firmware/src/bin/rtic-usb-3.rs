#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};
use panic_log as _; // panic handler

// use one of these
// use usb::{Descriptor, Request}; // your implementation
use usb2::{GetDescriptor as Descriptor, StandardRequest as Request}; // crates.io impl

#[rtfm::app(device = dk)]
const APP: () = {
    struct Resources {
        usbd: USBD,
        ep0in: Ep0In,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        usbd::connect(&board.usbd);

        init::LateResources {
            ep0in: board.ep0in,
            usbd: board.usbd,
        }
    }

    #[task(binds = USBD, resources = [usbd, ep0in])]
    fn on_usbd(cx: on_usbd::Context) {
        let usbd = cx.resources.usbd;
        let ep0in = cx.resources.ep0in;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, ep0in, event)
        }
    }
};

fn on_event(usbd: &USBD, ep0in: &mut Ep0In, event: Event) {
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
                        log::info!("GET_DESCRIPTOR Device [length={}]", length);

                        // FIXME send back a valid device descriptor, truncated to `length` bytes
                        // let desc = usb2::device::Descriptor { .. };
                        let resp = [];
                        let _ = ep0in.start(&resp, usbd);
                    }

                    _ => usbd::todo(usbd),
                }
            } else {
                usbd::todo(usbd)
            }
        }
    }
}
