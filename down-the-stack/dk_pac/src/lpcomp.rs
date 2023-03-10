#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start comparator"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop comparator"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Sample comparator value"]
    pub tasks_sample: TASKS_SAMPLE,
    _reserved3: [u8; 0xf4],
    #[doc = "0x100 - LPCOMP is ready and output is valid"]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Downward crossing"]
    pub events_down: EVENTS_DOWN,
    #[doc = "0x108 - Upward crossing"]
    pub events_up: EVENTS_UP,
    #[doc = "0x10c - Downward or upward crossing"]
    pub events_cross: EVENTS_CROSS,
    _reserved7: [u8; 0xf0],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved8: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 0xf4],
    #[doc = "0x400 - Compare result"]
    pub result: RESULT,
    _reserved11: [u8; 0xfc],
    #[doc = "0x500 - Enable LPCOMP"]
    pub enable: ENABLE,
    #[doc = "0x504 - Input pin select"]
    pub psel: PSEL,
    #[doc = "0x508 - Reference select"]
    pub refsel: REFSEL,
    #[doc = "0x50c - External reference select"]
    pub extrefsel: EXTREFSEL,
    _reserved15: [u8; 0x10],
    #[doc = "0x520 - Analog detect configuration"]
    pub anadetect: ANADETECT,
    _reserved16: [u8; 0x14],
    #[doc = "0x538 - Comparator hysteresis enable"]
    pub hyst: HYST,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start comparator"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop comparator"]
pub mod tasks_stop;
#[doc = "TASKS_SAMPLE (w) register accessor: an alias for `Reg<TASKS_SAMPLE_SPEC>`"]
pub type TASKS_SAMPLE = crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>;
#[doc = "Sample comparator value"]
pub mod tasks_sample;
#[doc = "EVENTS_READY (rw) register accessor: an alias for `Reg<EVENTS_READY_SPEC>`"]
pub type EVENTS_READY = crate::Reg<events_ready::EVENTS_READY_SPEC>;
#[doc = "LPCOMP is ready and output is valid"]
pub mod events_ready;
#[doc = "EVENTS_DOWN (rw) register accessor: an alias for `Reg<EVENTS_DOWN_SPEC>`"]
pub type EVENTS_DOWN = crate::Reg<events_down::EVENTS_DOWN_SPEC>;
#[doc = "Downward crossing"]
pub mod events_down;
#[doc = "EVENTS_UP (rw) register accessor: an alias for `Reg<EVENTS_UP_SPEC>`"]
pub type EVENTS_UP = crate::Reg<events_up::EVENTS_UP_SPEC>;
#[doc = "Upward crossing"]
pub mod events_up;
#[doc = "EVENTS_CROSS (rw) register accessor: an alias for `Reg<EVENTS_CROSS_SPEC>`"]
pub type EVENTS_CROSS = crate::Reg<events_cross::EVENTS_CROSS_SPEC>;
#[doc = "Downward or upward crossing"]
pub mod events_cross;
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
#[doc = "RESULT (r) register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "Compare result"]
pub mod result;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable LPCOMP"]
pub mod enable;
#[doc = "PSEL (rw) register accessor: an alias for `Reg<PSEL_SPEC>`"]
pub type PSEL = crate::Reg<psel::PSEL_SPEC>;
#[doc = "Input pin select"]
pub mod psel;
#[doc = "REFSEL (rw) register accessor: an alias for `Reg<REFSEL_SPEC>`"]
pub type REFSEL = crate::Reg<refsel::REFSEL_SPEC>;
#[doc = "Reference select"]
pub mod refsel;
#[doc = "EXTREFSEL (rw) register accessor: an alias for `Reg<EXTREFSEL_SPEC>`"]
pub type EXTREFSEL = crate::Reg<extrefsel::EXTREFSEL_SPEC>;
#[doc = "External reference select"]
pub mod extrefsel;
#[doc = "ANADETECT (rw) register accessor: an alias for `Reg<ANADETECT_SPEC>`"]
pub type ANADETECT = crate::Reg<anadetect::ANADETECT_SPEC>;
#[doc = "Analog detect configuration"]
pub mod anadetect;
#[doc = "HYST (rw) register accessor: an alias for `Reg<HYST_SPEC>`"]
pub type HYST = crate::Reg<hyst::HYST_SPEC>;
#[doc = "Comparator hysteresis enable"]
pub mod hyst;
