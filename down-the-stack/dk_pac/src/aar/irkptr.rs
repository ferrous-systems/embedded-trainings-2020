#[doc = "Register `IRKPTR` reader"]
pub struct R(crate::R<IRKPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRKPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRKPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRKPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRKPTR` writer"]
pub struct W(crate::W<IRKPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRKPTR_SPEC>;
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
impl From<crate::W<IRKPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRKPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRKPTR` reader - Pointer to the IRK data structure"]
pub type IRKPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IRKPTR` writer - Pointer to the IRK data structure"]
pub type IRKPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRKPTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Pointer to the IRK data structure"]
    #[inline(always)]
    pub fn irkptr(&self) -> IRKPTR_R {
        IRKPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the IRK data structure"]
    #[inline(always)]
    #[must_use]
    pub fn irkptr(&mut self) -> IRKPTR_W<0> {
        IRKPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pointer to IRK data structure\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irkptr](index.html) module"]
pub struct IRKPTR_SPEC;
impl crate::RegisterSpec for IRKPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irkptr::R](R) reader structure"]
impl crate::Readable for IRKPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irkptr::W](W) writer structure"]
impl crate::Writable for IRKPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRKPTR to value 0"]
impl crate::Resettable for IRKPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
