#![no_main]
#![no_std]

use cortex_m::asm;
use dk::{peripheral::USBD, usbd};
use panic_log as _; // panic handler

#[rtfm::app(device = dk)]
const APP: () = {
    struct Resources {
        usbd: USBD,
    }

    #[init]
    fn init(_cx: init::Context) -> init::LateResources {
        let board = dk::init().unwrap();

        // initialize the USB peripheral; will block until the USB cable is physically connected
        usbd::init(board.power, &board.usbd);

        // electrically connects the device to the host
        usbd::connect(&board.usbd);

        init::LateResources { usbd: board.usbd }
    }

    #[task(binds = USBD, resources = [usbd])]
    fn usb(cx: usb::Context) {
        let usbd = cx.resources.usbd;

        log::info!("USB event occurred");

        // electrically disconnects the device to the host
        usbd::disconnect(usbd);

        asm::bkpt();
    }
};
