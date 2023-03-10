#[doc = r"Register block"]
#[repr(C)]
pub struct FORK {
    #[doc = "0x00 - Description cluster: Channel n task end-point"]
    pub tep: TEP,
}
#[doc = "TEP (rw) register accessor: an alias for `Reg<TEP_SPEC>`"]
pub type TEP = crate::Reg<tep::TEP_SPEC>;
#[doc = "Description cluster: Channel n task end-point"]
pub mod tep;
