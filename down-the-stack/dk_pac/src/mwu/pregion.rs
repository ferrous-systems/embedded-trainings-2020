#[doc = r"Register block"]
#[repr(C)]
pub struct PREGION {
    #[doc = "0x00 - Description cluster: Reserved for future use"]
    pub start: START,
    #[doc = "0x04 - Description cluster: Reserved for future use"]
    pub end: END,
    #[doc = "0x08 - Description cluster: Subregions of region n"]
    pub subs: SUBS,
}
#[doc = "START (r) register accessor: an alias for `Reg<START_SPEC>`"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Description cluster: Reserved for future use"]
pub mod start;
#[doc = "END (r) register accessor: an alias for `Reg<END_SPEC>`"]
pub type END = crate::Reg<end::END_SPEC>;
#[doc = "Description cluster: Reserved for future use"]
pub mod end;
#[doc = "SUBS (rw) register accessor: an alias for `Reg<SUBS_SPEC>`"]
pub type SUBS = crate::Reg<subs::SUBS_SPEC>;
#[doc = "Description cluster: Subregions of region n"]
pub mod subs;
