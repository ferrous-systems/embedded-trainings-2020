#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - Ready flag"]
    pub ready: READY,
    _reserved1: [u8; 0x0100],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    _reserved_2_erasepage: [u8; 0x04],
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    pub erasepcr0: ERASEPCR0,
    #[doc = "0x514 - Register for erasing User Information Configuration Registers"]
    pub eraseuicr: ERASEUICR,
    _reserved6: [u8; 0x28],
    #[doc = "0x540 - I-Code cache configuration register."]
    pub icachecnf: ICACHECNF,
    _reserved7: [u8; 0x04],
    #[doc = "0x548 - I-Code cache hit counter."]
    pub ihit: IHIT,
    #[doc = "0x54c - I-Code cache miss counter."]
    pub imiss: IMISS,
}
impl RegisterBlock {
    #[doc = "0x508 - Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    #[inline(always)]
    pub const fn erasepcr1(&self) -> &ERASEPCR1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(1288usize).cast() }
    }
    #[doc = "0x508 - Register for erasing a page in Code area"]
    #[inline(always)]
    pub const fn erasepage(&self) -> &ERASEPAGE {
        unsafe { &*(self as *const Self).cast::<u8>().add(1288usize).cast() }
    }
}
#[doc = "READY (r) register accessor: an alias for `Reg<READY_SPEC>`"]
pub type READY = crate::Reg<ready::READY_SPEC>;
#[doc = "Ready flag"]
pub mod ready;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "ERASEPAGE (rw) register accessor: an alias for `Reg<ERASEPAGE_SPEC>`"]
pub type ERASEPAGE = crate::Reg<erasepage::ERASEPAGE_SPEC>;
#[doc = "Register for erasing a page in Code area"]
pub mod erasepage;
#[doc = "ERASEPCR1 (rw) register accessor: an alias for `Reg<ERASEPCR1_SPEC>`"]
pub type ERASEPCR1 = crate::Reg<erasepcr1::ERASEPCR1_SPEC>;
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub mod erasepcr1;
#[doc = "ERASEALL (rw) register accessor: an alias for `Reg<ERASEALL_SPEC>`"]
pub type ERASEALL = crate::Reg<eraseall::ERASEALL_SPEC>;
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "ERASEPCR0 (rw) register accessor: an alias for `Reg<ERASEPCR0_SPEC>`"]
pub type ERASEPCR0 = crate::Reg<erasepcr0::ERASEPCR0_SPEC>;
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub mod erasepcr0;
#[doc = "ERASEUICR (rw) register accessor: an alias for `Reg<ERASEUICR_SPEC>`"]
pub type ERASEUICR = crate::Reg<eraseuicr::ERASEUICR_SPEC>;
#[doc = "Register for erasing User Information Configuration Registers"]
pub mod eraseuicr;
#[doc = "ICACHECNF (rw) register accessor: an alias for `Reg<ICACHECNF_SPEC>`"]
pub type ICACHECNF = crate::Reg<icachecnf::ICACHECNF_SPEC>;
#[doc = "I-Code cache configuration register."]
pub mod icachecnf;
#[doc = "IHIT (rw) register accessor: an alias for `Reg<IHIT_SPEC>`"]
pub type IHIT = crate::Reg<ihit::IHIT_SPEC>;
#[doc = "I-Code cache hit counter."]
pub mod ihit;
#[doc = "IMISS (rw) register accessor: an alias for `Reg<IMISS_SPEC>`"]
pub type IMISS = crate::Reg<imiss::IMISS_SPEC>;
#[doc = "I-Code cache miss counter."]
pub mod imiss;
