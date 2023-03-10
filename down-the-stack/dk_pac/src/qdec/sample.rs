#[doc = "Register `SAMPLE` reader"]
pub struct R(crate::R<SAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SAMPLE` reader - Last motion sample"]
pub type SAMPLE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last motion sample"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(self.bits)
    }
}
#[doc = "Motion sample value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample](index.html) module"]
pub struct SAMPLE_SPEC;
impl crate::RegisterSpec for SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sample::R](R) reader structure"]
impl crate::Readable for SAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAMPLE to value 0"]
impl crate::Resettable for SAMPLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
