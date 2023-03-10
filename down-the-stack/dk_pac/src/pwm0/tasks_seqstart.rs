#[doc = "Register `TASKS_SEQSTART[%s]` writer"]
pub struct W(crate::W<TASKS_SEQSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_SEQSTART_SPEC>;
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
impl From<crate::W<TASKS_SEQSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_SEQSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start it was not running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_SEQSTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_SEQSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_SEQSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SEQSTART` writer - Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start it was not running."]
pub type TASKS_SEQSTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_SEQSTART_SPEC, TASKS_SEQSTART_AW, O>;
impl<'a, const O: u8> TASKS_SEQSTART_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_SEQSTART_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start it was not running."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_seqstart(&mut self) -> TASKS_SEQSTART_W<0> {
        TASKS_SEQSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Loads the first PWM value on all enabled channels from sequence n, and starts playing that sequence at the rate defined in SEQ\\[n\\]REFRESH and/or DECODER.MODE. Causes PWM generation to start it was not running.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_seqstart](index.html) module"]
pub struct TASKS_SEQSTART_SPEC;
impl crate::RegisterSpec for TASKS_SEQSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_seqstart::W](W) writer structure"]
impl crate::Writable for TASKS_SEQSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_SEQSTART[%s]
to value 0"]
impl crate::Resettable for TASKS_SEQSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
