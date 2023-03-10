#[doc = r"Register block"]
#[repr(C)]
pub struct FRAMESTATUS {
    #[doc = "0x00 - Result of last incoming frames"]
    pub rx: RX,
}
#[doc = "RX (rw) register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "Result of last incoming frames"]
pub mod rx;
