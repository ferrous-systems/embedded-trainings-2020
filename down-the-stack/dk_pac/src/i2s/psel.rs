#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for MCK signal."]
    pub mck: MCK,
    #[doc = "0x04 - Pin select for SCK signal."]
    pub sck: SCK,
    #[doc = "0x08 - Pin select for LRCK signal."]
    pub lrck: LRCK,
    #[doc = "0x0c - Pin select for SDIN signal."]
    pub sdin: SDIN,
    #[doc = "0x10 - Pin select for SDOUT signal."]
    pub sdout: SDOUT,
}
#[doc = "MCK (rw) register accessor: an alias for `Reg<MCK_SPEC>`"]
pub type MCK = crate::Reg<mck::MCK_SPEC>;
#[doc = "Pin select for MCK signal."]
pub mod mck;
#[doc = "SCK (rw) register accessor: an alias for `Reg<SCK_SPEC>`"]
pub type SCK = crate::Reg<sck::SCK_SPEC>;
#[doc = "Pin select for SCK signal."]
pub mod sck;
#[doc = "LRCK (rw) register accessor: an alias for `Reg<LRCK_SPEC>`"]
pub type LRCK = crate::Reg<lrck::LRCK_SPEC>;
#[doc = "Pin select for LRCK signal."]
pub mod lrck;
#[doc = "SDIN (rw) register accessor: an alias for `Reg<SDIN_SPEC>`"]
pub type SDIN = crate::Reg<sdin::SDIN_SPEC>;
#[doc = "Pin select for SDIN signal."]
pub mod sdin;
#[doc = "SDOUT (rw) register accessor: an alias for `Reg<SDOUT_SPEC>`"]
pub type SDOUT = crate::Reg<sdout::SDOUT_SPEC>;
#[doc = "Pin select for SDOUT signal."]
pub mod sdout;
