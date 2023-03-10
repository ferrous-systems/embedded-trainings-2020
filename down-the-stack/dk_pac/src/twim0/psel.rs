#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCL signal"]
    pub scl: SCL,
    #[doc = "0x04 - Pin select for SDA signal"]
    pub sda: SDA,
}
#[doc = "SCL (rw) register accessor: an alias for `Reg<SCL_SPEC>`"]
pub type SCL = crate::Reg<scl::SCL_SPEC>;
#[doc = "Pin select for SCL signal"]
pub mod scl;
#[doc = "SDA (rw) register accessor: an alias for `Reg<SDA_SPEC>`"]
pub type SDA = crate::Reg<sda::SDA_SPEC>;
#[doc = "Pin select for SDA signal"]
pub mod sda;
