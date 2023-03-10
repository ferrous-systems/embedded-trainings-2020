#[doc = "Register `EVENTS_ENDKSGEN` reader"]
pub struct R(crate::R<EVENTS_ENDKSGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_ENDKSGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_ENDKSGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_ENDKSGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_ENDKSGEN` writer"]
pub struct W(crate::W<EVENTS_ENDKSGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_ENDKSGEN_SPEC>;
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
impl From<crate::W<EVENTS_ENDKSGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_ENDKSGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_ENDKSGEN` reader - Key-stream generation complete"]
pub type EVENTS_ENDKSGEN_R = crate::BitReader<EVENTS_ENDKSGEN_A>;
#[doc = "Key-stream generation complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_ENDKSGEN_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_ENDKSGEN_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ENDKSGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_ENDKSGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ENDKSGEN_A {
        match self.bits {
            false => EVENTS_ENDKSGEN_A::NOT_GENERATED,
            true => EVENTS_ENDKSGEN_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ENDKSGEN_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ENDKSGEN_A::GENERATED
    }
}
#[doc = "Field `EVENTS_ENDKSGEN` writer - Key-stream generation complete"]
pub type EVENTS_ENDKSGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_ENDKSGEN_SPEC, EVENTS_ENDKSGEN_A, O>;
impl<'a, const O: u8> EVENTS_ENDKSGEN_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDKSGEN_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDKSGEN_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Key-stream generation complete"]
    #[inline(always)]
    pub fn events_endksgen(&self) -> EVENTS_ENDKSGEN_R {
        EVENTS_ENDKSGEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Key-stream generation complete"]
    #[inline(always)]
    #[must_use]
    pub fn events_endksgen(&mut self) -> EVENTS_ENDKSGEN_W<0> {
        EVENTS_ENDKSGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key-stream generation complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endksgen](index.html) module"]
pub struct EVENTS_ENDKSGEN_SPEC;
impl crate::RegisterSpec for EVENTS_ENDKSGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_endksgen::R](R) reader structure"]
impl crate::Readable for EVENTS_ENDKSGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_endksgen::W](W) writer structure"]
impl crate::Writable for EVENTS_ENDKSGEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_ENDKSGEN to value 0"]
impl crate::Resettable for EVENTS_ENDKSGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
