#[doc = "Register `TASKS_LFCLKSTART` writer"]
pub struct W(crate::W<TASKS_LFCLKSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_LFCLKSTART_SPEC>;
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
impl From<crate::W<TASKS_LFCLKSTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TASKS_LFCLKSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start LFCLK source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TASKS_LFCLKSTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_LFCLKSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_LFCLKSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_LFCLKSTART` writer - Start LFCLK source"]
pub type TASKS_LFCLKSTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TASKS_LFCLKSTART_SPEC, TASKS_LFCLKSTART_AW, O>;
impl<'a, const O: u8> TASKS_LFCLKSTART_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_LFCLKSTART_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Start LFCLK source"]
    #[inline(always)]
    #[must_use]
    pub fn tasks_lfclkstart(&mut self) -> TASKS_LFCLKSTART_W<0> {
        TASKS_LFCLKSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start LFCLK source\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_lfclkstart](index.html) module"]
pub struct TASKS_LFCLKSTART_SPEC;
impl crate::RegisterSpec for TASKS_LFCLKSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_lfclkstart::W](W) writer structure"]
impl crate::Writable for TASKS_LFCLKSTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TASKS_LFCLKSTART to value 0"]
impl crate::Resettable for TASKS_LFCLKSTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
