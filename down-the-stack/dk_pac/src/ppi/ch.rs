#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - Description cluster: Channel n event end-point"]
    pub eep: EEP,
    #[doc = "0x04 - Description cluster: Channel n task end-point"]
    pub tep: TEP,
}
#[doc = "EEP (rw) register accessor: an alias for `Reg<EEP_SPEC>`"]
pub type EEP = crate::Reg<eep::EEP_SPEC>;
#[doc = "Description cluster: Channel n event end-point"]
pub mod eep;
#[doc = "TEP (rw) register accessor: an alias for `Reg<TEP_SPEC>`"]
pub type TEP = crate::Reg<tep::TEP_SPEC>;
#[doc = "Description cluster: Channel n task end-point"]
pub mod tep;
