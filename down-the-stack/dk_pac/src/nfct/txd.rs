#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Configuration of outgoing frames"]
    pub frameconfig: FRAMECONFIG,
    #[doc = "0x04 - Size of outgoing frame"]
    pub amount: AMOUNT,
}
#[doc = "FRAMECONFIG (rw) register accessor: an alias for `Reg<FRAMECONFIG_SPEC>`"]
pub type FRAMECONFIG = crate::Reg<frameconfig::FRAMECONFIG_SPEC>;
#[doc = "Configuration of outgoing frames"]
pub mod frameconfig;
#[doc = "AMOUNT (rw) register accessor: an alias for `Reg<AMOUNT_SPEC>`"]
pub type AMOUNT = crate::Reg<amount::AMOUNT_SPEC>;
#[doc = "Size of outgoing frame"]
pub mod amount;
