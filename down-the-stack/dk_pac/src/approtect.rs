#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0550],
    #[doc = "0x550 - Software force enable APPROTECT mechanism until next reset. This register can only be written once."]
    pub forceprotect: FORCEPROTECT,
    _reserved1: [u8; 0x04],
    #[doc = "0x558 - Software disable APPROTECT mechanism"]
    pub disable: DISABLE,
}
#[doc = "FORCEPROTECT (rw) register accessor: an alias for `Reg<FORCEPROTECT_SPEC>`"]
pub type FORCEPROTECT = crate::Reg<forceprotect::FORCEPROTECT_SPEC>;
#[doc = "Software force enable APPROTECT mechanism until next reset. This register can only be written once."]
pub mod forceprotect;
#[doc = "DISABLE (rw) register accessor: an alias for `Reg<DISABLE_SPEC>`"]
pub type DISABLE = crate::Reg<disable::DISABLE_SPEC>;
#[doc = "Software disable APPROTECT mechanism"]
pub mod disable;
