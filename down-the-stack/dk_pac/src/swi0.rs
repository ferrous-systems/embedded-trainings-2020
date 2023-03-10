#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unused."]
    pub unused: UNUSED,
}
#[doc = "UNUSED (r) register accessor: an alias for `Reg<UNUSED_SPEC>`"]
pub type UNUSED = crate::Reg<unused::UNUSED_SPEC>;
#[doc = "Unused."]
pub mod unused;
