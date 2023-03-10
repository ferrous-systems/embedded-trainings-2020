#[doc = "Register `B1` reader"]
pub struct R(crate::R<B1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `B` reader - B (y-intercept)"]
pub type B_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - B (y-intercept)"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "y-intercept B1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b1](index.html) module"]
pub struct B1_SPEC;
impl crate::RegisterSpec for B1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b1::R](R) reader structure"]
impl crate::Readable for B1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets B1 to value 0x3f98"]
impl crate::Resettable for B1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f98;
}
