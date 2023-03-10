#[doc = "Register `TAGHEADER3` reader"]
pub struct R(crate::R<TAGHEADER3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGHEADER3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGHEADER3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGHEADER3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UD12` reader - Unique identifier byte 12"]
pub type UD12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD13` reader - Unique identifier byte 13"]
pub type UD13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD14` reader - Unique identifier byte 14"]
pub type UD14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UD15` reader - Unique identifier byte 15"]
pub type UD15_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 12"]
    #[inline(always)]
    pub fn ud12(&self) -> UD12_R {
        UD12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 13"]
    #[inline(always)]
    pub fn ud13(&self) -> UD13_R {
        UD13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 14"]
    #[inline(always)]
    pub fn ud14(&self) -> UD14_R {
        UD14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 15"]
    #[inline(always)]
    pub fn ud15(&self) -> UD15_R {
        UD15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagheader3](index.html) module"]
pub struct TAGHEADER3_SPEC;
impl crate::RegisterSpec for TAGHEADER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagheader3::R](R) reader structure"]
impl crate::Readable for TAGHEADER3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAGHEADER3 to value 0xffff_ffff"]
impl crate::Resettable for TAGHEADER3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
