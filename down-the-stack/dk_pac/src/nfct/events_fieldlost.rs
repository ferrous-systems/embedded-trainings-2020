#[doc = "Register `EVENTS_FIELDLOST` reader"]
pub struct R(crate::R<EVENTS_FIELDLOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_FIELDLOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_FIELDLOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_FIELDLOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_FIELDLOST` writer"]
pub struct W(crate::W<EVENTS_FIELDLOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_FIELDLOST_SPEC>;
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
impl From<crate::W<EVENTS_FIELDLOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_FIELDLOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_FIELDLOST` reader - Remote NFC field lost"]
pub type EVENTS_FIELDLOST_R = crate::BitReader<EVENTS_FIELDLOST_A>;
#[doc = "Remote NFC field lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_FIELDLOST_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_FIELDLOST_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_FIELDLOST_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_FIELDLOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_FIELDLOST_A {
        match self.bits {
            false => EVENTS_FIELDLOST_A::NOT_GENERATED,
            true => EVENTS_FIELDLOST_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_FIELDLOST_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_FIELDLOST_A::GENERATED
    }
}
#[doc = "Field `EVENTS_FIELDLOST` writer - Remote NFC field lost"]
pub type EVENTS_FIELDLOST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_FIELDLOST_SPEC, EVENTS_FIELDLOST_A, O>;
impl<'a, const O: u8> EVENTS_FIELDLOST_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_FIELDLOST_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_FIELDLOST_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Remote NFC field lost"]
    #[inline(always)]
    pub fn events_fieldlost(&self) -> EVENTS_FIELDLOST_R {
        EVENTS_FIELDLOST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote NFC field lost"]
    #[inline(always)]
    #[must_use]
    pub fn events_fieldlost(&mut self) -> EVENTS_FIELDLOST_W<0> {
        EVENTS_FIELDLOST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remote NFC field lost\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_fieldlost](index.html) module"]
pub struct EVENTS_FIELDLOST_SPEC;
impl crate::RegisterSpec for EVENTS_FIELDLOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_fieldlost::R](R) reader structure"]
impl crate::Readable for EVENTS_FIELDLOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_fieldlost::W](W) writer structure"]
impl crate::Writable for EVENTS_FIELDLOST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_FIELDLOST to value 0"]
impl crate::Resettable for EVENTS_FIELDLOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
