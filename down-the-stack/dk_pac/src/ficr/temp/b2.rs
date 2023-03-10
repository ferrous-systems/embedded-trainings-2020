#[doc = "Register `B2` reader"]
pub struct R(crate::R<B2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B2_SPEC>) -> Self {
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
#[doc = "y-intercept B2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b2](index.html) module"]
pub struct B2_SPEC;
impl crate::RegisterSpec for B2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b2::R](R) reader structure"]
impl crate::Readable for B2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets B2 to value 0x3f98"]
impl crate::Resettable for B2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f98;
}
