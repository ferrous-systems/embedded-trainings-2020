#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};
use panic_log as _; // panic handler
use usb::{Descriptor, Request};

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        usbd: USBD,
        ep0in: Ep0In,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        init::LateResources {
            ep0in: board.ep0in,
            usbd: board.usbd,
        }
    }

    #[task(binds = USBD, resources = [usbd, ep0in])]
    fn main(cx: main::Context) {
        let usbd = cx.resources.usbd;
        let ep0in = cx.resources.ep0in;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, ep0in, event)
        }
    }
};

fn on_event(usbd: &USBD, ep0in: &mut Ep0In, event: Event) {
    log::info!("USB: {:?} @ {:?}", event, dk::uptime());

    match event {
        Event::UsbReset => {
            // nothing to do here at the moment
        }

        Event::UsbEp0DataDone => todo!(), // <- TODO

        Event::UsbEp0Setup => {
            let bmrequesttype = usbd::bmrequesttype(usbd);
            let brequest = usbd::brequest(usbd);
            let wlength = usbd::wlength(usbd);
            let windex = usbd::windex(usbd);
            let wvalue = usbd::wvalue(usbd);

            log::info!(
                "SETUP: bmrequesttype: {}, brequest: {}, wlength: {}, windex: {}, wvalue: {}",
                bmrequesttype,
                brequest,
                wlength,
                windex,
                wvalue
            );

            let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength).expect(
                "Error parsing request (goal achieved if GET_DESCRIPTOR Device was handled before)",
            );
            match request {
                Request::GetDescriptor { descriptor, length }
                    if descriptor == Descriptor::Device =>
                {
                    log::info!("GET_DESCRIPTOR Device [length={}]", length);

                    // TODO send back a valid device descriptor, truncated to `length` bytes
                    // let desc = usb2::device::Descriptor { .. };
                    let resp = [];

                    // ensure we're not overstepping boundaries
                    assert!(resp.len() <= length as usize);
                    ep0in.start(&resp, usbd);
                }
                Request::SetAddress { .. } => {
                    // On Mac OS you'll get this request before the GET_DESCRIPTOR request so we
                    // need to catch it here. We'll properly handle this request later
                    // but for now it's OK to do nothing.
                }
                _ => {
                    log::error!(
                        "unknown request (goal achieved if GET_DESCRIPTOR Device was handled before)"
                    );
                    dk::exit()
                }
            }
        }
    }
}
