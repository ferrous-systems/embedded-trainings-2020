#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start HFCLK crystal oscillator"]
    pub tasks_hfclkstart: TASKS_HFCLKSTART,
    #[doc = "0x04 - Stop HFCLK crystal oscillator"]
    pub tasks_hfclkstop: TASKS_HFCLKSTOP,
    #[doc = "0x08 - Start LFCLK source"]
    pub tasks_lfclkstart: TASKS_LFCLKSTART,
    #[doc = "0x0c - Stop LFCLK source"]
    pub tasks_lfclkstop: TASKS_LFCLKSTOP,
    #[doc = "0x10 - Start calibration of LFRC oscillator"]
    pub tasks_cal: TASKS_CAL,
    #[doc = "0x14 - Start calibration timer"]
    pub tasks_ctstart: TASKS_CTSTART,
    #[doc = "0x18 - Stop calibration timer"]
    pub tasks_ctstop: TASKS_CTSTOP,
    _reserved7: [u8; 0xe4],
    #[doc = "0x100 - HFCLK oscillator started"]
    pub events_hfclkstarted: EVENTS_HFCLKSTARTED,
    #[doc = "0x104 - LFCLK started"]
    pub events_lfclkstarted: EVENTS_LFCLKSTARTED,
    _reserved9: [u8; 0x04],
    #[doc = "0x10c - Calibration of LFCLK RC oscillator complete event"]
    pub events_done: EVENTS_DONE,
    #[doc = "0x110 - Calibration timer timeout"]
    pub events_ctto: EVENTS_CTTO,
    _reserved11: [u8; 0x01f0],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved13: [u8; 0xfc],
    #[doc = "0x408 - Status indicating that HFCLKSTART task has been triggered"]
    pub hfclkrun: HFCLKRUN,
    #[doc = "0x40c - HFCLK status"]
    pub hfclkstat: HFCLKSTAT,
    _reserved15: [u8; 0x04],
    #[doc = "0x414 - Status indicating that LFCLKSTART task has been triggered"]
    pub lfclkrun: LFCLKRUN,
    #[doc = "0x418 - LFCLK status"]
    pub lfclkstat: LFCLKSTAT,
    #[doc = "0x41c - Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
    pub lfclksrccopy: LFCLKSRCCOPY,
    _reserved18: [u8; 0xf8],
    #[doc = "0x518 - Clock source for the LFCLK"]
    pub lfclksrc: LFCLKSRC,
    _reserved19: [u8; 0x1c],
    #[doc = "0x538 - Calibration timer interval"]
    pub ctiv: CTIV,
    _reserved20: [u8; 0x20],
    #[doc = "0x55c - Clocking options for the Trace Port debug interface"]
    pub traceconfig: TRACECONFIG,
}
#[doc = "TASKS_HFCLKSTART (w) register accessor: an alias for `Reg<TASKS_HFCLKSTART_SPEC>`"]
pub type TASKS_HFCLKSTART = crate::Reg<tasks_hfclkstart::TASKS_HFCLKSTART_SPEC>;
#[doc = "Start HFCLK crystal oscillator"]
pub mod tasks_hfclkstart;
#[doc = "TASKS_HFCLKSTOP (w) register accessor: an alias for `Reg<TASKS_HFCLKSTOP_SPEC>`"]
pub type TASKS_HFCLKSTOP = crate::Reg<tasks_hfclkstop::TASKS_HFCLKSTOP_SPEC>;
#[doc = "Stop HFCLK crystal oscillator"]
pub mod tasks_hfclkstop;
#[doc = "TASKS_LFCLKSTART (w) register accessor: an alias for `Reg<TASKS_LFCLKSTART_SPEC>`"]
pub type TASKS_LFCLKSTART = crate::Reg<tasks_lfclkstart::TASKS_LFCLKSTART_SPEC>;
#[doc = "Start LFCLK source"]
pub mod tasks_lfclkstart;
#[doc = "TASKS_LFCLKSTOP (w) register accessor: an alias for `Reg<TASKS_LFCLKSTOP_SPEC>`"]
pub type TASKS_LFCLKSTOP = crate::Reg<tasks_lfclkstop::TASKS_LFCLKSTOP_SPEC>;
#[doc = "Stop LFCLK source"]
pub mod tasks_lfclkstop;
#[doc = "TASKS_CAL (w) register accessor: an alias for `Reg<TASKS_CAL_SPEC>`"]
pub type TASKS_CAL = crate::Reg<tasks_cal::TASKS_CAL_SPEC>;
#[doc = "Start calibration of LFRC oscillator"]
pub mod tasks_cal;
#[doc = "TASKS_CTSTART (w) register accessor: an alias for `Reg<TASKS_CTSTART_SPEC>`"]
pub type TASKS_CTSTART = crate::Reg<tasks_ctstart::TASKS_CTSTART_SPEC>;
#[doc = "Start calibration timer"]
pub mod tasks_ctstart;
#[doc = "TASKS_CTSTOP (w) register accessor: an alias for `Reg<TASKS_CTSTOP_SPEC>`"]
pub type TASKS_CTSTOP = crate::Reg<tasks_ctstop::TASKS_CTSTOP_SPEC>;
#[doc = "Stop calibration timer"]
pub mod tasks_ctstop;
#[doc = "EVENTS_HFCLKSTARTED (rw) register accessor: an alias for `Reg<EVENTS_HFCLKSTARTED_SPEC>`"]
pub type EVENTS_HFCLKSTARTED = crate::Reg<events_hfclkstarted::EVENTS_HFCLKSTARTED_SPEC>;
#[doc = "HFCLK oscillator started"]
pub mod events_hfclkstarted;
#[doc = "EVENTS_LFCLKSTARTED (rw) register accessor: an alias for `Reg<EVENTS_LFCLKSTARTED_SPEC>`"]
pub type EVENTS_LFCLKSTARTED = crate::Reg<events_lfclkstarted::EVENTS_LFCLKSTARTED_SPEC>;
#[doc = "LFCLK started"]
pub mod events_lfclkstarted;
#[doc = "EVENTS_DONE (rw) register accessor: an alias for `Reg<EVENTS_DONE_SPEC>`"]
pub type EVENTS_DONE = crate::Reg<events_done::EVENTS_DONE_SPEC>;
#[doc = "Calibration of LFCLK RC oscillator complete event"]
pub mod events_done;
#[doc = "EVENTS_CTTO (rw) register accessor: an alias for `Reg<EVENTS_CTTO_SPEC>`"]
pub type EVENTS_CTTO = crate::Reg<events_ctto::EVENTS_CTTO_SPEC>;
#[doc = "Calibration timer timeout"]
pub mod events_ctto;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "HFCLKRUN (r) register accessor: an alias for `Reg<HFCLKRUN_SPEC>`"]
pub type HFCLKRUN = crate::Reg<hfclkrun::HFCLKRUN_SPEC>;
#[doc = "Status indicating that HFCLKSTART task has been triggered"]
pub mod hfclkrun;
#[doc = "HFCLKSTAT (r) register accessor: an alias for `Reg<HFCLKSTAT_SPEC>`"]
pub type HFCLKSTAT = crate::Reg<hfclkstat::HFCLKSTAT_SPEC>;
#[doc = "HFCLK status"]
pub mod hfclkstat;
#[doc = "LFCLKRUN (r) register accessor: an alias for `Reg<LFCLKRUN_SPEC>`"]
pub type LFCLKRUN = crate::Reg<lfclkrun::LFCLKRUN_SPEC>;
#[doc = "Status indicating that LFCLKSTART task has been triggered"]
pub mod lfclkrun;
#[doc = "LFCLKSTAT (r) register accessor: an alias for `Reg<LFCLKSTAT_SPEC>`"]
pub type LFCLKSTAT = crate::Reg<lfclkstat::LFCLKSTAT_SPEC>;
#[doc = "LFCLK status"]
pub mod lfclkstat;
#[doc = "LFCLKSRCCOPY (r) register accessor: an alias for `Reg<LFCLKSRCCOPY_SPEC>`"]
pub type LFCLKSRCCOPY = crate::Reg<lfclksrccopy::LFCLKSRCCOPY_SPEC>;
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered"]
pub mod lfclksrccopy;
#[doc = "LFCLKSRC (rw) register accessor: an alias for `Reg<LFCLKSRC_SPEC>`"]
pub type LFCLKSRC = crate::Reg<lfclksrc::LFCLKSRC_SPEC>;
#[doc = "Clock source for the LFCLK"]
pub mod lfclksrc;
#[doc = "CTIV (rw) register accessor: an alias for `Reg<CTIV_SPEC>`"]
pub type CTIV = crate::Reg<ctiv::CTIV_SPEC>;
#[doc = "Calibration timer interval"]
pub mod ctiv;
#[doc = "TRACECONFIG (rw) register accessor: an alias for `Reg<TRACECONFIG_SPEC>`"]
pub type TRACECONFIG = crate::Reg<traceconfig::TRACECONFIG_SPEC>;
#[doc = "Clocking options for the Trace Port debug interface"]
pub mod traceconfig;
