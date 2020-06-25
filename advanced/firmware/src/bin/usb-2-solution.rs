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
            let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
            let brequest = usbd.brequest.read().brequest().bits();
            let wlength = (u16::from(usbd.wlengthh.read().wlengthh().bits()) << 8)
                | u16::from(usbd.wlengthl.read().wlengthl().bits());
            let windex = (u16::from(usbd.windexh.read().windexh().bits()) << 8)
                | u16::from(usbd.windexl.read().windexl().bits());
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

            if let Ok(Request::GetDescriptor { descriptor, length }) =
                Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)
            {
                match descriptor {
                    Descriptor::Device => {
                        log::info!("GET_DESCRIPTOR Device [length={}]", length);

                        log::info!("Goal reached; move to the next section");
                        dk::exit()
                    }
                }
            } else {
                unreachable!() // don't care about this for now
            }
        }
    }
}
