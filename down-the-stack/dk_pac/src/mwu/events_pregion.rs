#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_PREGION {
    #[doc = "0x00 - Description cluster: Write access to peripheral region n detected"]
    pub wa: WA,
    #[doc = "0x04 - Description cluster: Read access to peripheral region n detected"]
    pub ra: RA,
}
#[doc = "WA (rw) register accessor: an alias for `Reg<WA_SPEC>`"]
pub type WA = crate::Reg<wa::WA_SPEC>;
#[doc = "Description cluster: Write access to peripheral region n detected"]
pub mod wa;
#[doc = "RA (rw) register accessor: an alias for `Reg<RA_SPEC>`"]
pub type RA = crate::Reg<ra::RA_SPEC>;
#[doc = "Description cluster: Read access to peripheral region n detected"]
pub mod ra;
