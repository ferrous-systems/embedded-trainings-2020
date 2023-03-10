#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    #[doc = "0x100..0x120 - Peripheral events."]
    pub events_region: [EVENTS_REGION; 4],
    _reserved1: [u8; 0x40],
    #[doc = "0x160..0x170 - Peripheral events."]
    pub events_pregion: [EVENTS_PREGION; 2],
    _reserved2: [u8; 0x0190],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x14],
    #[doc = "0x320 - Enable or disable interrupt"]
    pub nmien: NMIEN,
    #[doc = "0x324 - Enable interrupt"]
    pub nmienset: NMIENSET,
    #[doc = "0x328 - Disable interrupt"]
    pub nmienclr: NMIENCLR,
    _reserved8: [u8; 0xd4],
    #[doc = "0x400..0x410 - Unspecified"]
    pub perregion: [PERREGION; 2],
    _reserved9: [u8; 0x0100],
    #[doc = "0x510 - Enable/disable regions watch"]
    pub regionen: REGIONEN,
    #[doc = "0x514 - Enable regions watch"]
    pub regionenset: REGIONENSET,
    #[doc = "0x518 - Disable regions watch"]
    pub regionenclr: REGIONENCLR,
    _reserved12: [u8; 0xe4],
    #[doc = "0x600..0x608 - Unspecified"]
    pub region0: REGION,
    _reserved13: [u8; 0x08],
    #[doc = "0x610..0x618 - Unspecified"]
    pub region1: REGION,
    _reserved14: [u8; 0x08],
    #[doc = "0x620..0x628 - Unspecified"]
    pub region2: REGION,
    _reserved15: [u8; 0x08],
    #[doc = "0x630..0x638 - Unspecified"]
    pub region3: REGION,
    _reserved16: [u8; 0x88],
    #[doc = "0x6c0..0x6cc - Unspecified"]
    pub pregion0: PREGION,
    _reserved17: [u8; 0x04],
    #[doc = "0x6d0..0x6dc - Unspecified"]
    pub pregion1: PREGION,
}
#[doc = "Peripheral events."]
pub use self::events_region::EVENTS_REGION;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_region;
#[doc = "Peripheral events."]
pub use self::events_pregion::EVENTS_PREGION;
#[doc = r"Cluster"]
#[doc = "Peripheral events."]
pub mod events_pregion;
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
#[doc = "NMIEN (rw) register accessor: an alias for `Reg<NMIEN_SPEC>`"]
pub type NMIEN = crate::Reg<nmien::NMIEN_SPEC>;
#[doc = "Enable or disable interrupt"]
pub mod nmien;
#[doc = "NMIENSET (rw) register accessor: an alias for `Reg<NMIENSET_SPEC>`"]
pub type NMIENSET = crate::Reg<nmienset::NMIENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod nmienset;
#[doc = "NMIENCLR (rw) register accessor: an alias for `Reg<NMIENCLR_SPEC>`"]
pub type NMIENCLR = crate::Reg<nmienclr::NMIENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod nmienclr;
#[doc = "Unspecified"]
pub use self::perregion::PERREGION;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod perregion;
#[doc = "REGIONEN (rw) register accessor: an alias for `Reg<REGIONEN_SPEC>`"]
pub type REGIONEN = crate::Reg<regionen::REGIONEN_SPEC>;
#[doc = "Enable/disable regions watch"]
pub mod regionen;
#[doc = "REGIONENSET (rw) register accessor: an alias for `Reg<REGIONENSET_SPEC>`"]
pub type REGIONENSET = crate::Reg<regionenset::REGIONENSET_SPEC>;
#[doc = "Enable regions watch"]
pub mod regionenset;
#[doc = "REGIONENCLR (rw) register accessor: an alias for `Reg<REGIONENCLR_SPEC>`"]
pub type REGIONENCLR = crate::Reg<regionenclr::REGIONENCLR_SPEC>;
#[doc = "Disable regions watch"]
pub mod regionenclr;
#[doc = "Unspecified"]
pub use self::region::REGION;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod region;
#[doc = "Unspecified"]
pub use self::pregion::PREGION;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod pregion;
