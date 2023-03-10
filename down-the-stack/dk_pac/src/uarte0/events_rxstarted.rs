#[doc = "Register `EVENTS_RXSTARTED` reader"]
pub struct R(crate::R<EVENTS_RXSTARTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_RXSTARTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_RXSTARTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_RXSTARTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_RXSTARTED` writer"]
pub struct W(crate::W<EVENTS_RXSTARTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_RXSTARTED_SPEC>;
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
impl From<crate::W<EVENTS_RXSTARTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_RXSTARTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_RXSTARTED` reader - UART receiver has started"]
pub type EVENTS_RXSTARTED_R = crate::BitReader<EVENTS_RXSTARTED_A>;
#[doc = "UART receiver has started\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_RXSTARTED_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_RXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_RXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_RXSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_RXSTARTED_A {
        match self.bits {
            false => EVENTS_RXSTARTED_A::NOT_GENERATED,
            true => EVENTS_RXSTARTED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_RXSTARTED_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_RXSTARTED_A::GENERATED
    }
}
#[doc = "Field `EVENTS_RXSTARTED` writer - UART receiver has started"]
pub type EVENTS_RXSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_RXSTARTED_SPEC, EVENTS_RXSTARTED_A, O>;
impl<'a, const O: u8> EVENTS_RXSTARTED_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_RXSTARTED_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_RXSTARTED_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - UART receiver has started"]
    #[inline(always)]
    pub fn events_rxstarted(&self) -> EVENTS_RXSTARTED_R {
        EVENTS_RXSTARTED_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART receiver has started"]
    #[inline(always)]
    #[must_use]
    pub fn events_rxstarted(&mut self) -> EVENTS_RXSTARTED_W<0> {
        EVENTS_RXSTARTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART receiver has started\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxstarted](index.html) module"]
pub struct EVENTS_RXSTARTED_SPEC;
impl crate::RegisterSpec for EVENTS_RXSTARTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_rxstarted::R](R) reader structure"]
impl crate::Readable for EVENTS_RXSTARTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_rxstarted::W](W) writer structure"]
impl crate::Writable for EVENTS_RXSTARTED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_RXSTARTED to value 0"]
impl crate::Resettable for EVENTS_RXSTARTED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
