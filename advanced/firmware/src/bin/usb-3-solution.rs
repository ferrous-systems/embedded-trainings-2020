#![no_main]
#![no_std]

// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;

#[rtic::app(device = dk, peripherals = false)]
mod app {
    use dk::{
        peripheral::USBD,
        usbd::{self, Ep0In, Event},
    };

    use usb::{Descriptor, Request};

    #[local]
    struct MyLocalResources {
        usbd: USBD,
        ep0in: Ep0In,
    }

    #[shared]
    struct MySharedResources {}

    #[init]
    fn init(_cx: init::Context) -> (MySharedResources, MyLocalResources, init::Monotonics) {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        (
            MySharedResources {},
            MyLocalResources {
                usbd: board.usbd,
                ep0in: board.ep0in,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USBD, local = [usbd, ep0in])]
    fn main(cx: main::Context) {
        let usbd = cx.local.usbd;
        let ep0in = cx.local.ep0in;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, ep0in, event)
        }
    }

    fn on_event(usbd: &USBD, ep0in: &mut Ep0In, event: Event) {
        defmt::println!("USB: {} @ {}", event, dk::uptime());

        match event {
            Event::UsbReset => {
                // nothing to do here at the moment
            }

            Event::UsbEp0DataDone => ep0in.end(usbd),

            Event::UsbEp0Setup => {
                let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
                let brequest = usbd.brequest.read().brequest().bits();
                let wlength = usbd::wlength(usbd);
                let windex = usbd::windex(usbd);
                let wvalue = usbd::wvalue(usbd);

                defmt::println!(
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
                        defmt::println!("GET_DESCRIPTOR Device [length={}]", length);

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
                        let desc_bytes = desc.bytes();
                        let resp =
                            &desc_bytes[..core::cmp::min(desc_bytes.len(), usize::from(length))];
                        ep0in.start(&resp, usbd);
                    }
                    Request::SetAddress { .. } => {
                        // On Mac OS you'll get this request before the GET_DESCRIPTOR request so we
                        // need to catch it here. We'll properly handle this request later
                        // but for now it's OK to do nothing.
                    }
                    _ => {
                        defmt::error!(
                            "unknown request (goal achieved if GET_DESCRIPTOR Device was handled before)"
                        );
                        dk::exit()
                    }
                }
            }
        }
    }
}
