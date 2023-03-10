#[doc = r"Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Configuration of incoming frames"]
    pub frameconfig: FRAMECONFIG,
    #[doc = "0x04 - Size of last incoming frame"]
    pub amount: AMOUNT,
}
#[doc = "FRAMECONFIG (rw) register accessor: an alias for `Reg<FRAMECONFIG_SPEC>`"]
pub type FRAMECONFIG = crate::Reg<frameconfig::FRAMECONFIG_SPEC>;
#[doc = "Configuration of incoming frames"]
pub mod frameconfig;
#[doc = "AMOUNT (r) register accessor: an alias for `Reg<AMOUNT_SPEC>`"]
pub type AMOUNT = crate::Reg<amount::AMOUNT_SPEC>;
#[doc = "Size of last incoming frame"]
pub mod amount;
