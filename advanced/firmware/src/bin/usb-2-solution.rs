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

fn on_event(usbd: &USBD, event: Event) {
    log::info!("USB: {:?} @ {:?}", event, dk::uptime());

    match event {
        Event::UsbReset => {
            // nothing to do here at the moment
        }

        Event::UsbEp0DataDone => todo!(),

        Event::UsbEp0Setup => {

            // the BMREQUESTTYPE register contains information about data recipient, transfer type and direction
            let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
            // the BREQUEST register stores the type of the current request (e.g. SET_ADDRESS, GET_DESCRIPTOR, ...)
            let brequest = usbd.brequest.read().brequest().bits();
            // wLength denotes the number of bytes to transfer (if any)
            // composed of a high register (WLENGTHH) and a low register (WLENGTHL)
            let wlength = (u16::from(usbd.wlengthh.read().wlengthh().bits()) << 8)
                | u16::from(usbd.wlengthl.read().wlengthl().bits());
            // wIndex is a generic index field whose meaning depends on the request type
            // composed of a high register (WINDEXH) and a low register (WINDEXL)
            let windex = (u16::from(usbd.windexh.read().windexh().bits()) << 8)
                | u16::from(usbd.windexl.read().windexl().bits());
            // wValue is a generic paremeter field meaning depends on the request type (e.g. contains the device
            // address in SET_ADRESS requests)
            // composed of a high register (WVALUEH) and a low register (WVALUEL)
            let wvalue = (u16::from(usbd.wvalueh.read().wvalueh().bits()) << 8)
                | u16::from(usbd.wvaluel.read().wvaluel().bits());

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
