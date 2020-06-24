//! Hardware Abstraction Layer (HAL) for the nRF52840 Development Kit

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::{
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};

use cortex_m::asm;
use embedded_hal::digital::v2::{OutputPin as _, StatefulOutputPin};
#[cfg(feature = "beginner")]
pub use hal::ieee802154;
pub use hal::target::{interrupt, Interrupt, NVIC_PRIO_BITS, RTC0};
use hal::{
    clocks::{self, Clocks},
    gpio::{p0, Level, Output, Pin, PushPull},
    rtc::{Rtc, RtcInterrupt},
    timer::OneShot,
};
use log::{LevelFilter, Log};
use rtt_target::{rprintln, rtt_init_print};

#[cfg(feature = "advanced")]
use crate::{
    peripheral::{POWER, USBD},
    usbd::Ep0In,
};

#[cfg(feature = "advanced")]
mod errata;
pub mod peripheral;
#[cfg(feature = "advanced")]
pub mod usbd;

/// Components on the board
pub struct Board {
    /// LEDs
    pub leds: Leds,
    /// Timer
    pub timer: Timer,

    /// Radio interface
    #[cfg(feature = "beginner")]
    pub radio: ieee802154::Radio<'static>,
    /// USBD (Universal Serial Bus Device) peripheral
    #[cfg(feature = "advanced")]
    pub usbd: USBD,
    /// POWER (Power Supply) peripheral
    #[cfg(feature = "advanced")]
    pub power: POWER,
    /// USB control endpoint 0
    #[cfg(feature = "advanced")]
    pub ep0in: Ep0In,
}

/// All LEDs on the board
pub struct Leds {
    /// LED1: pin P0.13, green LED
    pub _1: Led,
    /// LED2: pin P0.14, green LED
    pub _2: Led,
    /// LED3: pin P0.15, green LED
    pub _3: Led,
    /// LED4: pin P0.16, green LED
    pub _4: Led,
}

/// A single LED
pub struct Led {
    inner: Pin<Output<PushPull>>,
}

impl Led {
    /// Turns on the LED
    pub fn on(&mut self) {
        log::trace!(
            "setting P{}.{} low (LED on)",
            if self.inner.port { '1' } else { '0' },
            self.inner.pin
        );

        // NOTE this operations returns a `Result` but never returns the `Err` variant
        let _ = self.inner.set_low();
    }

