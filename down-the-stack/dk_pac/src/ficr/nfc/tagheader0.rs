#[doc = "Register `TAGHEADER0` reader"]
pub struct R(crate::R<TAGHEADER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGHEADER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGHEADER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGHEADER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFGID` reader - Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
pub type MFGID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD1` reader - Unique identifier byte 1"]
pub type UD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD2` reader - Unique identifier byte 2"]
pub type UD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD3` reader - Unique identifier byte 3"]
pub type UD3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline(always)]
    pub fn mfgid(&self) -> MFGID_R {
        MFGID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 1"]
    #[inline(always)]
    pub fn ud1(&self) -> UD1_R {
        UD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 2"]
    #[inline(always)]
    pub fn ud2(&self) -> UD2_R {
        UD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 3"]
    #[inline(always)]
    pub fn ud3(&self) -> UD3_R {
        UD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagheader0](index.html) module"]
pub struct TAGHEADER0_SPEC;
impl crate::RegisterSpec for TAGHEADER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagheader0::R](R) reader structure"]
impl crate::Readable for TAGHEADER0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGHEADER0 to value 0xffff_ff5f"]
impl crate::Resettable for TAGHEADER0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff5f;
}
