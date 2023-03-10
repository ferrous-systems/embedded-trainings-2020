#[doc = "Register `EVENTS_TICK` reader"]
pub struct R(crate::R<EVENTS_TICK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_TICK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_TICK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_TICK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_TICK` writer"]
pub struct W(crate::W<EVENTS_TICK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_TICK_SPEC>;
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
impl From<crate::W<EVENTS_TICK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_TICK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_TICK` reader - Event on COUNTER increment"]
pub type EVENTS_TICK_R = crate::BitReader<EVENTS_TICK_A>;
#[doc = "Event on COUNTER increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_TICK_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_TICK_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_TICK_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_TICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_TICK_A {
        match self.bits {
            false => EVENTS_TICK_A::NOT_GENERATED,
            true => EVENTS_TICK_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TICK_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TICK_A::GENERATED
    }
}
#[doc = "Field `EVENTS_TICK` writer - Event on COUNTER increment"]
pub type EVENTS_TICK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_TICK_SPEC, EVENTS_TICK_A, O>;
impl<'a, const O: u8> EVENTS_TICK_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TICK_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TICK_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Event on COUNTER increment"]
    #[inline(always)]
    pub fn events_tick(&self) -> EVENTS_TICK_R {
        EVENTS_TICK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event on COUNTER increment"]
    #[inline(always)]
    #[must_use]
    pub fn events_tick(&mut self) -> EVENTS_TICK_W<0> {
        EVENTS_TICK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event on COUNTER increment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_tick](index.html) module"]
pub struct EVENTS_TICK_SPEC;
impl crate::RegisterSpec for EVENTS_TICK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_tick::R](R) reader structure"]
impl crate::Readable for EVENTS_TICK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_tick::W](W) writer structure"]
impl crate::Writable for EVENTS_TICK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_TICK to value 0"]
impl crate::Resettable for EVENTS_TICK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
