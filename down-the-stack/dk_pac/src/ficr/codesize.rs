#[doc = "Register `CODESIZE` reader"]
pub struct R(crate::R<CODESIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODESIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODESIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODESIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CODESIZE` reader - Code memory size in number of pages"]
pub type CODESIZE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Code memory size in number of pages"]
    #[inline(always)]
    pub fn codesize(&self) -> CODESIZE_R {
        CODESIZE_R::new(self.bits)
    }
}
#[doc = "Code memory size\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codesize](index.html) module"]
pub struct CODESIZE_SPEC;
impl crate::RegisterSpec for CODESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codesize::R](R) reader structure"]
impl crate::Readable for CODESIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CODESIZE to value 0xffff_ffff"]
impl crate::Resettable for CODESIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
