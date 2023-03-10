#[doc = r"Register block"]
#[repr(C)]
pub struct INFO {
    #[doc = "0x00 - Part code"]
    pub part: PART,
    #[doc = "0x04 - Part Variant, Hardware version and Production configuration"]
    pub variant: VARIANT,
    #[doc = "0x08 - Package option"]
    pub package: PACKAGE,
    #[doc = "0x0c - RAM variant"]
    pub ram: RAM,
    #[doc = "0x10 - Flash variant"]
    pub flash: FLASH,
}
#[doc = "PART (r) register accessor: an alias for `Reg<PART_SPEC>`"]
pub type PART = crate::Reg<part::PART_SPEC>;
#[doc = "Part code"]
pub mod part;
#[doc = "VARIANT (r) register accessor: an alias for `Reg<VARIANT_SPEC>`"]
pub type VARIANT = crate::Reg<variant::VARIANT_SPEC>;
#[doc = "Part Variant, Hardware version and Production configuration"]
pub mod variant;
#[doc = "PACKAGE (r) register accessor: an alias for `Reg<PACKAGE_SPEC>`"]
pub type PACKAGE = crate::Reg<package::PACKAGE_SPEC>;
#[doc = "Package option"]
pub mod package;
#[doc = "RAM (r) register accessor: an alias for `Reg<RAM_SPEC>`"]
pub type RAM = crate::Reg<ram::RAM_SPEC>;
#[doc = "RAM variant"]
pub mod ram;
#[doc = "FLASH (r) register accessor: an alias for `Reg<FLASH_SPEC>`"]
pub type FLASH = crate::Reg<flash::FLASH_SPEC>;
#[doc = "Flash variant"]
pub mod flash;
