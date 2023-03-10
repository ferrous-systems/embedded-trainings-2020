#[doc = "Register `RSSISAMPLE` reader"]
pub struct R(crate::R<RSSISAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSSISAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSSISAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSSISAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RSSISAMPLE` reader - RSSI sample"]
pub type RSSISAMPLE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - RSSI sample"]
    #[inline(always)]
    pub fn rssisample(&self) -> RSSISAMPLE_R {
        RSSISAMPLE_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "RSSI sample\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rssisample](index.html) module"]
pub struct RSSISAMPLE_SPEC;
impl crate::RegisterSpec for RSSISAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rssisample::R](R) reader structure"]
impl crate::Readable for RSSISAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSSISAMPLE to value 0"]
impl crate::Resettable for RSSISAMPLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
