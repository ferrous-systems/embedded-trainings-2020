#[doc = r"Register block"]
#[repr(C)]
pub struct NFC {
    #[doc = "0x00 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader0: TAGHEADER0,
    #[doc = "0x04 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader1: TAGHEADER1,
    #[doc = "0x08 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader2: TAGHEADER2,
    #[doc = "0x0c - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader3: TAGHEADER3,
}
#[doc = "TAGHEADER0 (r) register accessor: an alias for `Reg<TAGHEADER0_SPEC>`"]
pub type TAGHEADER0 = crate::Reg<tagheader0::TAGHEADER0_SPEC>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader0;
#[doc = "TAGHEADER1 (r) register accessor: an alias for `Reg<TAGHEADER1_SPEC>`"]
pub type TAGHEADER1 = crate::Reg<tagheader1::TAGHEADER1_SPEC>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader1;
#[doc = "TAGHEADER2 (r) register accessor: an alias for `Reg<TAGHEADER2_SPEC>`"]
pub type TAGHEADER2 = crate::Reg<tagheader2::TAGHEADER2_SPEC>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader2;
#[doc = "TAGHEADER3 (r) register accessor: an alias for `Reg<TAGHEADER3_SPEC>`"]
pub type TAGHEADER3 = crate::Reg<tagheader3::TAGHEADER3_SPEC>;
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
pub mod tagheader3;
