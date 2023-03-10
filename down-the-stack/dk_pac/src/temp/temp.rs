#[doc = "Register `TEMP` reader"]
pub struct R(crate::R<TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TEMP` reader - Temperature in degC (0.25deg steps)"]
pub type TEMP_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Temperature in degC (0.25deg steps)"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(self.bits)
    }
}
#[doc = "Temperature in degC (0.25deg steps)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp](index.html) module"]
pub struct TEMP_SPEC;
impl crate::RegisterSpec for TEMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [temp::R](R) reader structure"]
impl crate::Readable for TEMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TEMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
