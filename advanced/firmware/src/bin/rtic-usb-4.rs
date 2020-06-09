#![no_main]
#![no_std]

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};
use panic_log as _; // panic handler

// use one of these
// use usb::{Descriptor, Request}; // your implementation
use usb2::State;
use usb2::{GetDescriptor as Descriptor, StandardRequest as Request}; // crates.io impl

#[rtfm::app(device = dk)]
const APP: () = {
    struct Resources {
        #[init([0; 64])]
        buffer: [u8; 64],
        usbd: USBD,
        ep0in: Ep0In,
        state: State,
    }

    #[init(resources = [buffer])]
    fn init(cx: init::Context) -> init::LateResources {
        let buffer: &'static mut [u8; 64] = cx.resources.buffer;
        let ep0in = Ep0In::new(buffer);

        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        usbd::connect(&board.usbd);

        init::LateResources {
            usbd: board.usbd,
            state: State::Default,
            ep0in,
        }
    }

    #[task(binds = USBD, resources = [usbd, ep0in, state])]
    fn on_usbd(cx: on_usbd::Context) {
        let usbd = cx.resources.usbd;
        let ep0in = cx.resources.ep0in;
        let state = cx.resources.state;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, ep0in, state, event)
        }
    }
};

fn on_event(usbd: &USBD, ep0in: &mut Ep0In, state: &mut State, event: Event) {
    log::info!("USB: {:?}", event);

    match event {
        Event::UsbReset => {
            // TODO change `state`
        }

        Event::UsbEp0DataDone => ep0in.end(usbd),

        Event::UsbEp0Setup => {
            if ep0setup(usbd, ep0in, state).is_err() {
                // unsupported or invalid request: stall the endpoint
                log::warn!("EP0IN: stalled");
                usbd::todo(usbd)
            }
        }
    }
}

fn ep0setup(usbd: &USBD, ep0in: &mut Ep0In, state: &mut State) -> Result<(), ()> {
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

    let req = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)?;
    log::info!("{:?}", req);

    match req {
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

            _ => usbd::todo(usbd),
        },

        Request::SetAddress { .. } => usbd::todo(usbd),

        Request::SetConfiguration { .. } => usbd::todo(usbd),

        // this request is not supported
        _ => return Err(()),
    }

    Ok(())
}
