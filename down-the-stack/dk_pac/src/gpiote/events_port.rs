#[doc = "Register `EVENTS_PORT` reader"]
pub struct R(crate::R<EVENTS_PORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_PORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_PORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_PORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_PORT` writer"]
pub struct W(crate::W<EVENTS_PORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_PORT_SPEC>;
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
impl From<crate::W<EVENTS_PORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_PORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_PORT` reader - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub type EVENTS_PORT_R = crate::BitReader<EVENTS_PORT_A>;
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_PORT_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_PORT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_PORT_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_PORT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_PORT_A {
        match self.bits {
            false => EVENTS_PORT_A::NOT_GENERATED,
            true => EVENTS_PORT_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_PORT_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_PORT_A::GENERATED
    }
}
#[doc = "Field `EVENTS_PORT` writer - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
pub type EVENTS_PORT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_PORT_SPEC, EVENTS_PORT_A, O>;
impl<'a, const O: u8> EVENTS_PORT_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_PORT_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_PORT_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    pub fn events_port(&self) -> EVENTS_PORT_R {
        EVENTS_PORT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event generated from multiple input GPIO pins with SENSE mechanism enabled"]
    #[inline(always)]
    #[must_use]
    pub fn events_port(&mut self) -> EVENTS_PORT_W<0> {
        EVENTS_PORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event generated from multiple input GPIO pins with SENSE mechanism enabled\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_port](index.html) module"]
pub struct EVENTS_PORT_SPEC;
impl crate::RegisterSpec for EVENTS_PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_port::R](R) reader structure"]
impl crate::Readable for EVENTS_PORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_port::W](W) writer structure"]
impl crate::Writable for EVENTS_PORT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_PORT to value 0"]
impl crate::Resettable for EVENTS_PORT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
