#[doc = "Register `SAMPLERATE` reader"]
pub struct R(crate::R<SAMPLERATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLERATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLERATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLERATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLERATE` writer"]
pub struct W(crate::W<SAMPLERATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLERATE_SPEC>;
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
impl From<crate::W<SAMPLERATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLERATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC` reader - Capture and compare value. Sample rate is 16 MHz/CC"]
pub type CC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CC` writer - Capture and compare value. Sample rate is 16 MHz/CC"]
pub type CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMPLERATE_SPEC, u16, u16, 11, O>;
#[doc = "Field `MODE` reader - Select mode for sample rate control"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Select mode for sample rate control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: Rate is controlled from SAMPLE task"]
    TASK = 0,
    #[doc = "1: Rate is controlled from local timer (use CC to control the rate)"]
    TIMERS = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::TASK,
            true => MODE_A::TIMERS,
        }
    }
    #[doc = "Checks if the value of the field is `TASK`"]
    #[inline(always)]
    pub fn is_task(&self) -> bool {
        *self == MODE_A::TASK
    }
    #[doc = "Checks if the value of the field is `TIMERS`"]
    #[inline(always)]
    pub fn is_timers(&self) -> bool {
        *self == MODE_A::TIMERS
    }
}
#[doc = "Field `MODE` writer - Select mode for sample rate control"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAMPLERATE_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Rate is controlled from SAMPLE task"]
    #[inline(always)]
    pub fn task(self) -> &'a mut W {
        self.variant(MODE_A::TASK)
    }
    #[doc = "Rate is controlled from local timer (use CC to control the rate)"]
    #[inline(always)]
    pub fn timers(self) -> &'a mut W {
        self.variant(MODE_A::TIMERS)
    }
}
impl R {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Capture and compare value. Sample rate is 16 MHz/CC"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<0> {
        CC_W::new(self)
    }
    #[doc = "Bit 12 - Select mode for sample rate control"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<12> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls normal or continuous sample rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samplerate](index.html) module"]
pub struct SAMPLERATE_SPEC;
impl crate::RegisterSpec for SAMPLERATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [samplerate::R](R) reader structure"]
impl crate::Readable for SAMPLERATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [samplerate::W](W) writer structure"]
impl crate::Writable for SAMPLERATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLERATE to value 0"]
impl crate::Resettable for SAMPLERATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
