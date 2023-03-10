#[doc = "Register `EVENTS_ERRORECB` reader"]
pub struct R(crate::R<EVENTS_ERRORECB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_ERRORECB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_ERRORECB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_ERRORECB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_ERRORECB` writer"]
pub struct W(crate::W<EVENTS_ERRORECB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_ERRORECB_SPEC>;
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
impl From<crate::W<EVENTS_ERRORECB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_ERRORECB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_ERRORECB` reader - ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub type EVENTS_ERRORECB_R = crate::BitReader<EVENTS_ERRORECB_A>;
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_ERRORECB_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_ERRORECB_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ERRORECB_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_ERRORECB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ERRORECB_A {
        match self.bits {
            false => EVENTS_ERRORECB_A::NOT_GENERATED,
            true => EVENTS_ERRORECB_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ERRORECB_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ERRORECB_A::GENERATED
    }
}
#[doc = "Field `EVENTS_ERRORECB` writer - ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub type EVENTS_ERRORECB_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_ERRORECB_SPEC, EVENTS_ERRORECB_A, O>;
impl<'a, const O: u8> EVENTS_ERRORECB_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ERRORECB_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ERRORECB_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    #[inline(always)]
    pub fn events_errorecb(&self) -> EVENTS_ERRORECB_R {
        EVENTS_ERRORECB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    #[inline(always)]
    #[must_use]
    pub fn events_errorecb(&mut self) -> EVENTS_ERRORECB_W<0> {
        EVENTS_ERRORECB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_errorecb](index.html) module"]
pub struct EVENTS_ERRORECB_SPEC;
impl crate::RegisterSpec for EVENTS_ERRORECB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_errorecb::R](R) reader structure"]
impl crate::Readable for EVENTS_ERRORECB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_errorecb::W](W) writer structure"]
impl crate::Writable for EVENTS_ERRORECB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_ERRORECB to value 0"]
impl crate::Resettable for EVENTS_ERRORECB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
