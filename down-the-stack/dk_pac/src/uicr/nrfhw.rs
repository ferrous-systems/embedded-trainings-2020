#[doc = "Register `NRFHW[%s]` reader"]
pub struct R(crate::R<NRFHW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRFHW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NRFHW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NRFHW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRFHW[%s]` writer"]
pub struct W(crate::W<NRFHW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRFHW_SPEC>;
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
impl From<crate::W<NRFHW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NRFHW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRFHW` reader - Reserved for Nordic hardware design"]
pub type NRFHW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NRFHW` writer - Reserved for Nordic hardware design"]
pub type NRFHW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NRFHW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw(&self) -> NRFHW_R {
        NRFHW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic hardware design"]
    #[inline(always)]
    #[must_use]
    pub fn nrfhw(&mut self) -> NRFHW_W<0> {
        NRFHW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Reserved for Nordic hardware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrfhw](index.html) module"]
pub struct NRFHW_SPEC;
impl crate::RegisterSpec for NRFHW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrfhw::R](R) reader structure"]
impl crate::Readable for NRFHW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrfhw::W](W) writer structure"]
impl crate::Writable for NRFHW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NRFHW[%s]
to value 0xffff_ffff"]
impl crate::Resettable for NRFHW_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
