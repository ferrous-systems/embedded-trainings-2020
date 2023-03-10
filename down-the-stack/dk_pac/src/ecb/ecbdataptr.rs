#[doc = "Register `ECBDATAPTR` reader"]
pub struct R(crate::R<ECBDATAPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECBDATAPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECBDATAPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECBDATAPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECBDATAPTR` writer"]
pub struct W(crate::W<ECBDATAPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECBDATAPTR_SPEC>;
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
impl From<crate::W<ECBDATAPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECBDATAPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECBDATAPTR` reader - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
pub type ECBDATAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ECBDATAPTR` writer - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
pub type ECBDATAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ECBDATAPTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    pub fn ecbdataptr(&self) -> ECBDATAPTR_R {
        ECBDATAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    #[must_use]
    pub fn ecbdataptr(&mut self) -> ECBDATAPTR_W<0> {
        ECBDATAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECB block encrypt memory pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecbdataptr](index.html) module"]
pub struct ECBDATAPTR_SPEC;
impl crate::RegisterSpec for ECBDATAPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecbdataptr::R](R) reader structure"]
impl crate::Readable for ECBDATAPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecbdataptr::W](W) writer structure"]
impl crate::Writable for ECBDATAPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECBDATAPTR to value 0"]
impl crate::Resettable for ECBDATAPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
