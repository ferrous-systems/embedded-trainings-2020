#[doc = "Register `NRFFW[%s]` reader"]
pub struct R(crate::R<NRFFW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRFFW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRFFW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRFFW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRFFW[%s]` writer"]
pub struct W(crate::W<NRFFW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRFFW_SPEC>;
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
impl From<crate::W<NRFFW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRFFW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRFFW` reader - Reserved for Nordic firmware design"]
pub type NRFFW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NRFFW` writer - Reserved for Nordic firmware design"]
pub type NRFFW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NRFFW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(&self) -> NRFFW_R {
        NRFFW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    #[must_use]
    pub fn nrffw(&mut self) -> NRFFW_W<0> {
        NRFFW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Reserved for Nordic firmware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrffw](index.html) module"]
pub struct NRFFW_SPEC;
impl crate::RegisterSpec for NRFFW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrffw::R](R) reader structure"]
impl crate::Readable for NRFFW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrffw::W](W) writer structure"]
impl crate::Writable for NRFFW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRFFW[%s]
to value 0xffff_ffff"]
impl crate::Resettable for NRFFW_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
