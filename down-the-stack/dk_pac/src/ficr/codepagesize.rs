#[doc = "Register `CODEPAGESIZE` reader"]
pub struct R(crate::R<CODEPAGESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEPAGESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEPAGESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEPAGESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CODEPAGESIZE` reader - Code memory page size"]
pub type CODEPAGESIZE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Code memory page size"]
    #[inline(always)]
    pub fn codepagesize(&self) -> CODEPAGESIZE_R {
        CODEPAGESIZE_R::new(self.bits)
    }
}
#[doc = "Code memory page size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codepagesize](index.html) module"]
pub struct CODEPAGESIZE_SPEC;
impl crate::RegisterSpec for CODEPAGESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codepagesize::R](R) reader structure"]
impl crate::Readable for CODEPAGESIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CODEPAGESIZE to value 0xffff_ffff"]
impl crate::Resettable for CODEPAGESIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
