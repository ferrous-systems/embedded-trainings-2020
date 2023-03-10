#[doc = "Register `T0` reader"]
pub struct R(crate::R<T0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T` reader - T (segment end)register."]
pub type T_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - T (segment end)register."]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Segment end T0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0](index.html) module"]
pub struct T0_SPEC;
impl crate::RegisterSpec for T0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0::R](R) reader structure"]
impl crate::Readable for T0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T0 to value 0xe2"]
impl crate::Resettable for T0_SPEC {
    const RESET_VALUE: Self::Ux = 0xe2;
}
