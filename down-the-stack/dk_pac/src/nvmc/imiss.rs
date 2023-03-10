#[doc = "Register `IMISS` reader"]
pub struct R(crate::R<IMISS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMISS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMISS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMISS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMISS` writer"]
pub struct W(crate::W<IMISS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMISS_SPEC>;
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
impl From<crate::W<IMISS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMISS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MISSES` reader - Number of cache misses"]
pub type MISSES_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MISSES` writer - Number of cache misses"]
pub type MISSES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IMISS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of cache misses"]
    #[inline(always)]
    pub fn misses(&self) -> MISSES_R {
        MISSES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cache misses"]
    #[inline(always)]
    #[must_use]
    pub fn misses(&mut self) -> MISSES_W<0> {
        MISSES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I-Code cache miss counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imiss](index.html) module"]
pub struct IMISS_SPEC;
impl crate::RegisterSpec for IMISS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imiss::R](R) reader structure"]
impl crate::Readable for IMISS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imiss::W](W) writer structure"]
impl crate::Writable for IMISS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMISS to value 0"]
impl crate::Resettable for IMISS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
