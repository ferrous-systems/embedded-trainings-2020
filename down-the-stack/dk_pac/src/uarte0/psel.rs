#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for RTS signal"]
    pub rts: RTS,
    #[doc = "0x04 - Pin select for TXD signal"]
    pub txd: TXD,
    #[doc = "0x08 - Pin select for CTS signal"]
    pub cts: CTS,
    #[doc = "0x0c - Pin select for RXD signal"]
    pub rxd: RXD,
}
#[doc = "RTS (rw) register accessor: an alias for `Reg<RTS_SPEC>`"]
pub type RTS = crate::Reg<rts::RTS_SPEC>;
#[doc = "Pin select for RTS signal"]
pub mod rts;
#[doc = "TXD (rw) register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "Pin select for TXD signal"]
pub mod txd;
#[doc = "CTS (rw) register accessor: an alias for `Reg<CTS_SPEC>`"]
pub type CTS = crate::Reg<cts::CTS_SPEC>;
#[doc = "Pin select for CTS signal"]
pub mod cts;
#[doc = "RXD (rw) register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "Pin select for RXD signal"]
pub mod rxd;