    /// Turns off the LED
    pub fn off(&mut self) {
        log::trace!(
            "setting P{}.{} high (LED off)",
            if self.inner.port { '1' } else { '0' },
            self.inner.pin
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

/// A timer for blocking delay
pub struct Timer {
    inner: hal::Timer<hal::target::TIMER0, OneShot>,
}

impl Timer {
    /// Blocks program execution for at least the specified `duration`
    pub fn wait(&mut self, duration: Duration) {
        log::trace!("blocking for {:?} ...", duration);

        // 1 cycle = 1 microsecond
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

        log::trace!("... DONE");
    }
}

// add Instant API

/// Initializes the board
///
/// This return an `Err`or if called more than once
pub fn init() -> Result<Board, ()> {
    if let (Some(mut core), Some(periph)) = (
        cortex_m::Peripherals::take(),
        hal::target::Peripherals::take(),
    ) {
        // NOTE(static mut) this branch runs at most once
        #[cfg(feature = "advanced")]
        static mut EP0IN_BUF: [u8; 64] = [0; 64];
        #[cfg(feature = "beginner")]
        static mut CLOCKS: Option<
            Clocks<clocks::ExternalOscillator, clocks::ExternalOscillator, clocks::LfOscStarted>,
        > = None;

        // NOTE this must be executed as early as possible or the tool will timeout
        // NOTE the unsafety of this macro is incorrect; it must be run at most once
        #[cfg(feature = "beginner")]
        rtt_init_print!(BlockIfFull, 4096);
        #[cfg(feature = "advanced")]
        rtt_init_print!(NoBlockSkip, 4096);

        log::set_logger(&Logger).unwrap();

        // if not configured in the application we default to the `Info` level
        if log::max_level() == LevelFilter::Off {
            log::set_max_level(LevelFilter::Info)
        }

        log::debug!("Initializing the board");

        let clocks = Clocks::new(periph.CLOCK);
        let clocks = clocks.enable_ext_hfosc();
        let clocks = clocks.set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass);
        let clocks = clocks.start_lfclk();
        let _clocks = clocks.enable_ext_hfosc();
        // extend lifetime to `'static`
        #[cfg(feature = "beginner")]
        let clocks = unsafe { CLOCKS.get_or_insert(_clocks) };

        log::debug!("Clocks configured");

        let mut rtc = Rtc::new(periph.RTC0);
        rtc.enable_interrupt(RtcInterrupt::Overflow, Some(&mut core.NVIC));
        rtc.enable_counter();

        log::debug!("RTC started");

        let pins = p0::Parts::new(periph.P0);

        // NOTE LEDs turn on when the pin output level is low
        let _1 = pins.p0_13.degrade().into_push_pull_output(Level::High);
        let _2 = pins.p0_14.degrade().into_push_pull_output(Level::High);
        let _3 = pins.p0_15.degrade().into_push_pull_output(Level::High);
        let _4 = pins.p0_16.degrade().into_push_pull_output(Level::High);

        log::debug!("I/O pins have been configured for digital output");

        let timer = hal::Timer::new(periph.TIMER0);

        #[cfg(feature = "beginner")]
        let radio = {
            let mut radio = ieee802154::Radio::init(periph.RADIO, clocks);

            // set TX power to its maximum value
            radio.set_txpower(ieee802154::TxPower::Pos8dBm);
            log::debug!("Radio initialized and configured with TX power set to the maximum value");
            radio
        };

        Ok(Board {
            leds: Leds {
                _1: Led { inner: _1 },
                _2: Led { inner: _2 },
                _3: Led { inner: _3 },
                _4: Led { inner: _4 },
            },
            #[cfg(feature = "beginner")]
            radio,
            timer: Timer { inner: timer },
            #[cfg(feature = "advanced")]
            usbd: periph.USBD,
            #[cfg(feature = "advanced")]
            power: periph.POWER,
            #[cfg(feature = "advanced")]
            ep0in: unsafe { Ep0In::new(&mut EP0IN_BUF) },
        })
    } else {
        Err(())
    }
}

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        rprintln!(
            "{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}

// Counter of OVERFLOW events -- an OVERFLOW occurs every (1<<24) ticks
static OVERFLOWS: AtomicU32 = AtomicU32::new(0);

// NOTE this will at the highest priority, higher priority than RTIC tasks
#[interrupt]
fn RTC0() {
    let curr = OVERFLOWS.load(Ordering::Relaxed);
    OVERFLOWS.store(curr + 1, Ordering::Relaxed);

    // clear the EVENT register
    unsafe { core::mem::transmute::<_, RTC0>(()).events_ovrflw.reset() }
}

/// Exits the application and prints a backtrace when the program is executed through the `dk-run`
/// Cargo runner
pub fn exit() -> ! {
    loop {
        asm::bkpt()
    }
}

/// Returns the time elapsed since the call to the `dk::init` function
///
/// The clock that is read to compute this value has a resolution of 30 microseconds.
///
/// Calling this function before calling `dk::init` will return a value of `0` nanoseconds.
pub fn uptime() -> Duration {
    // here we are going to perform a 64-bit read of the number of ticks elapsed
    //
    // a 64-bit load operation cannot performed in a single instruction so the operation can be
    // preempted by the RTC0 interrupt handler (which increases the OVERFLOWS counter)
    //
    // the loop below will load both the lower and upper parts of the 64-bit value while preventing
    // the issue of mixing a low value with an "old" high value -- note that, due to interrupts, an
    // arbitrary amount of time may elapse between the `hi1` load and the `low` load
    let overflows = &OVERFLOWS as *const AtomicU32 as *const u32;
    let ticks = loop {
        unsafe {
            // NOTE volatile is used to order these load operations among themselves
            let hi1 = overflows.read_volatile();
            let low = core::mem::transmute::<_, RTC0>(())
                .counter
                .read()
                .counter()
                .bits();
            let hi2 = overflows.read_volatile();

            if hi1 == hi2 {
                break u64::from(low) | (u64::from(hi1) << 24);
            }
        }
    };

    // 2**15 ticks = 1 second
    let freq = 1 << 15;
    let secs = ticks / freq;
    // subsec ticks
    let ticks = (ticks % freq) as u32;
    // one tick is equal to `1e9 / 32768` nanos
    // the fraction can be reduced to `1953125 / 64`
    // which can be further decomposed as `78125 * (5 / 4) * (5 / 4) * (1 / 4)`.
    // Doing the operation this way we can stick to 32-bit arithmetic without overflowing the value
    // at any stage
    let nanos =
        (((ticks % 32768).wrapping_mul(78125) >> 2).wrapping_mul(5) >> 2).wrapping_mul(5) >> 2;
    Duration::new(secs, nanos as u32)
}
