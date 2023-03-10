#[doc = "Register `RXMATCH` reader"]
pub struct R(crate::R<RXMATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXMATCH` reader - Received address"]
pub type RXMATCH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Received address"]
    #[inline(always)]
    pub fn rxmatch(&self) -> RXMATCH_R {
        RXMATCH_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Received address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmatch](index.html) module"]
pub struct RXMATCH_SPEC;
impl crate::RegisterSpec for RXMATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmatch::R](R) reader structure"]
impl crate::Readable for RXMATCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXMATCH to value 0"]
impl crate::Resettable for RXMATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
