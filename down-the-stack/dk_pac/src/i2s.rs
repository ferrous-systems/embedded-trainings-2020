#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0xfc],
    #[doc = "0x104 - The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
    pub events_rxptrupd: EVENTS_RXPTRUPD,
    #[doc = "0x108 - I2S transfer stopped."]
    pub events_stopped: EVENTS_STOPPED,
    _reserved4: [u8; 0x08],
    #[doc = "0x114 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    pub events_txptrupd: EVENTS_TXPTRUPD,
    _reserved5: [u8; 0x01e8],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 0x01f4],
    #[doc = "0x500 - Enable I2S module."]
    pub enable: ENABLE,
    #[doc = "0x504..0x52c - Unspecified"]
    pub config: CONFIG,
    _reserved10: [u8; 0x0c],
    #[doc = "0x538 - Unspecified"]
    pub rxd: RXD,
    _reserved11: [u8; 0x04],
    #[doc = "0x540 - Unspecified"]
    pub txd: TXD,
    _reserved12: [u8; 0x0c],
    #[doc = "0x550 - Unspecified"]
    pub rxtxd: RXTXD,
    _reserved13: [u8; 0x0c],
    #[doc = "0x560..0x574 - Unspecified"]
    pub psel: PSEL,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Starts continuous I2S transfer. Also starts MCK generator when this is enabled."]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stops I2S transfer. Also stops MCK generator. Triggering this task will cause the {event:STOPPED} event to be generated."]
pub mod tasks_stop;
#[doc = "EVENTS_RXPTRUPD (rw) register accessor: an alias for `Reg<EVENTS_RXPTRUPD_SPEC>`"]
pub type EVENTS_RXPTRUPD = crate::Reg<events_rxptrupd::EVENTS_RXPTRUPD_SPEC>;
#[doc = "The RXD.PTR register has been copied to internal double-buffers. When the I2S module is started and RX is enabled, this event will be generated for every RXTXD.MAXCNT words that are received on the SDIN pin."]
pub mod events_rxptrupd;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "I2S transfer stopped."]
pub mod events_stopped;
#[doc = "EVENTS_TXPTRUPD (rw) register accessor: an alias for `Reg<EVENTS_TXPTRUPD_SPEC>`"]
pub type EVENTS_TXPTRUPD = crate::Reg<events_txptrupd::EVENTS_TXPTRUPD_SPEC>;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub mod events_txptrupd;
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
#[doc = "Enable I2S module."]
pub mod enable;
#[doc = "Unspecified"]
pub use self::config::CONFIG;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod config;
#[doc = "Unspecified"]
pub use self::rxd::RXD;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rxd;
#[doc = "Unspecified"]
pub use self::txd::TXD;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod txd;
#[doc = "Unspecified"]
pub use self::rxtxd::RXTXD;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod rxtxd;
#[doc = "Unspecified"]
pub use self::psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
