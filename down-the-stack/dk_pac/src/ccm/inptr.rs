#[doc = "Register `INPTR` reader"]
pub struct R(crate::R<INPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPTR` writer"]
pub struct W(crate::W<INPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPTR_SPEC>;
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
impl From<crate::W<INPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPTR` reader - Input pointer"]
pub type INPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INPTR` writer - Input pointer"]
pub type INPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INPTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Input pointer"]
    #[inline(always)]
    pub fn inptr(&self) -> INPTR_R {
        INPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input pointer"]
    #[inline(always)]
    #[must_use]
    pub fn inptr(&mut self) -> INPTR_W<0> {
        INPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inptr](index.html) module"]
pub struct INPTR_SPEC;
impl crate::RegisterSpec for INPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inptr::R](R) reader structure"]
impl crate::Readable for INPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inptr::W](W) writer structure"]
impl crate::Writable for INPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INPTR to value 0"]
impl crate::Resettable for INPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
