//! Board Support Crate (BSC) for the nRF52840 Development Kit

#![deny(missing_docs)]
#![no_std]

use core::{
    ops,
    fmt,
    sync::atomic::{self, Ordering},
    time::Duration,
};

use cortex_m::asm;
use embedded_hal::digital::v2::{OutputPin as _, StatefulOutputPin};
use nrf52840_hal as hal;
pub use hal::pac::uarte0::{
    baudrate::BAUDRATE_A as Baudrate, config::PARITY_A as Parity};


use hal::{
gpio::{p0, Level, Output, Input, PullUp, Pin, Port, PushPull},
timer::OneShot, prelude::InputPin,
uarte,
};

use defmt;
use defmt_rtt as _; // global logger

/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    // 🔽 ---  Button Exercise --- 🔽
    /// Buttons
    pub buttons: Buttons,
    // 🔼  --- Button Exercise --- 🔼 
    /// Timer
    pub timer: Timer,
    // 🔽 ---  UARTE Exercise --- 🔽
    /// uarte interface
    pub uarte: Uarte,
    // 🔼  --- UARTE Exercise --- 🔼 
}

/// All LEDs on the board
pub struct Leds {
    /// LED1: pin P0.13, green LED
    pub led_1: Led,
    /// LED2: pin P0.14, green LED
    pub led_2: Led,
    /// LED3: pin P0.15, green LED
    pub led_3: Led,
    /// LED4: pin P0.16, green LED
    pub led_4: Led,
}

/// A single LED
pub struct Led {
    inner: Pin<Output<PushPull>>,
}

impl Led {
    /// Turns on the LED
    pub fn on(&mut self) {
        defmt::trace!(
            "setting P{}.{} low (LED on)",
            port_as_char(&self.inner.port()),
            self.inner.pin()
        );

        // NOTE this operations returns a `Result` but never returns the `Err` variant
        let _ = self.inner.set_low();
    }

    /// Turns off the LED
    pub fn off(&mut self) {
        defmt::trace!(
            "setting P{}.{} high (LED off)",
            port_as_char(&self.inner.port()),
            self.inner.pin()
        );

        // NOTE this operations returns a `Result` but never returns the `Err` variant
        let _ = self.inner.set_high();
    }

    /// Returns `true` if the LED is in the OFF state
    pub fn is_off(&self) -> bool {
        self.inner.is_set_high() == Ok(true)
    }

    /// Returns `true` if the LED is in the ON state
    pub fn is_on(&self) -> bool {
        !self.is_off()
    }

    /// Toggles the state (on/off) of the LED
    pub fn toggle(&mut self) {
        if self.is_off() {
            self.on();
        } else {
            self.off()
        }
    }
}
// 🔽 ---  Button Exercise --- 🔽
/// All buttons on the board
pub struct Buttons {
    /// BUTTON1: pin P0.11
    pub b_1: Button,
    /// BUTTON2: pin P0.12
    pub b_2: Button,
    /// BUTTON3: pin P0.24
    pub b_3: Button,
    /// BUTTON4: pin P0.25
    pub b_4: Button,
}

/// A single button
pub struct Button {
    inner: Pin<Input<PullUp>>,
}

impl Button {
    /// returns true if button is pushed 
    pub fn is_pushed(&self) -> bool {
        self.inner.is_low() == Ok(true)
    }
}
// 🔼  --- Button Exercise --- 🔼 

/// A timer for creating blocking delays
pub struct Timer {
    inner: hal::Timer<hal::pac::TIMER0, OneShot>,
}


impl Timer {
    /// Blocks program execution for at least the specified `duration`
    pub fn wait(&mut self, duration: Duration) {
        defmt::trace!("blocking for {:?} ...", duration);

        // 1 cycle = 1 microsecond because the underlying HAL driver
        // always sets the timer to 1 MHz.
        const NANOS_IN_ONE_MICRO: u32 = 1_000;
        let subsec_micros = duration.subsec_nanos() / NANOS_IN_ONE_MICRO;
        if subsec_micros != 0 {
            self.inner.delay(subsec_micros);
        }

        const MICROS_IN_ONE_SEC: u32 = 1_000_000;
        // maximum number of seconds that fit in a single `delay` call without overflowing the `u32`
        // argument
        const MAX_SECS: u32 = u32::MAX / MICROS_IN_ONE_SEC;
        let mut secs = duration.as_secs();
        while secs != 0 {
            let cycles = if secs > MAX_SECS as u64 {
                secs -= MAX_SECS as u64;
                MAX_SECS * MICROS_IN_ONE_SEC
            } else {
                let cycles = secs as u32 * MICROS_IN_ONE_SEC;
                secs = 0;
                cycles
            };

            self.inner.delay(cycles)
        }

        defmt::trace!("... DONE");
    }
}

