#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start temperature measurement"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop temperature measurement"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Temperature measurement complete, data ready"]
    pub events_datardy: EVENTS_DATARDY,
    _reserved3: [u8; 0x0200],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x01fc],
    #[doc = "0x508 - Temperature in degC (0.25deg steps)"]
    pub temp: TEMP,
    _reserved6: [u8; 0x14],
    #[doc = "0x520 - Slope of 1st piece wise linear function"]
    pub a0: A0,
    #[doc = "0x524 - Slope of 2nd piece wise linear function"]
    pub a1: A1,
    #[doc = "0x528 - Slope of 3rd piece wise linear function"]
    pub a2: A2,
    #[doc = "0x52c - Slope of 4th piece wise linear function"]
    pub a3: A3,
    #[doc = "0x530 - Slope of 5th piece wise linear function"]
    pub a4: A4,
    #[doc = "0x534 - Slope of 6th piece wise linear function"]
    pub a5: A5,
    _reserved12: [u8; 0x08],
    #[doc = "0x540 - y-intercept of 1st piece wise linear function"]
    pub b0: B0,
    #[doc = "0x544 - y-intercept of 2nd piece wise linear function"]
    pub b1: B1,
    #[doc = "0x548 - y-intercept of 3rd piece wise linear function"]
    pub b2: B2,
    #[doc = "0x54c - y-intercept of 4th piece wise linear function"]
    pub b3: B3,
    #[doc = "0x550 - y-intercept of 5th piece wise linear function"]
    pub b4: B4,
    #[doc = "0x554 - y-intercept of 6th piece wise linear function"]
    pub b5: B5,
    _reserved18: [u8; 0x08],
    #[doc = "0x560 - End point of 1st piece wise linear function"]
    pub t0: T0,
    #[doc = "0x564 - End point of 2nd piece wise linear function"]
    pub t1: T1,
    #[doc = "0x568 - End point of 3rd piece wise linear function"]
    pub t2: T2,
    #[doc = "0x56c - End point of 4th piece wise linear function"]
    pub t3: T3,
    #[doc = "0x570 - End point of 5th piece wise linear function"]
    pub t4: T4,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start temperature measurement"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop temperature measurement"]
pub mod tasks_stop;
#[doc = "EVENTS_DATARDY (rw) register accessor: an alias for `Reg<EVENTS_DATARDY_SPEC>`"]
pub type EVENTS_DATARDY = crate::Reg<events_datardy::EVENTS_DATARDY_SPEC>;
#[doc = "Temperature measurement complete, data ready"]
pub mod events_datardy;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "TEMP (r) register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temperature in degC (0.25deg steps)"]
pub mod temp;
#[doc = "A0 (rw) register accessor: an alias for `Reg<A0_SPEC>`"]
pub type A0 = crate::Reg<a0::A0_SPEC>;
#[doc = "Slope of 1st piece wise linear function"]
pub mod a0;
#[doc = "A1 (rw) register accessor: an alias for `Reg<A1_SPEC>`"]
pub type A1 = crate::Reg<a1::A1_SPEC>;
#[doc = "Slope of 2nd piece wise linear function"]
pub mod a1;
#[doc = "A2 (rw) register accessor: an alias for `Reg<A2_SPEC>`"]
pub type A2 = crate::Reg<a2::A2_SPEC>;
#[doc = "Slope of 3rd piece wise linear function"]
pub mod a2;
#[doc = "A3 (rw) register accessor: an alias for `Reg<A3_SPEC>`"]
pub type A3 = crate::Reg<a3::A3_SPEC>;
#[doc = "Slope of 4th piece wise linear function"]
pub mod a3;
#[doc = "A4 (rw) register accessor: an alias for `Reg<A4_SPEC>`"]
pub type A4 = crate::Reg<a4::A4_SPEC>;
#[doc = "Slope of 5th piece wise linear function"]
pub mod a4;
#[doc = "A5 (rw) register accessor: an alias for `Reg<A5_SPEC>`"]
pub type A5 = crate::Reg<a5::A5_SPEC>;
#[doc = "Slope of 6th piece wise linear function"]
pub mod a5;
#[doc = "B0 (rw) register accessor: an alias for `Reg<B0_SPEC>`"]
pub type B0 = crate::Reg<b0::B0_SPEC>;
#[doc = "y-intercept of 1st piece wise linear function"]
pub mod b0;
#[doc = "B1 (rw) register accessor: an alias for `Reg<B1_SPEC>`"]
pub type B1 = crate::Reg<b1::B1_SPEC>;
#[doc = "y-intercept of 2nd piece wise linear function"]
pub mod b1;
#[doc = "B2 (rw) register accessor: an alias for `Reg<B2_SPEC>`"]
pub type B2 = crate::Reg<b2::B2_SPEC>;
#[doc = "y-intercept of 3rd piece wise linear function"]
pub mod b2;
#[doc = "B3 (rw) register accessor: an alias for `Reg<B3_SPEC>`"]
pub type B3 = crate::Reg<b3::B3_SPEC>;
#[doc = "y-intercept of 4th piece wise linear function"]
pub mod b3;
#[doc = "B4 (rw) register accessor: an alias for `Reg<B4_SPEC>`"]
pub type B4 = crate::Reg<b4::B4_SPEC>;
#[doc = "y-intercept of 5th piece wise linear function"]
pub mod b4;
#[doc = "B5 (rw) register accessor: an alias for `Reg<B5_SPEC>`"]
pub type B5 = crate::Reg<b5::B5_SPEC>;
#[doc = "y-intercept of 6th piece wise linear function"]
pub mod b5;
#[doc = "T0 (rw) register accessor: an alias for `Reg<T0_SPEC>`"]
pub type T0 = crate::Reg<t0::T0_SPEC>;
#[doc = "End point of 1st piece wise linear function"]
pub mod t0;
#[doc = "T1 (rw) register accessor: an alias for `Reg<T1_SPEC>`"]
pub type T1 = crate::Reg<t1::T1_SPEC>;
#[doc = "End point of 2nd piece wise linear function"]
pub mod t1;
#[doc = "T2 (rw) register accessor: an alias for `Reg<T2_SPEC>`"]
pub type T2 = crate::Reg<t2::T2_SPEC>;
#[doc = "End point of 3rd piece wise linear function"]
pub mod t2;
#[doc = "T3 (rw) register accessor: an alias for `Reg<T3_SPEC>`"]
pub type T3 = crate::Reg<t3::T3_SPEC>;
#[doc = "End point of 4th piece wise linear function"]
pub mod t3;
#[doc = "T4 (rw) register accessor: an alias for `Reg<T4_SPEC>`"]
pub type T4 = crate::Reg<t4::T4_SPEC>;
#[doc = "End point of 5th piece wise linear function"]
pub mod t4;
