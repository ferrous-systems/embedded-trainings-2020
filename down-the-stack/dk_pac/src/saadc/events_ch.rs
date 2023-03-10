#[doc = r"Register block"]
#[repr(C)]
pub struct EVENTS_CH {
    #[doc = "0x00 - Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
    pub limith: LIMITH,
    #[doc = "0x04 - Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    pub limitl: LIMITL,
}
#[doc = "LIMITH (rw) register accessor: an alias for `Reg<LIMITH_SPEC>`"]
pub type LIMITH = crate::Reg<limith::LIMITH_SPEC>;
#[doc = "Description cluster: Last results is equal or above CH\\[n\\].LIMIT.HIGH"]
pub mod limith;
#[doc = "LIMITL (rw) register accessor: an alias for `Reg<LIMITL_SPEC>`"]
pub type LIMITL = crate::Reg<limitl::LIMITL_SPEC>;
#[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW"]
pub mod limitl;
