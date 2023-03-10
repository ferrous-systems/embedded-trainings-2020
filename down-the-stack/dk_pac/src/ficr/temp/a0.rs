#[doc = "Register `A0` reader"]
pub struct R(crate::R<A0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A0_SPEC>) -> Self {
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
#[doc = "Slope definition A0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a0](index.html) module"]
pub struct A0_SPEC;
impl crate::RegisterSpec for A0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a0::R](R) reader structure"]
impl crate::Readable for A0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets A0 to value 0x0320"]
impl crate::Resettable for A0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0320;
}
