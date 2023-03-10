#[doc = "Register `TASKS_OUT[%s]` writer"]
pub struct W(crate::W<TASKS_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_OUT_SPEC>;
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
impl From<crate::W<TASKS_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_OUT_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_OUT_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_OUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_OUT` writer - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
pub type TASKS_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TASKS_OUT_SPEC, TASKS_OUT_AW, O>;
impl<'a, const O: u8> TASKS_OUT_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_OUT_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY."]
    #[inline(always)]
    #[must_use]
    pub fn tasks_out(&mut self) -> TASKS_OUT_W<0> {
        TASKS_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Task for writing to pin specified in CONFIG\\[n\\].PSEL. Action on pin is configured in CONFIG\\[n\\].POLARITY.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_out](index.html) module"]
pub struct TASKS_OUT_SPEC;
impl crate::RegisterSpec for TASKS_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_out::W](W) writer structure"]
impl crate::Writable for TASKS_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_OUT[%s]
to value 0"]
impl crate::Resettable for TASKS_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
