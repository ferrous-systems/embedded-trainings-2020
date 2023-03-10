#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0600],
    #[doc = "0x600 - Block protect configuration register 0"]
    pub config0: CONFIG0,
    #[doc = "0x604 - Block protect configuration register 1"]
    pub config1: CONFIG1,
    #[doc = "0x608 - Disable protection mechanism in debug interface mode"]
    pub disableindebug: DISABLEINDEBUG,
    _reserved3: [u8; 0x04],
    #[doc = "0x610 - Block protect configuration register 2"]
    pub config2: CONFIG2,
    #[doc = "0x614 - Block protect configuration register 3"]
    pub config3: CONFIG3,
}
#[doc = "CONFIG0 (rw) register accessor: an alias for `Reg<CONFIG0_SPEC>`"]
pub type CONFIG0 = crate::Reg<config0::CONFIG0_SPEC>;
#[doc = "Block protect configuration register 0"]
pub mod config0;
#[doc = "CONFIG1 (rw) register accessor: an alias for `Reg<CONFIG1_SPEC>`"]
pub type CONFIG1 = crate::Reg<config1::CONFIG1_SPEC>;
#[doc = "Block protect configuration register 1"]
pub mod config1;
#[doc = "DISABLEINDEBUG (rw) register accessor: an alias for `Reg<DISABLEINDEBUG_SPEC>`"]
pub type DISABLEINDEBUG = crate::Reg<disableindebug::DISABLEINDEBUG_SPEC>;
#[doc = "Disable protection mechanism in debug interface mode"]
pub mod disableindebug;
#[doc = "CONFIG2 (rw) register accessor: an alias for `Reg<CONFIG2_SPEC>`"]
pub type CONFIG2 = crate::Reg<config2::CONFIG2_SPEC>;
#[doc = "Block protect configuration register 2"]
pub mod config2;
#[doc = "CONFIG3 (rw) register accessor: an alias for `Reg<CONFIG3_SPEC>`"]
pub type CONFIG3 = crate::Reg<config3::CONFIG3_SPEC>;
#[doc = "Block protect configuration register 3"]
pub mod config3;
