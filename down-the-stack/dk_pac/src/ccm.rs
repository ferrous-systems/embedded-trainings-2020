#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of key-stream. This operation will stop by itself when completed."]
    pub tasks_ksgen: TASKS_KSGEN,
    #[doc = "0x04 - Start encryption/decryption. This operation will stop by itself when completed."]
    pub tasks_crypt: TASKS_CRYPT,
    #[doc = "0x08 - Stop encryption/decryption"]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 0xf4],
    #[doc = "0x100 - Key-stream generation complete"]
    pub events_endksgen: EVENTS_ENDKSGEN,
    #[doc = "0x104 - Encrypt/decrypt complete"]
    pub events_endcrypt: EVENTS_ENDCRYPT,
    #[doc = "0x108 - CCM error event"]
    pub events_error: EVENTS_ERROR,
    _reserved6: [u8; 0xf4],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved7: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 0xf4],
    #[doc = "0x400 - MIC check result"]
    pub micstatus: MICSTATUS,
    _reserved10: [u8; 0xfc],
    #[doc = "0x500 - Enable"]
    pub enable: ENABLE,
    #[doc = "0x504 - Operation mode"]
    pub mode: MODE,
    #[doc = "0x508 - Pointer to data structure holding AES key and NONCE vector"]
    pub cnfptr: CNFPTR,
    #[doc = "0x50c - Input pointer"]
    pub inptr: INPTR,
    #[doc = "0x510 - Output pointer"]
    pub outptr: OUTPTR,
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    pub scratchptr: SCRATCHPTR,
}
#[doc = "TASKS_KSGEN (w) register accessor: an alias for `Reg<TASKS_KSGEN_SPEC>`"]
pub type TASKS_KSGEN = crate::Reg<tasks_ksgen::TASKS_KSGEN_SPEC>;
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "TASKS_CRYPT (w) register accessor: an alias for `Reg<TASKS_CRYPT_SPEC>`"]
pub type TASKS_CRYPT = crate::Reg<tasks_crypt::TASKS_CRYPT_SPEC>;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop encryption/decryption"]
pub mod tasks_stop;
#[doc = "EVENTS_ENDKSGEN (rw) register accessor: an alias for `Reg<EVENTS_ENDKSGEN_SPEC>`"]
pub type EVENTS_ENDKSGEN = crate::Reg<events_endksgen::EVENTS_ENDKSGEN_SPEC>;
#[doc = "Key-stream generation complete"]
pub mod events_endksgen;
#[doc = "EVENTS_ENDCRYPT (rw) register accessor: an alias for `Reg<EVENTS_ENDCRYPT_SPEC>`"]
pub type EVENTS_ENDCRYPT = crate::Reg<events_endcrypt::EVENTS_ENDCRYPT_SPEC>;
#[doc = "Encrypt/decrypt complete"]
pub mod events_endcrypt;
#[doc = "EVENTS_ERROR (rw) register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "CCM error event"]
pub mod events_error;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "MICSTATUS (r) register accessor: an alias for `Reg<MICSTATUS_SPEC>`"]
pub type MICSTATUS = crate::Reg<micstatus::MICSTATUS_SPEC>;
#[doc = "MIC check result"]
pub mod micstatus;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable"]
pub mod enable;
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Operation mode"]
pub mod mode;
#[doc = "CNFPTR (rw) register accessor: an alias for `Reg<CNFPTR_SPEC>`"]
pub type CNFPTR = crate::Reg<cnfptr::CNFPTR_SPEC>;
#[doc = "Pointer to data structure holding AES key and NONCE vector"]
pub mod cnfptr;
#[doc = "INPTR (rw) register accessor: an alias for `Reg<INPTR_SPEC>`"]
pub type INPTR = crate::Reg<inptr::INPTR_SPEC>;
#[doc = "Input pointer"]
pub mod inptr;
#[doc = "OUTPTR (rw) register accessor: an alias for `Reg<OUTPTR_SPEC>`"]
pub type OUTPTR = crate::Reg<outptr::OUTPTR_SPEC>;
#[doc = "Output pointer"]
pub mod outptr;
#[doc = "SCRATCHPTR (rw) register accessor: an alias for `Reg<SCRATCHPTR_SPEC>`"]
pub type SCRATCHPTR = crate::Reg<scratchptr::SCRATCHPTR_SPEC>;
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
