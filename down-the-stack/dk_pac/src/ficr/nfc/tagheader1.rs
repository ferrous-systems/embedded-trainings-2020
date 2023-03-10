#[doc = "Register `TAGHEADER1` reader"]
pub struct R(crate::R<TAGHEADER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGHEADER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGHEADER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGHEADER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UD4` reader - Unique identifier byte 4"]
pub type UD4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD5` reader - Unique identifier byte 5"]
pub type UD5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD6` reader - Unique identifier byte 6"]
pub type UD6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD7` reader - Unique identifier byte 7"]
pub type UD7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 4"]
    #[inline(always)]
    pub fn ud4(&self) -> UD4_R {
        UD4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 5"]
    #[inline(always)]
    pub fn ud5(&self) -> UD5_R {
        UD5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 6"]
    #[inline(always)]
    pub fn ud6(&self) -> UD6_R {
        UD6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 7"]
    #[inline(always)]
    pub fn ud7(&self) -> UD7_R {
        UD7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagheader1](index.html) module"]
pub struct TAGHEADER1_SPEC;
impl crate::RegisterSpec for TAGHEADER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagheader1::R](R) reader structure"]
impl crate::Readable for TAGHEADER1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGHEADER1 to value 0xffff_ffff"]
impl crate::Resettable for TAGHEADER1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
