#[doc = r"Register block"]
#[repr(C)]
pub struct SEQ {
    #[doc = "0x00 - Description cluster: Beginning address in Data RAM of this sequence"]
    pub ptr: PTR,
    #[doc = "0x04 - Description cluster: Amount of values (duty cycles) in this sequence"]
    pub cnt: CNT,
    #[doc = "0x08 - Description cluster: Amount of additional PWM periods between samples loaded into compare register"]
    pub refresh: REFRESH,
    #[doc = "0x0c - Description cluster: Time added after the sequence"]
    pub enddelay: ENDDELAY,
}
#[doc = "PTR (rw) register accessor: an alias for `Reg<PTR_SPEC>`"]
pub type PTR = crate::Reg<ptr::PTR_SPEC>;
#[doc = "Description cluster: Beginning address in Data RAM of this sequence"]
pub mod ptr;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Description cluster: Amount of values (duty cycles) in this sequence"]
pub mod cnt;
#[doc = "REFRESH (rw) register accessor: an alias for `Reg<REFRESH_SPEC>`"]
pub type REFRESH = crate::Reg<refresh::REFRESH_SPEC>;
#[doc = "Description cluster: Amount of additional PWM periods between samples loaded into compare register"]
pub mod refresh;
#[doc = "ENDDELAY (rw) register accessor: an alias for `Reg<ENDDELAY_SPEC>`"]
pub type ENDDELAY = crate::Reg<enddelay::ENDDELAY_SPEC>;
#[doc = "Description cluster: Time added after the sequence"]
pub mod enddelay;
