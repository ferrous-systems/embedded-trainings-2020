#[doc = "Register `TASKS_STARTTX` writer"]
pub struct W(crate::W<TASKS_STARTTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_STARTTX_SPEC>;
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
impl From<crate::W<TASKS_STARTTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_STARTTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start transmission of a outgoing frame, change state to transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_STARTTX_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_STARTTX_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_STARTTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STARTTX` writer - Start transmission of a outgoing frame, change state to transmit"]
pub type TASKS_STARTTX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_STARTTX_SPEC, TASKS_STARTTX_AW, O>;
impl<'a, const O: u8> TASKS_STARTTX_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_STARTTX_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Start transmission of a outgoing frame, change state to transmit"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_starttx(&mut self) -> TASKS_STARTTX_W<0> {
        TASKS_STARTTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start transmission of a outgoing frame, change state to transmit\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_starttx](index.html) module"]
pub struct TASKS_STARTTX_SPEC;
impl crate::RegisterSpec for TASKS_STARTTX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_starttx::W](W) writer structure"]
impl crate::Writable for TASKS_STARTTX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_STARTTX to value 0"]
impl crate::Resettable for TASKS_STARTTX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
