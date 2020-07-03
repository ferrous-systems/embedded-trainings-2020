#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};
use panic_log as _; // panic handler

// use one of these
use usb::{Descriptor, Request};

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        usbd: USBD,
        ep0in: Ep0In,
        state: usb2::State,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        init::LateResources {
            usbd: board.usbd,
            state: usb2::State::Default,
            ep0in: board.ep0in,
        }
    }

    #[task(binds = USBD, resources = [usbd, ep0in, state])]
    fn main(cx: main::Context) {
        let usbd = cx.resources.usbd;
        let ep0in = cx.resources.ep0in;
        let state = cx.resources.state;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, ep0in, state, event)
        }
    }
};

fn on_event(usbd: &USBD, ep0in: &mut Ep0In, state: &mut usb2::State, event: Event) {
    log::info!("USB: {:?} @ {:?}", event, dk::uptime());

    match event {
        // TODO change `state`
        Event::UsbReset => todo!(),

        Event::UsbEp0DataDone => ep0in.end(usbd),

        Event::UsbEp0Setup => {
            if ep0setup(usbd, ep0in, state).is_err() {
                // unsupported or invalid request: stall the endpoint
                log::warn!("EP0IN: stalled");
            }
        }
    }
}

fn ep0setup(usbd: &USBD, ep0in: &mut Ep0In, _state: &mut usb2::State) -> Result<(), ()> {
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

    let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)
        .expect("Error parsing request");
    log::info!("EP0: {:?}", request);

    match request {
        Request::GetDescriptor { descriptor, length } => match descriptor {
            Descriptor::Device => {
                let desc = usb2::device::Descriptor {
                    bDeviceClass: 0,
                    bDeviceProtocol: 0,
                    bDeviceSubClass: 0,
                    bMaxPacketSize0: usb2::device::bMaxPacketSize0::B64,
                    bNumConfigurations: core::num::NonZeroU8::new(1).unwrap(),
                    bcdDevice: 0x01_00, // 1.00
                    iManufacturer: None,
                    iProduct: None,
                    iSerialNumber: None,
                    idProduct: consts::PID,
                    idVendor: consts::VID,
                };
                let bytes = desc.bytes();
                let _ = ep0in.start(&bytes[..core::cmp::min(bytes.len(), length.into())], usbd);
            }

            // TODO implement Configuration descriptor
            // Descriptor::Configuration { .. } => todo!(),
        },
        Request::SetAddress { .. } => {
            // On Mac OS you'll get this request before the GET_DESCRIPTOR request so we
            // need to catch it here.

            // TODO: handle this request properly now.
            todo!()
        }
        // TODO handle SET_CONFIGURATION request
        // Request::SetConfiguration { .. } => todo!(),
    }

    Ok(())
}
