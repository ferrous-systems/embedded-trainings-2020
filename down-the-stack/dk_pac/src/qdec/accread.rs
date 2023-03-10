#[doc = "Register `ACCREAD` reader"]
pub struct R(crate::R<ACCREAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCREAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACCREAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACCREAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACCREAD` reader - Snapshot of the ACC register."]
pub type ACCREAD_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Snapshot of the ACC register."]
    #[inline(always)]
    pub fn accread(&self) -> ACCREAD_R {
        ACCREAD_R::new(self.bits)
    }
}
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accread](index.html) module"]
pub struct ACCREAD_SPEC;
impl crate::RegisterSpec for ACCREAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [accread::R](R) reader structure"]
impl crate::Readable for ACCREAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACCREAD to value 0"]
impl crate::Resettable for ACCREAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
