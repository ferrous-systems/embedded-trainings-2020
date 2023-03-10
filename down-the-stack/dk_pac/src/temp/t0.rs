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
#[doc = "Register `T0` writer"]
pub struct W(crate::W<T0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<T0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0` reader - End point of 1st piece wise linear function"]
pub type T0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T0` writer - End point of 1st piece wise linear function"]
pub type T0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(&self) -> T0_R {
        T0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn t0(&mut self) -> T0_W<0> {
        T0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End point of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0](index.html) module"]
pub struct T0_SPEC;
impl crate::RegisterSpec for T0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0::R](R) reader structure"]
impl crate::Readable for T0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0::W](W) writer structure"]
impl crate::Writable for T0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0 to value 0xe2"]
impl crate::Resettable for T0_SPEC {
    const RESET_VALUE: Self::Ux = 0xe2;
}
