#[doc = r"Register block"]
#[repr(C)]
pub struct RXTXD {
    #[doc = "0x00 - Size of RXD and TXD buffers."]
    pub maxcnt: MAXCNT,
}
#[doc = "MAXCNT (rw) register accessor: an alias for `Reg<MAXCNT_SPEC>`"]
pub type MAXCNT = crate::Reg<maxcnt::MAXCNT_SPEC>;
#[doc = "Size of RXD and TXD buffers."]
pub mod maxcnt;
