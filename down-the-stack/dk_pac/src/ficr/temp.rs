#[doc = r"Register block"]
#[repr(C)]
pub struct TEMP {
    #[doc = "0x00 - Slope definition A0."]
    pub a0: A0,
    #[doc = "0x04 - Slope definition A1."]
    pub a1: A1,
    #[doc = "0x08 - Slope definition A2."]
    pub a2: A2,
    #[doc = "0x0c - Slope definition A3."]
    pub a3: A3,
    #[doc = "0x10 - Slope definition A4."]
    pub a4: A4,
    #[doc = "0x14 - Slope definition A5."]
    pub a5: A5,
    #[doc = "0x18 - y-intercept B0."]
    pub b0: B0,
    #[doc = "0x1c - y-intercept B1."]
    pub b1: B1,
    #[doc = "0x20 - y-intercept B2."]
    pub b2: B2,
    #[doc = "0x24 - y-intercept B3."]
    pub b3: B3,
    #[doc = "0x28 - y-intercept B4."]
    pub b4: B4,
    #[doc = "0x2c - y-intercept B5."]
    pub b5: B5,
    #[doc = "0x30 - Segment end T0."]
    pub t0: T0,
    #[doc = "0x34 - Segment end T1."]
    pub t1: T1,
    #[doc = "0x38 - Segment end T2."]
    pub t2: T2,
    #[doc = "0x3c - Segment end T3."]
    pub t3: T3,
    #[doc = "0x40 - Segment end T4."]
    pub t4: T4,
}
#[doc = "A0 (r) register accessor: an alias for `Reg<A0_SPEC>`"]
pub type A0 = crate::Reg<a0::A0_SPEC>;
#[doc = "Slope definition A0."]
pub mod a0;
#[doc = "A1 (r) register accessor: an alias for `Reg<A1_SPEC>`"]
pub type A1 = crate::Reg<a1::A1_SPEC>;
#[doc = "Slope definition A1."]
pub mod a1;
#[doc = "A2 (r) register accessor: an alias for `Reg<A2_SPEC>`"]
pub type A2 = crate::Reg<a2::A2_SPEC>;
#[doc = "Slope definition A2."]
pub mod a2;
#[doc = "A3 (r) register accessor: an alias for `Reg<A3_SPEC>`"]
pub type A3 = crate::Reg<a3::A3_SPEC>;
#[doc = "Slope definition A3."]
pub mod a3;
#[doc = "A4 (r) register accessor: an alias for `Reg<A4_SPEC>`"]
pub type A4 = crate::Reg<a4::A4_SPEC>;
#[doc = "Slope definition A4."]
pub mod a4;
#[doc = "A5 (r) register accessor: an alias for `Reg<A5_SPEC>`"]
pub type A5 = crate::Reg<a5::A5_SPEC>;
#[doc = "Slope definition A5."]
pub mod a5;
#[doc = "B0 (r) register accessor: an alias for `Reg<B0_SPEC>`"]
pub type B0 = crate::Reg<b0::B0_SPEC>;
#[doc = "y-intercept B0."]
pub mod b0;
#[doc = "B1 (r) register accessor: an alias for `Reg<B1_SPEC>`"]
pub type B1 = crate::Reg<b1::B1_SPEC>;
#[doc = "y-intercept B1."]
pub mod b1;
#[doc = "B2 (r) register accessor: an alias for `Reg<B2_SPEC>`"]
pub type B2 = crate::Reg<b2::B2_SPEC>;
#[doc = "y-intercept B2."]
pub mod b2;
#[doc = "B3 (r) register accessor: an alias for `Reg<B3_SPEC>`"]
pub type B3 = crate::Reg<b3::B3_SPEC>;
#[doc = "y-intercept B3."]
pub mod b3;
#[doc = "B4 (r) register accessor: an alias for `Reg<B4_SPEC>`"]
pub type B4 = crate::Reg<b4::B4_SPEC>;
#[doc = "y-intercept B4."]
pub mod b4;
#[doc = "B5 (r) register accessor: an alias for `Reg<B5_SPEC>`"]
pub type B5 = crate::Reg<b5::B5_SPEC>;
#[doc = "y-intercept B5."]
pub mod b5;
#[doc = "T0 (r) register accessor: an alias for `Reg<T0_SPEC>`"]
pub type T0 = crate::Reg<t0::T0_SPEC>;
#[doc = "Segment end T0."]
pub mod t0;
#[doc = "T1 (r) register accessor: an alias for `Reg<T1_SPEC>`"]
pub type T1 = crate::Reg<t1::T1_SPEC>;
#[doc = "Segment end T1."]
pub mod t1;
#[doc = "T2 (r) register accessor: an alias for `Reg<T2_SPEC>`"]
pub type T2 = crate::Reg<t2::T2_SPEC>;
#[doc = "Segment end T2."]
pub mod t2;
#[doc = "T3 (r) register accessor: an alias for `Reg<T3_SPEC>`"]
pub type T3 = crate::Reg<t3::T3_SPEC>;
#[doc = "Segment end T3."]
pub mod t3;
#[doc = "T4 (r) register accessor: an alias for `Reg<T4_SPEC>`"]
pub type T4 = crate::Reg<t4::T4_SPEC>;
#[doc = "Segment end T4."]
pub mod t4;
