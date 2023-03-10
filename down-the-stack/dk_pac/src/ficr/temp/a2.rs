#[doc = "Register `A2` reader"]
pub struct R(crate::R<A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `A` reader - A (slope definition) register."]
pub type A_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - A (slope definition) register."]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Slope definition A2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a2](index.html) module"]
pub struct A2_SPEC;
impl crate::RegisterSpec for A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a2::R](R) reader structure"]
impl crate::Readable for A2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets A2 to value 0x035d"]
impl crate::Resettable for A2_SPEC {
    const RESET_VALUE: Self::Ux = 0x035d;
}
