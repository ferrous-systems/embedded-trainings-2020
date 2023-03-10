#[doc = "Register `PRESCALER` reader"]
pub struct R(crate::R<PRESCALER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCALER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCALER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCALER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESCALER` writer"]
pub struct W(crate::W<PRESCALER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCALER_SPEC>;
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
impl From<crate::W<PRESCALER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCALER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Prescaler value"]
pub type PRESCALER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRESCALER` writer - Prescaler value"]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PRESCALER_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Prescaler value"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Prescaler value"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "12 bit prescaler for COUNTER frequency (32768/(PRESCALER+1)).Must be written when RTC is stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescaler](index.html) module"]
pub struct PRESCALER_SPEC;
impl crate::RegisterSpec for PRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prescaler::R](R) reader structure"]
impl crate::Readable for PRESCALER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prescaler::W](W) writer structure"]
impl crate::Writable for PRESCALER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESCALER to value 0"]
impl crate::Resettable for PRESCALER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
