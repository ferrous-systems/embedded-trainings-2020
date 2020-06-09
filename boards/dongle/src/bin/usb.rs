//! Test program used to debug the `serial-term` tool

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin as _;
use hal::{
    clocks::Clocks,
    gpio::{p0, Level},
    target as pac,
    usbd::Usbd,
};
use panic_halt as _;
use usb_device::device::{UsbDeviceBuilder, UsbDeviceState, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    static mut EP_BUF: [u8; 1024] = [0; 1024];

    let periph = pac::Peripherals::take().unwrap();

    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();

    let port = p0::Parts::new(periph.P0);

    // NOTE LEDs turn on when the output level is low (0V)
    let mut green = port.p0_06.into_push_pull_output(Level::High); // LD1
    let mut blue = port.p0_12.into_push_pull_output(Level::High); // LD2 (RGB LED)
    let mut red = port.p0_08.into_push_pull_output(Level::High); // LD2 (RGB LED)

    // these loops should always terminate
    // detected USB power? (device is USB powered)
    while !periph
        .POWER
        .usbregstatus
        .read()
        .vbusdetect()
        .is_vbus_present()
    {
        continue;
    }

    // wait until USB 3.3V supply (internal regulator) is stable
    while !periph
        .POWER
        .events_usbpwrrdy
        .read()
        .events_usbpwrrdy()
        .bit_is_clear()
    {
        continue;
    }

    // turn on green LED to show the program is working
    green.set_low().ok();

    let usb_bus = Usbd::new_alloc(periph.USBD, EP_BUF, &clocks);
    let mut serial_port = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(consts::VID, consts::PID))
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64) // (makes control transfers 8x faster)
        .build();

    let mut buf = [0; 64];
    let mut once = true;
    loop {
        if !usb_dev.poll(&mut [&mut serial_port]) {
            continue;
        }

        if usb_dev.state() == UsbDeviceState::Configured {
            // turn on the blue LED to show the device has been configured
            blue.set_low().ok();

            if once {
                once = false;

                serial_port.write(b"Hello, world!\n").ok();
                serial_port.flush().ok();
            }

            if serial_port.read(&mut buf).is_ok() {
                // received data from the PC. I'm interpreting this as an error but it may actually
                // be part of the CDC/ACM protocol? -- we never send data to the device
                red.set_low().ok();
            }
        } else {
            // otherwise turn it off
            blue.set_high().ok();
        }
    }
}
