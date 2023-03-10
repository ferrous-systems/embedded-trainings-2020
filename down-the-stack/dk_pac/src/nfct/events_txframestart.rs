#[doc = "Register `EVENTS_TXFRAMESTART` reader"]
pub struct R(crate::R<EVENTS_TXFRAMESTART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_TXFRAMESTART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_TXFRAMESTART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_TXFRAMESTART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_TXFRAMESTART` writer"]
pub struct W(crate::W<EVENTS_TXFRAMESTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_TXFRAMESTART_SPEC>;
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
impl From<crate::W<EVENTS_TXFRAMESTART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_TXFRAMESTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_TXFRAMESTART` reader - Marks the start of the first symbol of a transmitted frame"]
pub type EVENTS_TXFRAMESTART_R = crate::BitReader<EVENTS_TXFRAMESTART_A>;
#[doc = "Marks the start of the first symbol of a transmitted frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_TXFRAMESTART_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_TXFRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_TXFRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_TXFRAMESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_TXFRAMESTART_A {
        match self.bits {
            false => EVENTS_TXFRAMESTART_A::NOT_GENERATED,
            true => EVENTS_TXFRAMESTART_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TXFRAMESTART_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TXFRAMESTART_A::GENERATED
    }
}
#[doc = "Field `EVENTS_TXFRAMESTART` writer - Marks the start of the first symbol of a transmitted frame"]
pub type EVENTS_TXFRAMESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_TXFRAMESTART_SPEC, EVENTS_TXFRAMESTART_A, O>;
impl<'a, const O: u8> EVENTS_TXFRAMESTART_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TXFRAMESTART_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TXFRAMESTART_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Marks the start of the first symbol of a transmitted frame"]
    #[inline(always)]
    pub fn events_txframestart(&self) -> EVENTS_TXFRAMESTART_R {
        EVENTS_TXFRAMESTART_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Marks the start of the first symbol of a transmitted frame"]
    #[inline(always)]
    #[must_use]
    pub fn events_txframestart(&mut self) -> EVENTS_TXFRAMESTART_W<0> {
        EVENTS_TXFRAMESTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Marks the start of the first symbol of a transmitted frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txframestart](index.html) module"]
pub struct EVENTS_TXFRAMESTART_SPEC;
impl crate::RegisterSpec for EVENTS_TXFRAMESTART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_txframestart::R](R) reader structure"]
impl crate::Readable for EVENTS_TXFRAMESTART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_txframestart::W](W) writer structure"]
impl crate::Writable for EVENTS_TXFRAMESTART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_TXFRAMESTART to value 0"]
impl crate::Resettable for EVENTS_TXFRAMESTART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
