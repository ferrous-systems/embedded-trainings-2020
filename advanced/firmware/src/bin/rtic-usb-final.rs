#![no_main]
#![no_std]

use core::num::NonZeroU8;

use dk::{
    peripheral::USBD,
    usbd::{self, Ep0In, Event},
};
use panic_log as _; // panic handler
use usb2::{GetDescriptor, StandardRequest, State};

#[rtic::app(device = dk)]
const APP: () = {
    struct Resources {
        ep0in: Ep0In,
        state: State,
        usbd: USBD,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        usbd::init(board.power, &board.usbd);

        usbd::connect(&board.usbd);

        init::LateResources {
            usbd: board.usbd,
            state: State::Default,
            ep0in: board.ep0in,
        }
    }

    #[task(binds = USBD, resources = [state, usbd, ep0in])]
    fn usb(cx: usb::Context) {
        let usbd = cx.resources.usbd;
        let state = cx.resources.state;
        let ep0in = cx.resources.ep0in;

        while let Some(event) = usbd::next_event(usbd) {
            on_event(usbd, state, ep0in, event)
        }
    }
};

fn on_event(usbd: &USBD, state: &mut State, ep0in: &mut Ep0In, event: Event) {
    log::info!("USB: {:?}", event);

    match event {
        Event::UsbReset => {
            log::info!("USB reset condition detected");
            *state = State::Default;
        }

        Event::UsbEp0DataDone => {
            log::info!("EP0IN: transfer complete");
            ep0in.end(usbd)
        }

        Event::UsbEp0Setup => {
            if ep0setup(usbd, state, ep0in).is_err() {
                log::warn!("EP0IN: stalled");
                usbd::ep0stall(usbd);
            }
        }
    }
}

/// The `bConfigurationValue` of the only supported configuration
const CONFIG_VAL: u8 = 1;

fn ep0setup(usbd: &USBD, state: &mut State, ep0in: &mut Ep0In) -> Result<(), ()> {
    let bmrequesttype = usbd.bmrequesttype.read().bits() as u8;
    let brequest = usbd.brequest.read().brequest().bits();
    let wlength = usbd::wlength(usbd);
    let windex = usbd::windex(usbd);
    let wvalue = usbd::wvalue(usbd);

    let request = StandardRequest::parse(bmrequesttype, brequest, wvalue, windex, wlength)?;
    log::info!("{:?}", request);

    match request {
        // section 9.4.3
        // this request is valid in any state
        StandardRequest::GetDescriptor { descriptor, length } => match descriptor {
            GetDescriptor::Device => {
                let desc = usb2::device::Descriptor {
                    bDeviceClass: 0,
                    bDeviceProtocol: 0,
                    bDeviceSubClass: 0,
                    bMaxPacketSize0: usb2::device::bMaxPacketSize0::B64,
                    bNumConfigurations: core::num::NonZeroU8::new(1).unwrap(),
                    bcdDevice: 0x0100, // 1.00
                    iManufacturer: None,
                    iProduct: None,
                    iSerialNumber: None,
                    idProduct: consts::PID,
                    idVendor: consts::VID,
                };
                let bytes = desc.bytes();
                ep0in.start(&bytes[..core::cmp::min(bytes.len(), length.into())], usbd)?
            }

            GetDescriptor::Configuration { index } => {
                if index == 0 {
                    let mut full_desc = heapless::Vec::<u8, heapless::consts::U64>::new();

                    let conf_desc = usb2::configuration::Descriptor {
                        wTotalLength: usb2::configuration::Descriptor::SIZE.into(),
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

                    full_desc.extend_from_slice(&conf_desc.bytes()).unwrap();
                    full_desc.extend_from_slice(&iface_desc.bytes()).unwrap();
                    ep0in.start(
                        &full_desc[..core::cmp::min(full_desc.len(), length.into())],
                        usbd,
                    )?;
                } else {
                    // out of bounds access: stall the endpoint
                    return Err(());
                }
            }

            _ => return Err(()),
        },

        StandardRequest::SetAddress { address } => {
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

        StandardRequest::SetConfiguration { value } => {
            log::info!("SET_CONFIGURATION {:?} ({:?})", value, state);

            match *state {
                // unspecified behavior
                State::Default => return Err(()),

                State::Address(address) => {
                    if let Some(value) = value {
                        if value.get() == CONFIG_VAL {
                            log::info!("entering the configured state");
                            *state = State::Configured { address, value };
                        } else {
                            log::error!("unsupported configuration value");
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
                                log::info!("changing configuration");
                                *state = State::Configured {
                                    address,
                                    value: new_value,
                                };
                            }
                        } else {
                            log::error!("unsupported configuration value");
                            return Err(());
                        }
                    } else {
                        log::info!("returned to the address state");
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
