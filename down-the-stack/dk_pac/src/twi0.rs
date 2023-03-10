#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start TWI receive sequence"]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - Start TWI transmit sequence"]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 0x08],
    #[doc = "0x14 - Stop TWI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 0x04],
    #[doc = "0x1c - Suspend TWI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume TWI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 0xe0],
    #[doc = "0x104 - TWI stopped"]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - TWI RXD byte received"]
    pub events_rxdready: EVENTS_RXDREADY,
    _reserved7: [u8; 0x10],
    #[doc = "0x11c - TWI TXD byte sent"]
    pub events_txdsent: EVENTS_TXDSENT,
    _reserved8: [u8; 0x04],
    #[doc = "0x124 - TWI error"]
    pub events_error: EVENTS_ERROR,
    _reserved9: [u8; 0x10],
    #[doc = "0x138 - TWI byte boundary, generated before each byte that is sent or received"]
    pub events_bb: EVENTS_BB,
    _reserved10: [u8; 0x0c],
    #[doc = "0x148 - TWI entered the suspended state"]
    pub events_suspended: EVENTS_SUSPENDED,
    _reserved11: [u8; 0xb4],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved12: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 0x01b8],
    #[doc = "0x4c4 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 0x38],
    #[doc = "0x500 - Enable TWI"]
    pub enable: ENABLE,
    _reserved16: [u8; 0x04],
    #[doc = "0x508 - Pin select for SCL"]
    pub pselscl: PSELSCL,
    #[doc = "0x50c - Pin select for SDA"]
    pub pselsda: PSELSDA,
    _reserved18: [u8; 0x08],
    #[doc = "0x518 - RXD register"]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register"]
    pub txd: TXD,
    _reserved20: [u8; 0x04],
    #[doc = "0x524 - TWI frequency"]
    pub frequency: FREQUENCY,
    _reserved21: [u8; 0x60],
    #[doc = "0x588 - Address used in the TWI transfer"]
    pub address: ADDRESS,
}
#[doc = "TASKS_STARTRX (w) register accessor: an alias for `Reg<TASKS_STARTRX_SPEC>`"]
pub type TASKS_STARTRX = crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>;
#[doc = "Start TWI receive sequence"]
pub mod tasks_startrx;
#[doc = "TASKS_STARTTX (w) register accessor: an alias for `Reg<TASKS_STARTTX_SPEC>`"]
pub type TASKS_STARTTX = crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>;
#[doc = "Start TWI transmit sequence"]
pub mod tasks_starttx;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop TWI transaction"]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend TWI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: an alias for `Reg<TASKS_RESUME_SPEC>`"]
pub type TASKS_RESUME = crate::Reg<tasks_resume::TASKS_RESUME_SPEC>;
#[doc = "Resume TWI transaction"]
pub mod tasks_resume;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "TWI stopped"]
pub mod events_stopped;
#[doc = "EVENTS_RXDREADY (rw) register accessor: an alias for `Reg<EVENTS_RXDREADY_SPEC>`"]
pub type EVENTS_RXDREADY = crate::Reg<events_rxdready::EVENTS_RXDREADY_SPEC>;
#[doc = "TWI RXD byte received"]
pub mod events_rxdready;
#[doc = "EVENTS_TXDSENT (rw) register accessor: an alias for `Reg<EVENTS_TXDSENT_SPEC>`"]
pub type EVENTS_TXDSENT = crate::Reg<events_txdsent::EVENTS_TXDSENT_SPEC>;
#[doc = "TWI TXD byte sent"]
pub mod events_txdsent;
#[doc = "EVENTS_ERROR (rw) register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "TWI error"]
pub mod events_error;
#[doc = "EVENTS_BB (rw) register accessor: an alias for `Reg<EVENTS_BB_SPEC>`"]
pub type EVENTS_BB = crate::Reg<events_bb::EVENTS_BB_SPEC>;
#[doc = "TWI byte boundary, generated before each byte that is sent or received"]
pub mod events_bb;
#[doc = "EVENTS_SUSPENDED (rw) register accessor: an alias for `Reg<EVENTS_SUSPENDED_SPEC>`"]
pub type EVENTS_SUSPENDED = crate::Reg<events_suspended::EVENTS_SUSPENDED_SPEC>;
#[doc = "TWI entered the suspended state"]
pub mod events_suspended;
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
#[doc = "ERRORSRC (rw) register accessor: an alias for `Reg<ERRORSRC_SPEC>`"]
pub type ERRORSRC = crate::Reg<errorsrc::ERRORSRC_SPEC>;
#[doc = "Error source"]
pub mod errorsrc;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable TWI"]
pub mod enable;
#[doc = "PSELSCL (rw) register accessor: an alias for `Reg<PSELSCL_SPEC>`"]
pub type PSELSCL = crate::Reg<pselscl::PSELSCL_SPEC>;
#[doc = "Pin select for SCL"]
pub mod pselscl;
#[doc = "PSELSDA (rw) register accessor: an alias for `Reg<PSELSDA_SPEC>`"]
pub type PSELSDA = crate::Reg<pselsda::PSELSDA_SPEC>;
#[doc = "Pin select for SDA"]
pub mod pselsda;
#[doc = "RXD (r) register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD (rw) register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "TXD register"]
pub mod txd;
#[doc = "FREQUENCY (rw) register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "TWI frequency"]
pub mod frequency;
#[doc = "ADDRESS (rw) register accessor: an alias for `Reg<ADDRESS_SPEC>`"]
pub type ADDRESS = crate::Reg<address::ADDRESS_SPEC>;
#[doc = "Address used in the TWI transfer"]
pub mod address;
