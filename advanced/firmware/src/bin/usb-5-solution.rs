#![no_main]
#![no_std]

use core::num::NonZeroU8;

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};
// this imports `beginner/apps/lib.rs` to retrieve our global logger + panicking-behavior
use firmware as _;
use usb2::{GetDescriptor as Descriptor, StandardRequest as Request, State};

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        usbd: USBD,
        ep0in: Ep0In,
        state: State,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        init::LateResources {
            usbd: board.usbd,
            state: State::Default,
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

fn on_event(usbd: &USBD, ep0in: &mut Ep0In, state: &mut State, event: Event) {
    defmt::info!("USB: {:?} @ {:?}", event, dk::uptime());

    match event {
        Event::UsbReset => {
            defmt::info!("USB reset condition detected");
            *state = State::Default;
        }

        Event::UsbEp0DataDone => {
            defmt::info!("EP0IN: transfer complete");
            ep0in.end(usbd);
        }

        Event::UsbEp0Setup => {
            if ep0setup(usbd, ep0in, state).is_err() {
                defmt::warn!("EP0IN: unexpected request; stalling the endpoint");
                usbd::ep0stall(usbd);
            }
        }
    }
}

/// The `bConfigurationValue` of the only supported configuration
const CONFIG_VAL: u8 = 42;

fn ep0setup(usbd: &USBD, ep0in: &mut Ep0In, state: &mut State) -> Result<(), ()> {
    let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
    let brequest = usbd.brequest.read().brequest().bits();
    let wlength = usbd::wlength(usbd);
    let windex = usbd::windex(usbd);
    let wvalue = usbd::wvalue(usbd);

    defmt::info!(
        "bmrequesttype: {}, brequest: {}, wlength: {}, windex: {}, wvalue: {}",
        bmrequesttype,
        brequest,
        wlength,
        windex,
        wvalue
    );

    let request = Request::parse(bmrequesttype, brequest, wvalue, windex, wlength)
        .expect("Error parsing request");
    defmt::info!("EP0: {:?}", defmt::Debug2Format(&request));
    //                        ^^^^^^^^^^^^^^^^^^^ this adapter is currently needed to log
    //                                            `StandardRequest` with `defmt`

    match request {
        // section 9.4.3
        // this request is valid in any state
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

            Descriptor::Configuration { index } => {
                if index == 0 {
                    let mut resp = heapless::Vec::<u8, 64>::new();

                    let conf_desc = usb2::configuration::Descriptor {
                        wTotalLength: (usb2::configuration::Descriptor::SIZE
                            + usb2::interface::Descriptor::SIZE)
                            .into(),
                        bNumInterfaces: NonZeroU8::new(1).unwrap(),
                        bConfigurationValue: core::num::NonZeroU8::new(CONFIG_VAL).unwrap(),
                        iConfiguration: None,
                        bmAttributes: usb2::configuration::bmAttributes {
                            self_powered: true,
                            remote_wakeup: false,
                        },
                        bMaxPower: 250, // 500 mA
                    };

                    let iface_desc = usb2::interface::Descriptor {
                        bInterfaceNumber: 0,
                        bAlternativeSetting: 0,
                        bNumEndpoints: 0,
                        bInterfaceClass: 0,
                        bInterfaceSubClass: 0,
                        bInterfaceProtocol: 0,
                        iInterface: None,
                    };

                    resp.extend_from_slice(&conf_desc.bytes()).unwrap();
                    resp.extend_from_slice(&iface_desc.bytes()).unwrap();
                    ep0in.start(&resp[..core::cmp::min(resp.len(), length.into())], usbd);
                } else {
                    // out of bounds access: stall the endpoint
                    return Err(());
                }
            }

            _ => return Err(()),
        },

        Request::SetAddress { address } => {
            match state {
                State::Default => {
                    if let Some(address) = address {
                        *state = State::Address(address);
                    } else {
                        // stay in the default state
                    }
                }

                State::Address(..) => {
                    if let Some(address) = address {
                        // use the new address
                        *state = State::Address(address);
                    } else {
                        *state = State::Default;
                    }
                }

                // unspecified behavior
                State::Configured { .. } => return Err(()),
            }

            // the response to this request is handled in hardware
        }

        Request::SetConfiguration { value } => {
            match *state {
                // unspecified behavior
                State::Default => return Err(()),

                State::Address(address) => {
                    if let Some(value) = value {
                        if value.get() == CONFIG_VAL {
                            defmt::info!("entering the configured state");
                            *state = State::Configured { address, value };
                        } else {
                            defmt::error!("unsupported configuration value");
                            return Err(());
                        }
                    } else {
                        // stay in the address mode
                    }
                }

                State::Configured {
                    address,
                    value: curr_value,
                } => {
                    if let Some(new_value) = value {
                        if new_value.get() == CONFIG_VAL {
                            if new_value != curr_value {
                                defmt::info!("changing configuration");
                                *state = State::Configured {
                                    address,
                                    value: new_value,
                                };
                            }
                        } else {
                            defmt::error!("unsupported configuration value");
                            return Err(());
                        }
                    } else {
                        defmt::info!("returned to the address state");
                        *state = State::Address(address);
                    }
                }
            }

            usbd.tasks_ep0status
                .write(|w| w.tasks_ep0status().set_bit());
        }

        // stall any other request
        _ => return Err(()),
    }

    Ok(())
}
