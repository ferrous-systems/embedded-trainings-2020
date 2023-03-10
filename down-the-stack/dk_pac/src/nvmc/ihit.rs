#[doc = "Register `IHIT` reader"]
pub struct R(crate::R<IHIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IHIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IHIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IHIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IHIT` writer"]
pub struct W(crate::W<IHIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IHIT_SPEC>;
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
impl From<crate::W<IHIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IHIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HITS` reader - Number of cache hits"]
pub type HITS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HITS` writer - Number of cache hits"]
pub type HITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IHIT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Number of cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HITS_R {
        HITS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cache hits"]
    #[inline(always)]
    #[must_use]
    pub fn hits(&mut self) -> HITS_W<0> {
        HITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I-Code cache hit counter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ihit](index.html) module"]
pub struct IHIT_SPEC;
impl crate::RegisterSpec for IHIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ihit::R](R) reader structure"]
impl crate::Readable for IHIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ihit::W](W) writer structure"]
impl crate::Writable for IHIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IHIT to value 0"]
impl crate::Resettable for IHIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
