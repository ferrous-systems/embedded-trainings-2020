#[doc = "Register `ACC` reader"]
pub struct R(crate::R<ACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACC` reader - Register accumulating all valid samples (not double transition) read from the SAMPLE register"]
pub type ACC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register accumulating all valid samples (not double transition) read from the SAMPLE register"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(self.bits)
    }
}
#[doc = "Register accumulating the valid transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc](index.html) module"]
pub struct ACC_SPEC;
impl crate::RegisterSpec for ACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acc::R](R) reader structure"]
impl crate::Readable for ACC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACC to value 0"]
impl crate::Resettable for ACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
