//! `radio` & `usb` apps merged into a single application

#![no_std]
#![no_main]

use core::fmt::Write as _;
use core::sync::atomic::{AtomicBool, Ordering};

use embedded_hal::digital::v2::{OutputPin as _, StatefulOutputPin as _};
use hal::{
    clocks::{Clocks, ExternalOscillator, Internal, LfOscStopped},
    gpio::{
        p0::{self, P0_06, P0_12},
        Level, Output, PushPull,
    },
    ieee802154::{Packet, Radio},
    target::USBD,
    usbd::Usbd,
};
use heapless::String;
use panic_halt as _; // panic handler
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

// TODO make channel configurable via a single `const`ant

#[rtfm::app(device = hal::target, peripherals = true)]
const APP: () = {
    struct Resources {
        blue: P0_12<Output<PushPull>>,
        configured: AtomicBool,
        green: P0_06<Output<PushPull>>,
        radio: Radio<'static>,
        serial_port: SerialPort<'static, Usbd<'static>>,
        usb_dev: UsbDevice<'static, Usbd<'static>>,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        static mut EP_BUF: [u8; 1024] = [0; 1024];
        static mut CLOCKS: Option<Clocks<ExternalOscillator, Internal, LfOscStopped>> = None;
        static mut USB_BUS: Option<UsbBusAllocator<Usbd<'static>>> = None;

        let periph = cx.device;

        let port = p0::Parts::new(periph.P0);
        // P0.6: green LED indicates USB is working
        // P0.12: blue LED is toggled on each new packet
        let green = port.p0_06.into_push_pull_output(Level::High);
        let blue = port.p0_12.into_push_pull_output(Level::High);

        let clocks = Clocks::new(periph.CLOCK);
        // extend lifetime to `'static`
        let clocks = CLOCKS.get_or_insert(clocks.enable_ext_hfosc());

        let mut radio = Radio::init(periph.RADIO, clocks);
        // set TX power to its maximum value
        radio.set_txpower(8);

        // these loops should always terminate
        // detected USB power? (device is USB powered)
        while !periph
            .POWER
            .usbregstatus
            .read()
            .vbusdetect()
            .is_vbus_present()
        {}

        // wait until USB 3.3V supply (internal regulator) is stable
        while !periph
            .POWER
            .events_usbpwrrdy
            .read()
            .events_usbpwrrdy()
            .bit_is_clear()
        {}

        // enable all interrupts
        periph.USBD.intenset.write(|w| {
            w.endepin0().set_bit();
            w.endepin1().set_bit();
            w.endepin2().set_bit();
            w.endepout0().set_bit();
            w.endepout1().set_bit();
            w.endepout2().set_bit();
            w.ep0datadone().set_bit();
            w.ep0setup().set_bit();
            w.epdata().set_bit();
            w.usbevent().set_bit();
            w.usbreset().set_bit();
            w.sof().set_bit();
            w
        });

        let usb_bus = USB_BUS.get_or_insert(Usbd::new_alloc(periph.USBD, EP_BUF, clocks));
        let serial_port = SerialPort::new(usb_bus);

        let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(consts::VID, consts::PID))
            .device_class(USB_CLASS_CDC)
            .max_packet_size_0(64) // (makes control transfers 8x faster)
            .build();

        init::LateResources {
            blue,
            green,
            configured: AtomicBool::new(false),
            radio,
            usb_dev,
            serial_port,
        }
    }

    #[idle(resources = [blue, &configured, radio, serial_port])]
    fn idle(cx: idle::Context) -> ! {
        let blue = cx.resources.blue;
        let configured = cx.resources.configured;
        let mut serial_port = cx.resources.serial_port;
        let radio = cx.resources.radio;

        // TODO run Radio in parallel to the USBD task
        let mut packet = Packet::new();
        let mut buf = String::<heapless::consts::U64>::new();
        loop {
            // let res = radio.recv(&mut packet);
            // let lqi = packet.lqi();
            // let len = packet.len();
            // if res.is_ok() {
            //     if blue.is_set_low() == Ok(true) {
            //         blue.set_high().ok();
            //     } else {
            //         blue.set_low().ok();
            //     }

            //     // loopback the packet
            //     radio.send(&packet);
            // }

            let is_configured = configured.load(Ordering::Relaxed);

            if is_configured {
                blue.set_low().ok();
                // buf.clear();

                // // TODO switch to a single `uwriteln!`
                // if res.is_ok() {
                //     buf.push_str("CRC OK").ok()
                // } else {
                //     buf.push_str("BAD CRC").ok()
                // };
                // buf.push_str(" - LQI: ").ok();
                // write!(buf, "{}", lqi).ok();
                // buf.push_str(" - payload: ").ok();
                // write!(buf, "{}", len).ok();
                // buf.push_str(" bytes\n").ok();

                // serial_port.lock(|port| port.write(buf.as_bytes()).ok());
            }
        }
    }

    #[task(binds = USBD, resources = [&configured, green, usb_dev, serial_port])]
    fn usbd(cx: usbd::Context) {
        let configured = cx.resources.configured;
        let green = cx.resources.green;
        let serial_port = cx.resources.serial_port;
        let usb_dev = cx.resources.usb_dev;

        usb_dev.poll(&mut [serial_port]);
        if !configured.load(Ordering::Relaxed) {
            if usb_dev.state() == UsbDeviceState::Configured {
                // off
                green.set_high().ok();
                configured.store(true, Ordering::Relaxed);
                let _ = serial_port.write(b"bChannel: 20\nTX power: +8 dBm\n");
            }
        } else {
            // blink until configured
            if green.is_set_low() == Ok(true) {
                green.set_high().ok();
            } else {
                green.set_low().ok();
            }
        }

        // FIXME `hal::Usbd` is not clearing all EVENTS?
        let usbd: USBD = unsafe { core::mem::transmute(()) };
        if usbd.events_epdata.read().bits() != 0 && usbd.epdatastatus.read().bits() == 0 {
            usbd.events_epdata.reset();
        }
        if usbd.events_sof.read().bits() != 0 {
            usbd.events_sof.reset();
        }
        // usbd.events_endepout[0].reset();
        // usbd.events_endepout[1].reset();
        // usbd.events_endepout[2].reset();
        // usbd.events_ep0setup.reset();
        // usbd.events_ep0datadone.reset();
        // usbd.events_epdata.reset();
        // usbd.events_usbevent.reset();
        // usbd.events_usbreset.reset();
    }
};
