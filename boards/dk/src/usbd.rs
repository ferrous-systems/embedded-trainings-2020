use core::sync::atomic::{self, Ordering};

use cortex_m::asm;

use crate::{
    errata,
    peripheral::{POWER, USBD},
};

/// Endpoint IN 0
pub struct Ep0In {
    buffer: &'static mut [u8; 64],
    busy: bool,
}

impl Ep0In {
    pub fn new(buffer: &'static mut [u8; 64]) -> Self {
        Self {
            buffer,
            busy: false,
        }
    }

    pub fn start(&mut self, bytes: &[u8], usbd: &USBD) -> Result<(), ()> {
        if self.busy {
            log::error!("EP0IN: last transfer not completed");
            return Err(());
        }

        if bytes.len() > self.buffer.len() {
            log::error!("EP0IN: multi-packet data transfers are not supported");
            return Err(());
        }

        let n = bytes.len();
        self.buffer[..n].copy_from_slice(bytes);

        // use a "shortcut" to issue a status stage after the data transfer is complete
        usbd.shorts
            .modify(|_, w| w.ep0datadone_ep0status().set_bit());
        usbd.epin0
            .maxcnt
            .write(|w| unsafe { w.maxcnt().bits(n as u8) });
        usbd.epin0
            .ptr
            .write(|w| unsafe { w.ptr().bits(self.buffer.as_ptr() as u32) });

        self.busy = true;

        log::info!("EP0IN: start {}B transfer", n);

        // start DMA transfer
        dma_start();
        usbd.tasks_startepin[0].write(|w| w.tasks_startepin().set_bit());

        Ok(())
    }

    pub fn end(&mut self, usbd: &USBD) {
        if usbd.events_ep0datadone.read().bits() == 0 {
            log::error!("Ep0In.end called before the EP0DATADONE event was raised");
            loop {
                asm::bkpt();
            }
        } else {
            // DMA transfer complete
            dma_end();
            usbd.events_ep0datadone.reset();

            self.busy = false;
        }
    }
}

// memory barrier to synchronize the start of a DMA transfer (which will run in parallel) with the
// caller's memory operations
//
// This function call *must* be *followed* by a memory *store* operation. Memory operations that
// follow this function call will *not* be moved, by the compiler or the instruction pipeline, to
// *after* the function call. 
fn dma_start() {
    atomic::fence(Ordering::Release);
}

// memory barrier to synchronize the end of a DMA transfer (which ran in parallel) to the caller's
// memory operations
//
// This function call *must* be *preceded* by a memory *load* operation. Memory operations that
// follow this function call will *not* be moved, by the compiler or the instruction pipeline, to
// *before* the function call. 
fn dma_end() {
    atomic::fence(Ordering::Acquire);
}

// NOTE will be called from user code; at that point the high frequency clock source has already
// been configured to use to the external crystal
// Reference: section 6.35.4 of the nRF52840 Product Specification
pub fn init(power: POWER, usbd: &USBD) {
    // wait until the USB has been connected
    while power.events_usbdetected.read().bits() == 0 {
        power.events_usbdetected.reset();

        continue;
    }

    // workaround silicon bug
    unsafe { errata::e187a() }
    // enable the USB peripheral
    usbd.enable.write(|w| w.enable().set_bit());

    // wait for the peripheral to signal it has reached the READY state
    while usbd.eventcause.read().ready().bit_is_clear() {
        continue;
    }
    // write 1 to clear the flag
    usbd.eventcause.write(|w| w.ready().set_bit());

    // if EVENTCAUSE is all zeroes then also clear the USBEVENT register
    if usbd.eventcause.read().bits() == 0 {
        usbd.events_usbevent.reset();
    }

    // complete the silicon bug workaround
    unsafe { errata::e187b() }

    // also need to wait for the USB power supply regulator to stabilize
    while power.events_usbpwrrdy.read().bits() == 0 {
        continue;
    }
    power.events_usbpwrrdy.reset();

    // before returning unmask the relevant interrupts
    usbd.intenset.write(|w| {
        w.ep0datadone().set_bit();
        w.ep0setup().set_bit();
        w.usbreset().set_bit()
    });
}

/// Connects the device to the host (enables the D+ line pull-up)
pub fn connect(usbd: &USBD) {
    usbd.usbpullup.write(|w| w.connect().set_bit());
}

/// Disconnects the device from the host (disables the D+ line pull-up)
pub fn disconnect(usbd: &USBD) {
    usbd.usbpullup.reset();
}

pub fn ep0stall(usbd: &USBD) {
    usbd.tasks_ep0stall.write(|w| w.tasks_ep0stall().set_bit());
}

/// USBD.EVENTS registers mapped to an enum
#[derive(Debug)]
pub enum Event {
    /// `EVENTS_USBRESET` register was active
    UsbReset,

    /// `EVENTS_EP0DATADONE` register was active
    UsbEp0DataDone,

    /// `EVENTS_EP0SETUP` register was active
    UsbEp0Setup,
}

/// Returns the next unhandled USB event; returns none if there's no event to handle
///
/// NOTE this function will clear the corresponding the EVENT register (*) so the caller should
/// handle the returned event properly. Expect for USBEVENT and EP0DATADONE
pub fn next_event(usbd: &USBD) -> Option<Event> {
    if usbd.events_usbreset.read().bits() != 0 {
        usbd.events_usbreset.reset();

        return Some(Event::UsbReset);
    }

    if usbd.events_ep0datadone.read().bits() != 0 {
        // this will be cleared elsewhere
        // usbd.events_ep0datadone.reset();

        return Some(Event::UsbEp0DataDone);
    }

    if usbd.events_ep0setup.read().bits() != 0 {
        usbd.events_ep0setup.reset();

        return Some(Event::UsbEp0Setup);
    }

    None
}

/// Use this instead of the `todo!` / `unimplemented!` macro
pub fn todo(usbd: &USBD) {
    disconnect(usbd);
    log::error!("unimplemented");
    loop {
        asm::bkpt()
    }
}

pub fn wlength(usbd: &USBD) -> u16 {
    u16::from(usbd.wlengthl.read().wlengthl().bits())
        | u16::from(usbd.wlengthh.read().wlengthh().bits()) << 8
}

pub fn windex(usbd: &USBD) -> u16 {
    u16::from(usbd.windexl.read().windexl().bits())
        | u16::from(usbd.windexh.read().windexh().bits()) << 8
}

pub fn wvalue(usbd: &USBD) -> u16 {
    u16::from(usbd.wvaluel.read().wvaluel().bits())
        | u16::from(usbd.wvalueh.read().wvalueh().bits()) << 8
}
