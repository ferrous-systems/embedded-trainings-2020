#[doc = "Register `B4` reader"]
pub struct R(crate::R<B4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B4_SPEC>) -> Self {
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
#[doc = "y-intercept B4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b4](index.html) module"]
pub struct B4_SPEC;
impl crate::RegisterSpec for B4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b4::R](R) reader structure"]
impl crate::Readable for B4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets B4 to value 0x4d"]
impl crate::Resettable for B4_SPEC {
    const RESET_VALUE: Self::Ux = 0x4d;
}
