#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin number configuration for PDM CLK signal"]
    pub clk: CLK,
    #[doc = "0x04 - Pin number configuration for PDM DIN signal"]
    pub din: DIN,
}
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Pin number configuration for PDM CLK signal"]
pub mod clk;
#[doc = "DIN (rw) register accessor: an alias for `Reg<DIN_SPEC>`"]
pub type DIN = crate::Reg<din::DIN_SPEC>;
#[doc = "Pin number configuration for PDM DIN signal"]
pub mod din;
