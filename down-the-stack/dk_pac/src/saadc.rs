#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the ADC and prepare the result buffer in RAM"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Take one ADC sample, if scan is enabled all channels are sampled"]
    pub tasks_sample: TASKS_SAMPLE,
    #[doc = "0x08 - Stop the ADC and terminate any on-going conversion"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x0c - Starts offset auto-calibration"]
    pub tasks_calibrateoffset: TASKS_CALIBRATEOFFSET,
    _reserved4: [u8; 0xf0],
    #[doc = "0x100 - The ADC has started"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x104 - The ADC has filled up the Result buffer"]
    pub events_end: EVENTS_END,
    #[doc = "0x108 - A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
    pub events_done: EVENTS_DONE,
    #[doc = "0x10c - A result is ready to get transferred to RAM."]
    pub events_resultdone: EVENTS_RESULTDONE,
    #[doc = "0x110 - Calibration is complete"]
    pub events_calibratedone: EVENTS_CALIBRATEDONE,
    #[doc = "0x114 - The ADC has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x118..0x158 - Peripheral events."]
    pub events_ch: [EVENTS_CH; 8],
    _reserved11: [u8; 0x01a8],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 0xf4],
    #[doc = "0x400 - Status"]
    pub status: STATUS,
    _reserved15: [u8; 0xfc],
    #[doc = "0x500 - Enable or disable ADC"]
    pub enable: ENABLE,
    _reserved16: [u8; 0x0c],
    #[doc = "0x510..0x590 - Unspecified"]
    pub ch: [CH; 8],
    _reserved17: [u8; 0x60],
    #[doc = "0x5f0 - Resolution configuration"]
    pub resolution: RESOLUTION,
    #[doc = "0x5f4 - Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
    pub oversample: OVERSAMPLE,
    #[doc = "0x5f8 - Controls normal or continuous sample rate"]
    pub samplerate: SAMPLERATE,
    _reserved20: [u8; 0x30],
    #[doc = "0x62c..0x638 - RESULT EasyDMA channel"]
    pub result: RESULT,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start the ADC and prepare the result buffer in RAM"]
pub mod tasks_start;
#[doc = "TASKS_SAMPLE (w) register accessor: an alias for `Reg<TASKS_SAMPLE_SPEC>`"]
pub type TASKS_SAMPLE = crate::Reg<tasks_sample::TASKS_SAMPLE_SPEC>;
#[doc = "Take one ADC sample, if scan is enabled all channels are sampled"]
pub mod tasks_sample;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop the ADC and terminate any on-going conversion"]
pub mod tasks_stop;
#[doc = "TASKS_CALIBRATEOFFSET (w) register accessor: an alias for `Reg<TASKS_CALIBRATEOFFSET_SPEC>`"]
pub type TASKS_CALIBRATEOFFSET = crate::Reg<tasks_calibrateoffset::TASKS_CALIBRATEOFFSET_SPEC>;
#[doc = "Starts offset auto-calibration"]
pub mod tasks_calibrateoffset;
#[doc = "EVENTS_STARTED (rw) register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "The ADC has started"]
pub mod events_started;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "The ADC has filled up the Result buffer"]
pub mod events_end;
#[doc = "EVENTS_DONE (rw) register accessor: an alias for `Reg<EVENTS_DONE_SPEC>`"]
pub type EVENTS_DONE = crate::Reg<events_done::EVENTS_DONE_SPEC>;
#[doc = "A conversion task has been completed. Depending on the mode, multiple conversions might be needed for a result to be transferred to RAM."]
pub mod events_done;
#[doc = "EVENTS_RESULTDONE (rw) register accessor: an alias for `Reg<EVENTS_RESULTDONE_SPEC>`"]
pub type EVENTS_RESULTDONE = crate::Reg<events_resultdone::EVENTS_RESULTDONE_SPEC>;
#[doc = "A result is ready to get transferred to RAM."]
pub mod events_resultdone;
#[doc = "EVENTS_CALIBRATEDONE (rw) register accessor: an alias for `Reg<EVENTS_CALIBRATEDONE_SPEC>`"]
pub type EVENTS_CALIBRATEDONE = crate::Reg<events_calibratedone::EVENTS_CALIBRATEDONE_SPEC>;
#[doc = "Calibration is complete"]
pub mod events_calibratedone;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "The ADC has stopped"]
pub mod events_stopped;
#[doc = "Peripheral events."]
pub use self::events_ch::EVENTS_CH;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_ch;
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
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable or disable ADC"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ch;
#[doc = "RESOLUTION (rw) register accessor: an alias for `Reg<RESOLUTION_SPEC>`"]
pub type RESOLUTION = crate::Reg<resolution::RESOLUTION_SPEC>;
#[doc = "Resolution configuration"]
pub mod resolution;
#[doc = "OVERSAMPLE (rw) register accessor: an alias for `Reg<OVERSAMPLE_SPEC>`"]
pub type OVERSAMPLE = crate::Reg<oversample::OVERSAMPLE_SPEC>;
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used."]
pub mod oversample;
#[doc = "SAMPLERATE (rw) register accessor: an alias for `Reg<SAMPLERATE_SPEC>`"]
pub type SAMPLERATE = crate::Reg<samplerate::SAMPLERATE_SPEC>;
#[doc = "Controls normal or continuous sample rate"]
pub mod samplerate;
#[doc = "RESULT EasyDMA channel"]
pub use self::result::RESULT;
#[doc = r"Cluster"]
#[doc = "RESULT EasyDMA channel"]
pub mod result;
