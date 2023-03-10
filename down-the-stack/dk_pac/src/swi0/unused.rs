#[doc = "Register `UNUSED` reader"]
pub struct R(crate::R<UNUSED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNUSED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNUSED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNUSED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Unused.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused](index.html) module"]
pub struct UNUSED_SPEC;
impl crate::RegisterSpec for UNUSED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unused::R](R) reader structure"]
impl crate::Readable for UNUSED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNUSED to value 0"]
impl crate::Resettable for UNUSED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
