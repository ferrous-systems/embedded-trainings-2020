#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x30 - Channel group tasks"]
    pub tasks_chg: [TASKS_CHG; 6],
    _reserved1: [u8; 0x04d0],
    #[doc = "0x500 - Channel enable register"]
    pub chen: CHEN,
    #[doc = "0x504 - Channel enable set register"]
    pub chenset: CHENSET,
    #[doc = "0x508 - Channel enable clear register"]
    pub chenclr: CHENCLR,
    _reserved4: [u8; 0x04],
    #[doc = "0x510..0x5b0 - PPI Channel"]
    pub ch: [CH; 20],
    _reserved5: [u8; 0x0250],
    #[doc = "0x800..0x818 - Description collection: Channel group n"]
    pub chg: [CHG; 6],
    _reserved6: [u8; 0xf8],
    #[doc = "0x910..0x990 - Fork"]
    pub fork: [FORK; 32],
}
#[doc = "Channel group tasks"]
pub use self::tasks_chg::TASKS_CHG;
#[doc = r"Cluster"]
#[doc = "Channel group tasks"]
pub mod tasks_chg;
#[doc = "CHEN (rw) register accessor: an alias for `Reg<CHEN_SPEC>`"]
pub type CHEN = crate::Reg<chen::CHEN_SPEC>;
#[doc = "Channel enable register"]
pub mod chen;
#[doc = "CHENSET (rw) register accessor: an alias for `Reg<CHENSET_SPEC>`"]
pub type CHENSET = crate::Reg<chenset::CHENSET_SPEC>;
#[doc = "Channel enable set register"]
pub mod chenset;
#[doc = "CHENCLR (rw) register accessor: an alias for `Reg<CHENCLR_SPEC>`"]
pub type CHENCLR = crate::Reg<chenclr::CHENCLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod chenclr;
#[doc = "PPI Channel"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "PPI Channel"]
pub mod ch;
#[doc = "CHG (rw) register accessor: an alias for `Reg<CHG_SPEC>`"]
pub type CHG = crate::Reg<chg::CHG_SPEC>;
#[doc = "Description collection: Channel group n"]
pub mod chg;
#[doc = "Fork"]
pub use self::fork::FORK;
#[doc = r"Cluster"]
#[doc = "Fork"]
pub mod fork;
