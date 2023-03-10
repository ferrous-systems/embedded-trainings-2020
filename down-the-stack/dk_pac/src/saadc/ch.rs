#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster: Input positive pin selection for CH\\[n\\]"]
    pub pselp: PSELP,
    #[doc = "0x04 - Description cluster: Input negative pin selection for CH\\[n\\]"]
    pub pseln: PSELN,
    #[doc = "0x08 - Description cluster: Input configuration for CH\\[n\\]"]
    pub config: CONFIG,
    #[doc = "0x0c - Description cluster: High/low limits for event monitoring a channel"]
    pub limit: LIMIT,
}
#[doc = "PSELP (rw) register accessor: an alias for `Reg<PSELP_SPEC>`"]
pub type PSELP = crate::Reg<pselp::PSELP_SPEC>;
#[doc = "Description cluster: Input positive pin selection for CH\\[n\\]"]
pub mod pselp;
#[doc = "PSELN (rw) register accessor: an alias for `Reg<PSELN_SPEC>`"]
pub type PSELN = crate::Reg<pseln::PSELN_SPEC>;
#[doc = "Description cluster: Input negative pin selection for CH\\[n\\]"]
pub mod pseln;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Description cluster: Input configuration for CH\\[n\\]"]
pub mod config;
#[doc = "LIMIT (rw) register accessor: an alias for `Reg<LIMIT_SPEC>`"]
pub type LIMIT = crate::Reg<limit::LIMIT_SPEC>;
#[doc = "Description cluster: High/low limits for event monitoring a channel"]
pub mod limit;
