#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00..0x10 - Description collection: Output pin select for PWM channel n"]
    pub out: [OUT; 4],
}
#[doc = "OUT (rw) register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Description collection: Output pin select for PWM channel n"]
pub mod out;
