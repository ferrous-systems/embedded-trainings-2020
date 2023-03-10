#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start Timer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop Timer"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Increment Timer (Counter mode only)"]
    pub tasks_count: TASKS_COUNT,
    #[doc = "0x0c - Clear time"]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x10 - Deprecated register - Shut down timer"]
    pub tasks_shutdown: TASKS_SHUTDOWN,
    _reserved5: [u8; 0x2c],
    #[doc = "0x40..0x58 - Description collection: Capture Timer value to CC\\[n\\]
register"]
    pub tasks_capture: [TASKS_CAPTURE; 6],
    _reserved6: [u8; 0xe8],
    #[doc = "0x140..0x158 - Description collection: Compare event on CC\\[n\\]
match"]
    pub events_compare: [EVENTS_COMPARE; 6],
    _reserved7: [u8; 0xa8],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved8: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 0x01f8],
    #[doc = "0x504 - Timer mode selection"]
    pub mode: MODE,
    #[doc = "0x508 - Configure the number of bits used by the TIMER"]
    pub bitmode: BITMODE,
    _reserved12: [u8; 0x04],
    #[doc = "0x510 - Timer prescaler register"]
    pub prescaler: PRESCALER,
    _reserved13: [u8; 0x2c],
    #[doc = "0x540..0x558 - Description collection: Capture/Compare register n"]
    pub cc: [CC; 6],
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start Timer"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop Timer"]
pub mod tasks_stop;
#[doc = "TASKS_COUNT (w) register accessor: an alias for `Reg<TASKS_COUNT_SPEC>`"]
pub type TASKS_COUNT = crate::Reg<tasks_count::TASKS_COUNT_SPEC>;
#[doc = "Increment Timer (Counter mode only)"]
pub mod tasks_count;
#[doc = "TASKS_CLEAR (w) register accessor: an alias for `Reg<TASKS_CLEAR_SPEC>`"]
pub type TASKS_CLEAR = crate::Reg<tasks_clear::TASKS_CLEAR_SPEC>;
#[doc = "Clear time"]
pub mod tasks_clear;
#[doc = "TASKS_SHUTDOWN (w) register accessor: an alias for `Reg<TASKS_SHUTDOWN_SPEC>`"]
pub type TASKS_SHUTDOWN = crate::Reg<tasks_shutdown::TASKS_SHUTDOWN_SPEC>;
#[doc = "Deprecated register - Shut down timer"]
pub mod tasks_shutdown;
#[doc = "TASKS_CAPTURE (w) register accessor: an alias for `Reg<TASKS_CAPTURE_SPEC>`"]
pub type TASKS_CAPTURE = crate::Reg<tasks_capture::TASKS_CAPTURE_SPEC>;
#[doc = "Description collection: Capture Timer value to CC\\[n\\]
register"]
pub mod tasks_capture;
#[doc = "EVENTS_COMPARE (rw) register accessor: an alias for `Reg<EVENTS_COMPARE_SPEC>`"]
pub type EVENTS_COMPARE = crate::Reg<events_compare::EVENTS_COMPARE_SPEC>;
#[doc = "Description collection: Compare event on CC\\[n\\]
match"]
pub mod events_compare;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Timer mode selection"]
pub mod mode;
#[doc = "BITMODE (rw) register accessor: an alias for `Reg<BITMODE_SPEC>`"]
pub type BITMODE = crate::Reg<bitmode::BITMODE_SPEC>;
#[doc = "Configure the number of bits used by the TIMER"]
pub mod bitmode;
#[doc = "PRESCALER (rw) register accessor: an alias for `Reg<PRESCALER_SPEC>`"]
pub type PRESCALER = crate::Reg<prescaler::PRESCALER_SPEC>;
#[doc = "Timer prescaler register"]
pub mod prescaler;
#[doc = "CC (rw) register accessor: an alias for `Reg<CC_SPEC>`"]
pub type CC = crate::Reg<cc::CC_SPEC>;
#[doc = "Description collection: Capture/Compare register n"]
pub mod cc;
