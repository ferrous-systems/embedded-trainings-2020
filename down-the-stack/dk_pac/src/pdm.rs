#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous PDM transfer"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stops PDM transfer"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - PDM transfer has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - PDM transfer has finished"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
    pub events_end: EVENTS_END,
    _reserved5: [u8; 0x01f4],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 0x01f4],
    #[doc = "0x500 - PDM module enable register"]
    pub enable: ENABLE,
    #[doc = "0x504 - PDM clock generator control"]
    pub pdmclkctrl: PDMCLKCTRL,
    #[doc = "0x508 - Defines the routing of the connected PDM microphones' signals"]
    pub mode: MODE,
    _reserved11: [u8; 0x0c],
    #[doc = "0x518 - Left output gain adjustment"]
    pub gainl: GAINL,
    #[doc = "0x51c - Right output gain adjustment"]
    pub gainr: GAINR,
    _reserved13: [u8; 0x20],
    #[doc = "0x540..0x548 - Unspecified"]
    pub psel: PSEL,
    _reserved14: [u8; 0x18],
    #[doc = "0x560..0x568 - Unspecified"]
    pub sample: SAMPLE,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts continuous PDM transfer"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops PDM transfer"]
pub mod tasks_stop;
#[doc = "EVENTS_STARTED (rw) register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "PDM transfer has started"]
pub mod events_started;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "PDM transfer has finished"]
pub mod events_stopped;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "The PDM has written the last sample specified by SAMPLE.MAXCNT (or the last sample after a STOP task has been received) to Data RAM"]
pub mod events_end;
#[doc = "INTEN (rw) register accessor: an alias for `Reg<INTEN_SPEC>`"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "PDM module enable register"]
pub mod enable;
#[doc = "PDMCLKCTRL (rw) register accessor: an alias for `Reg<PDMCLKCTRL_SPEC>`"]
pub type PDMCLKCTRL = crate::Reg<pdmclkctrl::PDMCLKCTRL_SPEC>;
#[doc = "PDM clock generator control"]
pub mod pdmclkctrl;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Defines the routing of the connected PDM microphones' signals"]
pub mod mode;
#[doc = "GAINL (rw) register accessor: an alias for `Reg<GAINL_SPEC>`"]
pub type GAINL = crate::Reg<gainl::GAINL_SPEC>;
#[doc = "Left output gain adjustment"]
pub mod gainl;
#[doc = "GAINR (rw) register accessor: an alias for `Reg<GAINR_SPEC>`"]
pub type GAINR = crate::Reg<gainr::GAINR_SPEC>;
#[doc = "Right output gain adjustment"]
pub mod gainr;
#[doc = "Unspecified"]
pub use self::psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Unspecified"]
pub use self::sample::SAMPLE;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod sample;
