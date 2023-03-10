#[doc = r"Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - TXD Data pointer"]
    pub ptr: PTR,
    #[doc = "0x04 - Maximum number of bytes in TXD buffer"]
    pub maxcnt: MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last TXD transaction"]
    pub amount: AMOUNT,
}
#[doc = "PTR (rw) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "TXD Data pointer"]
pub mod ptr;
#[doc = "MAXCNT (rw) register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "Maximum number of bytes in TXD buffer"]
pub mod maxcnt;
#[doc = "AMOUNT (r) register accessor: an alias for `Reg<AMOUNT_SPEC>`"]
pub type AMOUNT = crate::Reg<amount::AMOUNT_SPEC>;
#[doc = "Number of bytes transferred in the last TXD transaction"]
pub mod amount;
