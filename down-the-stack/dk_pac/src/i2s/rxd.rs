#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Receive buffer RAM start address."]
    pub ptr: PTR,
}
#[doc = "PTR (rw) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Receive buffer RAM start address."]
pub mod ptr;
