#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start UART receiver"]
    pub tasks_startrx: TASKS_STARTRX,
    #[doc = "0x04 - Stop UART receiver"]
    pub tasks_stoprx: TASKS_STOPRX,
    #[doc = "0x08 - Start UART transmitter"]
    pub tasks_starttx: TASKS_STARTTX,
    #[doc = "0x0c - Stop UART transmitter"]
    pub tasks_stoptx: TASKS_STOPTX,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - Suspend UART"]
    pub tasks_suspend: TASKS_SUSPEND,
    _reserved5: [u8; 0xe0],
    #[doc = "0x100 - CTS is activated (set low). Clear To Send."]
    pub events_cts: EVENTS_CTS,
    #[doc = "0x104 - CTS is deactivated (set high). Not Clear To Send."]
    pub events_ncts: EVENTS_NCTS,
    #[doc = "0x108 - Data received in RXD"]
    pub events_rxdrdy: EVENTS_RXDRDY,
    _reserved8: [u8; 0x10],
    #[doc = "0x11c - Data sent from TXD"]
    pub events_txdrdy: EVENTS_TXDRDY,
    _reserved9: [u8; 0x04],
    #[doc = "0x124 - Error detected"]
    pub events_error: EVENTS_ERROR,
    _reserved10: [u8; 0x1c],
    #[doc = "0x144 - Receiver timeout"]
    pub events_rxto: EVENTS_RXTO,
    _reserved11: [u8; 0xb8],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved12: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 0x0174],
    #[doc = "0x480 - Error source"]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 0x7c],
    #[doc = "0x500 - Enable UART"]
    pub enable: ENABLE,
    _reserved16: [u8; 0x04],
    #[doc = "0x508 - Pin select for RTS"]
    pub pselrts: PSELRTS,
    #[doc = "0x50c - Pin select for TXD"]
    pub pseltxd: PSELTXD,
    #[doc = "0x510 - Pin select for CTS"]
    pub pselcts: PSELCTS,
    #[doc = "0x514 - Pin select for RXD"]
    pub pselrxd: PSELRXD,
    #[doc = "0x518 - RXD register"]
    pub rxd: RXD,
    #[doc = "0x51c - TXD register"]
    pub txd: TXD,
    _reserved22: [u8; 0x04],
    #[doc = "0x524 - Baud rate"]
    pub baudrate: BAUDRATE,
    _reserved23: [u8; 0x44],
    #[doc = "0x56c - Configuration of parity and hardware flow control"]
    pub config: CONFIG,
}
#[doc = "TASKS_STARTRX (w) register accessor: an alias for `Reg<TASKS_STARTRX_SPEC>`"]
pub type TASKS_STARTRX = crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>;
#[doc = "Start UART receiver"]
pub mod tasks_startrx;
#[doc = "TASKS_STOPRX (w) register accessor: an alias for `Reg<TASKS_STOPRX_SPEC>`"]
pub type TASKS_STOPRX = crate::Reg<tasks_stoprx::TASKS_STOPRX_SPEC>;
#[doc = "Stop UART receiver"]
pub mod tasks_stoprx;
#[doc = "TASKS_STARTTX (w) register accessor: an alias for `Reg<TASKS_STARTTX_SPEC>`"]
pub type TASKS_STARTTX = crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>;
#[doc = "Start UART transmitter"]
pub mod tasks_starttx;
#[doc = "TASKS_STOPTX (w) register accessor: an alias for `Reg<TASKS_STOPTX_SPEC>`"]
pub type TASKS_STOPTX = crate::Reg<tasks_stoptx::TASKS_STOPTX_SPEC>;
#[doc = "Stop UART transmitter"]
pub mod tasks_stoptx;
#[doc = "TASKS_SUSPEND (w) register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend UART"]
pub mod tasks_suspend;
#[doc = "EVENTS_CTS (rw) register accessor: an alias for `Reg<EVENTS_CTS_SPEC>`"]
pub type EVENTS_CTS = crate::Reg<events_cts::EVENTS_CTS_SPEC>;
#[doc = "CTS is activated (set low). Clear To Send."]
pub mod events_cts;
#[doc = "EVENTS_NCTS (rw) register accessor: an alias for `Reg<EVENTS_NCTS_SPEC>`"]
pub type EVENTS_NCTS = crate::Reg<events_ncts::EVENTS_NCTS_SPEC>;
#[doc = "CTS is deactivated (set high). Not Clear To Send."]
pub mod events_ncts;
#[doc = "EVENTS_RXDRDY (rw) register accessor: an alias for `Reg<EVENTS_RXDRDY_SPEC>`"]
pub type EVENTS_RXDRDY = crate::Reg<events_rxdrdy::EVENTS_RXDRDY_SPEC>;
#[doc = "Data received in RXD"]
pub mod events_rxdrdy;
#[doc = "EVENTS_TXDRDY (rw) register accessor: an alias for `Reg<EVENTS_TXDRDY_SPEC>`"]
pub type EVENTS_TXDRDY = crate::Reg<events_txdrdy::EVENTS_TXDRDY_SPEC>;
#[doc = "Data sent from TXD"]
pub mod events_txdrdy;
#[doc = "EVENTS_ERROR (rw) register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "Error detected"]
pub mod events_error;
#[doc = "EVENTS_RXTO (rw) register accessor: an alias for `Reg<EVENTS_RXTO_SPEC>`"]
pub type EVENTS_RXTO = crate::Reg<events_rxto::EVENTS_RXTO_SPEC>;
#[doc = "Receiver timeout"]
pub mod events_rxto;
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
#[doc = "Enable UART"]
pub mod enable;
#[doc = "PSELRTS (rw) register accessor: an alias for `Reg<PSELRTS_SPEC>`"]
pub type PSELRTS = crate::Reg<pselrts::PSELRTS_SPEC>;
#[doc = "Pin select for RTS"]
pub mod pselrts;
#[doc = "PSELTXD (rw) register accessor: an alias for `Reg<PSELTXD_SPEC>`"]
pub type PSELTXD = crate::Reg<pseltxd::PSELTXD_SPEC>;
#[doc = "Pin select for TXD"]
pub mod pseltxd;
#[doc = "PSELCTS (rw) register accessor: an alias for `Reg<PSELCTS_SPEC>`"]
pub type PSELCTS = crate::Reg<pselcts::PSELCTS_SPEC>;
#[doc = "Pin select for CTS"]
pub mod pselcts;
#[doc = "PSELRXD (rw) register accessor: an alias for `Reg<PSELRXD_SPEC>`"]
pub type PSELRXD = crate::Reg<pselrxd::PSELRXD_SPEC>;
#[doc = "Pin select for RXD"]
pub mod pselrxd;
#[doc = "RXD (r) register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "RXD register"]
pub mod rxd;
#[doc = "TXD (w) register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "TXD register"]
pub mod txd;
#[doc = "BAUDRATE (rw) register accessor: an alias for `Reg<BAUDRATE_SPEC>`"]
pub type BAUDRATE = crate::Reg<baudrate::BAUDRATE_SPEC>;
#[doc = "Baud rate"]
pub mod baudrate;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration of parity and hardware flow control"]
pub mod config;
