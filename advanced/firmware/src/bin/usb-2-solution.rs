#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Event},
};
use panic_log as _; // panic handler
use usb::{Descriptor, Request, DeviceState};

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        state: DeviceState,
        usbd: USBD,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        init::LateResources {
            usbd: board.usbd,
            state: DeviceState::Default,
        }
    }

    #[task(binds = USBD, resources = [state, usbd])]
    fn main(cx: main::Context) {
        let usbd = cx.resources.usbd;
        let state = cx.resources.state; // used to store the device address sent by the host

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, state, event)
        }
    }
};

fn on_event(usbd: &USBD, state: &mut DeviceState, event: Event) {
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

            // todo handle less indentedly?
            if let Ok(request) = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength) {
                match request {
                    Request::GetDescriptor { descriptor, length }
                        if descriptor == Descriptor::Device =>
                    {
                        log::info!("GET_DESCRIPTOR Device [length={}]", length);

                        log::info!("Goal reached; move to the next section");
                        dk::exit()
                    }
                    Request::SetAddress { address } => {
                        log::info!("SETUP: device state is {:?}", state);

                        // todo hide this in a helper?
                        // todo check for state configured -> err
                        if let Some(address) = address {
                            log::info!("SETUP: assigning address {}", address);
                            *state = DeviceState::Address(address);
                        } else {
                            log::info!("SETUP: address was None; assigning Default");
                            *state = DeviceState::Default;
                        }
                    }
                    _ => unreachable!(), // we don't handle any other Requests
                }
            } else {
                unreachable!() // don't care about this for now
            }
        }
    }
}
