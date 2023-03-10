#[doc = "Register `EVENTS_LFCLKSTARTED` reader"]
pub struct R(crate::R<EVENTS_LFCLKSTARTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_LFCLKSTARTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_LFCLKSTARTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_LFCLKSTARTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_LFCLKSTARTED` writer"]
pub struct W(crate::W<EVENTS_LFCLKSTARTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_LFCLKSTARTED_SPEC>;
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
impl From<crate::W<EVENTS_LFCLKSTARTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_LFCLKSTARTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_LFCLKSTARTED` reader - LFCLK started"]
pub type EVENTS_LFCLKSTARTED_R = crate::BitReader<EVENTS_LFCLKSTARTED_A>;
#[doc = "LFCLK started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_LFCLKSTARTED_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_LFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_LFCLKSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_LFCLKSTARTED_A {
        match self.bits {
            false => EVENTS_LFCLKSTARTED_A::NOT_GENERATED,
            true => EVENTS_LFCLKSTARTED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_LFCLKSTARTED_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_LFCLKSTARTED_A::GENERATED
    }
}
#[doc = "Field `EVENTS_LFCLKSTARTED` writer - LFCLK started"]
pub type EVENTS_LFCLKSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_LFCLKSTARTED_SPEC, EVENTS_LFCLKSTARTED_A, O>;
impl<'a, const O: u8> EVENTS_LFCLKSTARTED_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_LFCLKSTARTED_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_LFCLKSTARTED_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - LFCLK started"]
    #[inline(always)]
    pub fn events_lfclkstarted(&self) -> EVENTS_LFCLKSTARTED_R {
        EVENTS_LFCLKSTARTED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFCLK started"]
    #[inline(always)]
    #[must_use]
    pub fn events_lfclkstarted(&mut self) -> EVENTS_LFCLKSTARTED_W<0> {
        EVENTS_LFCLKSTARTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LFCLK started\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_lfclkstarted](index.html) module"]
pub struct EVENTS_LFCLKSTARTED_SPEC;
impl crate::RegisterSpec for EVENTS_LFCLKSTARTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_lfclkstarted::R](R) reader structure"]
impl crate::Readable for EVENTS_LFCLKSTARTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_lfclkstarted::W](W) writer structure"]
impl crate::Writable for EVENTS_LFCLKSTARTED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_LFCLKSTARTED to value 0"]
impl crate::Resettable for EVENTS_LFCLKSTARTED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
