#[doc = "Register `TASKS_FLUSHRX` writer"]
pub struct W(crate::W<TASKS_FLUSHRX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_FLUSHRX_SPEC>;
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
impl From<crate::W<TASKS_FLUSHRX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_FLUSHRX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flush RX FIFO into RX buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_FLUSHRX_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_FLUSHRX_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_FLUSHRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_FLUSHRX` writer - Flush RX FIFO into RX buffer"]
pub type TASKS_FLUSHRX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_FLUSHRX_SPEC, TASKS_FLUSHRX_AW, O>;
impl<'a, const O: u8> TASKS_FLUSHRX_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_FLUSHRX_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Flush RX FIFO into RX buffer"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_flushrx(&mut self) -> TASKS_FLUSHRX_W<0> {
        TASKS_FLUSHRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flush RX FIFO into RX buffer\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_flushrx](index.html) module"]
pub struct TASKS_FLUSHRX_SPEC;
impl crate::RegisterSpec for TASKS_FLUSHRX_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_flushrx::W](W) writer structure"]
impl crate::Writable for TASKS_FLUSHRX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_FLUSHRX to value 0"]
impl crate::Resettable for TASKS_FLUSHRX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
