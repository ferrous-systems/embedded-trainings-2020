#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x24],
    #[doc = "0x24 - Acquire SPI semaphore"]
    pub tasks_acquire: TASKS_ACQUIRE,
    #[doc = "0x28 - Release SPI semaphore, enabling the SPI slave to acquire it"]
    pub tasks_release: TASKS_RELEASE,
    _reserved2: [u8; 0xd8],
    #[doc = "0x104 - Granted transaction completed"]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 0x08],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 0x14],
    #[doc = "0x128 - Semaphore acquired"]
    pub events_acquired: EVENTS_ACQUIRED,
    _reserved5: [u8; 0xd4],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved6: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 0xf4],
    #[doc = "0x400 - Semaphore status register"]
    pub semstat: SEMSTAT,
    _reserved9: [u8; 0x3c],
    #[doc = "0x440 - Status from last transaction"]
    pub status: STATUS,
    _reserved10: [u8; 0xbc],
    #[doc = "0x500 - Enable SPI slave"]
    pub enable: ENABLE,
    _reserved11: [u8; 0x04],
    #[doc = "0x508..0x518 - Unspecified"]
    pub psel: PSEL,
    _reserved12: [u8; 0x1c],
    #[doc = "0x534..0x540 - Unspecified"]
    pub rxd: RXD,
    _reserved13: [u8; 0x04],
    #[doc = "0x544..0x550 - Unspecified"]
    pub txd: TXD,
    _reserved14: [u8; 0x04],
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved15: [u8; 0x04],
    #[doc = "0x55c - Default character. Character clocked out in case of an ignored transaction."]
    pub def: DEF,
    _reserved16: [u8; 0x60],
    #[doc = "0x5c0 - Over-read character"]
    pub orc: ORC,
}
#[doc = "TASKS_ACQUIRE (w) register accessor: an alias for `Reg<TASKS_ACQUIRE_SPEC>`"]
pub type TASKS_ACQUIRE = crate::Reg<tasks_acquire::TASKS_ACQUIRE_SPEC>;
#[doc = "Acquire SPI semaphore"]
pub mod tasks_acquire;
#[doc = "TASKS_RELEASE (w) register accessor: an alias for `Reg<TASKS_RELEASE_SPEC>`"]
pub type TASKS_RELEASE = crate::Reg<tasks_release::TASKS_RELEASE_SPEC>;
#[doc = "Release SPI semaphore, enabling the SPI slave to acquire it"]
pub mod tasks_release;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "Granted transaction completed"]
pub mod events_end;
#[doc = "EVENTS_ENDRX (rw) register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "EVENTS_ACQUIRED (rw) register accessor: an alias for `Reg<EVENTS_ACQUIRED_SPEC>`"]
pub type EVENTS_ACQUIRED = crate::Reg<events_acquired::EVENTS_ACQUIRED_SPEC>;
#[doc = "Semaphore acquired"]
pub mod events_acquired;
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
#[doc = "SEMSTAT (r) register accessor: an alias for `Reg<SEMSTAT_SPEC>`"]
pub type SEMSTAT = crate::Reg<semstat::SEMSTAT_SPEC>;
#[doc = "Semaphore status register"]
pub mod semstat;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status from last transaction"]
pub mod status;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPI slave"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
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
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "DEF (rw) register accessor: an alias for `Reg<DEF_SPEC>`"]
pub type DEF = crate::Reg<def::DEF_SPEC>;
#[doc = "Default character. Character clocked out in case of an ignored transaction."]
pub mod def;
#[doc = "ORC (rw) register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Over-read character"]
pub mod orc;
