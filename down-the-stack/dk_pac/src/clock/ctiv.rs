#[doc = "Register `CTIV` reader"]
pub struct R(crate::R<CTIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTIV` writer"]
pub struct W(crate::W<CTIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIV_SPEC>;
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
impl From<crate::W<CTIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTIV` reader - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
pub type CTIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTIV` writer - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
pub type CTIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTIV_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn ctiv(&self) -> CTIV_R {
        CTIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    #[must_use]
    pub fn ctiv(&mut self) -> CTIV_W<0> {
        CTIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration timer interval\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiv](index.html) module"]
pub struct CTIV_SPEC;
impl crate::RegisterSpec for CTIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctiv::R](R) reader structure"]
impl crate::Readable for CTIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctiv::W](W) writer structure"]
impl crate::Writable for CTIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTIV to value 0"]
impl crate::Resettable for CTIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