impl ops::Deref for Timer {
    type Target = hal::Timer<hal::pac::TIMER0, OneShot>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl ops::DerefMut for Timer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

// 🔽 ---  UARTE Exercise --- 🔽
/// Uarte peripheral
pub struct Uarte {
    inner: hal::Uarte<hal::pac::UARTE1>,
}

impl fmt::Write for Uarte {

    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Copy all data into an on-stack buffer so we never try to EasyDMA from
        // flash.
        let mut buf: [u8; 16] = [0; 16];
        for block in s.as_bytes().chunks(16) {
            buf[..block.len()].copy_from_slice(block);
            self.inner.write(&buf[..block.len()]).map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}
// 🔼  --- UARTE Exercise --- 🔼 

/// Initializes the board
///
/// This return an `Err`or if called more than once
pub fn init() -> Result<Board, ()> {
    if let Some(periph) = hal::pac::Peripherals::take() {

        let pins = p0::Parts::new(periph.P0);

        // NOTE LEDs turn on when the pin output level is low
        let led_1 = pins.p0_13.degrade().into_push_pull_output(Level::High);
        let led_2 = pins.p0_14.degrade().into_push_pull_output(Level::High);
        let led_3 = pins.p0_15.degrade().into_push_pull_output(Level::High);
        let led_4 = pins.p0_16.degrade().into_push_pull_output(Level::High);
        
        // 🔽 ---  Button Exercise --- 🔽
        // Buttons
        let b_1 = pins.p0_11.degrade().into_pullup_input();
        let b_2 = pins.p0_12.degrade().into_pullup_input();
        let b_3 = pins.p0_24.degrade().into_pullup_input();
        let b_4 = pins.p0_25.degrade().into_pullup_input();
        // 🔼  --- Button Exercise --- 🔼 

        defmt::debug!("I/O pins have been configured for digital output");

        let timer = hal::Timer::new(periph.TIMER0);

        // 🔽 ---  UARTE Exercise --- 🔽
        // Uarte
        let pins =  hal::uarte::Pins {
            rxd: pins.p0_08.degrade().into_floating_input(),
            txd: pins.p0_06.degrade().into_push_pull_output(Level::High),
            cts: Some(pins.p0_07.degrade().into_floating_input()),
            rts: Some(pins.p0_05.degrade().into_push_pull_output(Level::High)),
        };
       

        let uarte = hal::uarte::Uarte::new(periph.UARTE1, pins, Parity::INCLUDED, Baudrate::BAUD115200);
        // 🔼  --- UARTE Exercise --- 🔼 

        Ok(Board {
            leds: Leds {
                led_1: Led { inner: led_1 },
                led_2: Led { inner: led_2 },
                led_3: Led { inner: led_3 },
                led_4: Led { inner: led_4 },
            },

            // 🔽 ---  Button Exercise --- 🔽
            buttons: Buttons {
                b_1: Button { inner: b_1},
                b_2: Button { inner: b_2},
                b_3: Button { inner: b_3},
                b_4: Button { inner: b_4},
            },
            // 🔼  --- Button Exercise --- 🔼 

            timer: Timer { inner: timer },

            // 🔽 ---  UARTE Exercise --- 🔽
            uarte: Uarte { inner: uarte },
            // 🔼  --- UARTE Exercise --- 🔼 
        })
    } else {
        Err(())
    }
}

/// Exits the application when the program is executed through the `probe-run` Cargo runner
pub fn exit() -> ! {
    unsafe {
        // turn off the USB D+ pull-up before pausing the device with a breakpoint
        // this disconnects the nRF device from the USB host so the USB host won't attempt further
        // USB communication (and see an unresponsive device). probe-run will also reset the nRF's
        // USBD peripheral when it sees the device in a halted state which has the same effect as
        // this line but that can take a while and the USB host may issue a power cycle of the USB
        // port / hub / root in the meantime, which can bring down the probe and break probe-run
        const USBD_USBPULLUP: *mut u32 = 0x4002_7504 as *mut u32;
        USBD_USBPULLUP.write_volatile(0)
    }
    defmt::println!("`dk::exit()` called; exiting ...");
    // force any pending memory operation to complete before the BKPT instruction that follows
    atomic::compiler_fence(Ordering::SeqCst);
    loop {
        asm::bkpt()
    }
}

// Helper functions

fn port_as_char(port: &Port) -> char {
    match port {
        Port::Port0 => '0',
        Port::Port1 => '1',
    }
}
