#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for LED signal"]
    pub led: LED,
    #[doc = "0x04 - Pin select for A signal"]
    pub a: A,
    #[doc = "0x08 - Pin select for B signal"]
    pub b: B,
}
#[doc = "LED (rw) register accessor: an alias for `Reg<LED_SPEC>`"]
pub type LED = crate::Reg<led::LED_SPEC>;
#[doc = "Pin select for LED signal"]
pub mod led;
#[doc = "A (rw) register accessor: an alias for `Reg<A_SPEC>`"]
pub type A = crate::Reg<a::A_SPEC>;
#[doc = "Pin select for A signal"]
pub mod a;
#[doc = "B (rw) register accessor: an alias for `Reg<B_SPEC>`"]
pub type B = crate::Reg<b::B_SPEC>;
#[doc = "Pin select for B signal"]
pub mod b;
