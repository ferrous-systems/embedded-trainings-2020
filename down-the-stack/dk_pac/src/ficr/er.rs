#[doc = "Register `ER[%s]` reader"]
pub struct R(crate::R<ER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ER` reader - Encryption Root, word n"]
pub type ER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Encryption Root, word n"]
    #[inline(always)]
    pub fn er(&self) -> ER_R {
        ER_R::new(self.bits)
    }
}
#[doc = "Description collection: Encryption Root, word n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [er](index.html) module"]
pub struct ER_SPEC;
impl crate::RegisterSpec for ER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [er::R](R) reader structure"]
impl crate::Readable for ER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ER[%s]
to value 0xffff_ffff"]
impl crate::Resettable for ER_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
