#[doc = "Register `T1` reader"]
pub struct R(crate::R<T1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1_SPEC>) -> Self {
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
#[doc = "Segment end T1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1](index.html) module"]
pub struct T1_SPEC;
impl crate::RegisterSpec for T1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1::R](R) reader structure"]
impl crate::Readable for T1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T1 to value 0"]
impl crate::Resettable for T1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}