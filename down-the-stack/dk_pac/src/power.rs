#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x78],
    #[doc = "0x78 - Enable constant latency mode"]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 0x88],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 0x08],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    _reserved5: [u8; 0x01e8],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 0xf4],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved8: [u8; 0x24],
    #[doc = "0x428 - Deprecated register - RAM status register"]
    pub ramstatus: RAMSTATUS,
    _reserved9: [u8; 0xd4],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved10: [u8; 0x0c],
    #[doc = "0x510 - Power failure comparator configuration"]
    pub pofcon: POFCON,
    _reserved11: [u8; 0x08],
    #[doc = "0x51c - General purpose retention register"]
    pub gpregret: GPREGRET,
    #[doc = "0x520 - General purpose retention register"]
    pub gpregret2: GPREGRET2,
    #[doc = "0x524 - Deprecated register - RAM on/off register (this register is retained)"]
    pub ramon: RAMON,
    _reserved14: [u8; 0x2c],
    #[doc = "0x554 - Deprecated register - RAM on/off register (this register is retained)"]
    pub ramonb: RAMONB,
    _reserved15: [u8; 0x20],
    #[doc = "0x578 - DC/DC enable register"]
    pub dcdcen: DCDCEN,
    _reserved16: [u8; 0x0384],
    #[doc = "0x900..0x90c - Unspecified"]
    pub ram0: RAM,
    _reserved17: [u8; 0x04],
    #[doc = "0x910..0x91c - Unspecified"]
    pub ram1: RAM,
    _reserved18: [u8; 0x04],
    #[doc = "0x920..0x92c - Unspecified"]
    pub ram2: RAM,
    _reserved19: [u8; 0x04],
    #[doc = "0x930..0x93c - Unspecified"]
    pub ram3: RAM,
    _reserved20: [u8; 0x04],
    #[doc = "0x940..0x94c - Unspecified"]
    pub ram4: RAM,
    _reserved21: [u8; 0x04],
    #[doc = "0x950..0x95c - Unspecified"]
    pub ram5: RAM,
    _reserved22: [u8; 0x04],
    #[doc = "0x960..0x96c - Unspecified"]
    pub ram6: RAM,
    _reserved23: [u8; 0x04],
    #[doc = "0x970..0x97c - Unspecified"]
    pub ram7: RAM,
}
#[doc = "TASKS_CONSTLAT (w) register accessor: an alias for `Reg<TASKS_CONSTLAT_SPEC>`"]
pub type TASKS_CONSTLAT = crate::Reg<tasks_constlat::TASKS_CONSTLAT_SPEC>;
#[doc = "Enable constant latency mode"]
pub mod tasks_constlat;
#[doc = "TASKS_LOWPWR (w) register accessor: an alias for `Reg<TASKS_LOWPWR_SPEC>`"]
pub type TASKS_LOWPWR = crate::Reg<tasks_lowpwr::TASKS_LOWPWR_SPEC>;
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "EVENTS_POFWARN (rw) register accessor: an alias for `Reg<EVENTS_POFWARN_SPEC>`"]
pub type EVENTS_POFWARN = crate::Reg<events_pofwarn::EVENTS_POFWARN_SPEC>;
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "EVENTS_SLEEPENTER (rw) register accessor: an alias for `Reg<EVENTS_SLEEPENTER_SPEC>`"]
pub type EVENTS_SLEEPENTER = crate::Reg<events_sleepenter::EVENTS_SLEEPENTER_SPEC>;
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "EVENTS_SLEEPEXIT (rw) register accessor: an alias for `Reg<EVENTS_SLEEPEXIT_SPEC>`"]
pub type EVENTS_SLEEPEXIT = crate::Reg<events_sleepexit::EVENTS_SLEEPEXIT_SPEC>;
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "RESETREAS (rw) register accessor: an alias for `Reg<RESETREAS_SPEC>`"]
pub type RESETREAS = crate::Reg<resetreas::RESETREAS_SPEC>;
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "RAMSTATUS (r) register accessor: an alias for `Reg<RAMSTATUS_SPEC>`"]
pub type RAMSTATUS = crate::Reg<ramstatus::RAMSTATUS_SPEC>;
#[doc = "Deprecated register - RAM status register"]
pub mod ramstatus;
#[doc = "SYSTEMOFF (w) register accessor: an alias for `Reg<SYSTEMOFF_SPEC>`"]
pub type SYSTEMOFF = crate::Reg<systemoff::SYSTEMOFF_SPEC>;
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "POFCON (rw) register accessor: an alias for `Reg<POFCON_SPEC>`"]
pub type POFCON = crate::Reg<pofcon::POFCON_SPEC>;
#[doc = "Power failure comparator configuration"]
pub mod pofcon;
#[doc = "GPREGRET (rw) register accessor: an alias for `Reg<GPREGRET_SPEC>`"]
pub type GPREGRET = crate::Reg<gpregret::GPREGRET_SPEC>;
#[doc = "General purpose retention register"]
pub mod gpregret;
#[doc = "GPREGRET2 (rw) register accessor: an alias for `Reg<GPREGRET2_SPEC>`"]
pub type GPREGRET2 = crate::Reg<gpregret2::GPREGRET2_SPEC>;
#[doc = "General purpose retention register"]
pub mod gpregret2;
#[doc = "RAMON (rw) register accessor: an alias for `Reg<RAMON_SPEC>`"]
pub type RAMON = crate::Reg<ramon::RAMON_SPEC>;
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramon;
#[doc = "RAMONB (rw) register accessor: an alias for `Reg<RAMONB_SPEC>`"]
pub type RAMONB = crate::Reg<ramonb::RAMONB_SPEC>;
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramonb;
#[doc = "DCDCEN (rw) register accessor: an alias for `Reg<DCDCEN_SPEC>`"]
pub type DCDCEN = crate::Reg<dcdcen::DCDCEN_SPEC>;
#[doc = "DC/DC enable register"]
pub mod dcdcen;
#[doc = "Unspecified"]
pub use self::ram::RAM;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod ram;
