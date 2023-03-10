#[doc = "Register `ADDRPTR` reader"]
pub struct R(crate::R<ADDRPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRPTR` writer"]
pub struct W(crate::W<ADDRPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRPTR_SPEC>;
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
impl From<crate::W<ADDRPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRPTR` reader - Pointer to the resolvable address (6-bytes)"]
pub type ADDRPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRPTR` writer - Pointer to the resolvable address (6-bytes)"]
pub type ADDRPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDRPTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Pointer to the resolvable address (6-bytes)"]
    #[inline(always)]
    pub fn addrptr(&self) -> ADDRPTR_R {
        ADDRPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the resolvable address (6-bytes)"]
    #[inline(always)]
    #[must_use]
    pub fn addrptr(&mut self) -> ADDRPTR_W<0> {
        ADDRPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pointer to the resolvable address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrptr](index.html) module"]
pub struct ADDRPTR_SPEC;
impl crate::RegisterSpec for ADDRPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrptr::R](R) reader structure"]
impl crate::Readable for ADDRPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrptr::W](W) writer structure"]
impl crate::Writable for ADDRPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRPTR to value 0"]
impl crate::Resettable for ADDRPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
