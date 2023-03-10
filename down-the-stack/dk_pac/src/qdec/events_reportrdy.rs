#[doc = "Register `EVENTS_REPORTRDY` reader"]
pub struct R(crate::R<EVENTS_REPORTRDY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_REPORTRDY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_REPORTRDY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_REPORTRDY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_REPORTRDY` writer"]
pub struct W(crate::W<EVENTS_REPORTRDY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_REPORTRDY_SPEC>;
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
impl From<crate::W<EVENTS_REPORTRDY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_REPORTRDY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_REPORTRDY` reader - Non-null report ready"]
pub type EVENTS_REPORTRDY_R = crate::BitReader<EVENTS_REPORTRDY_A>;
#[doc = "Non-null report ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_REPORTRDY_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_REPORTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_REPORTRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_REPORTRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_REPORTRDY_A {
        match self.bits {
            false => EVENTS_REPORTRDY_A::NOT_GENERATED,
            true => EVENTS_REPORTRDY_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_REPORTRDY_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_REPORTRDY_A::GENERATED
    }
}
#[doc = "Field `EVENTS_REPORTRDY` writer - Non-null report ready"]
pub type EVENTS_REPORTRDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_REPORTRDY_SPEC, EVENTS_REPORTRDY_A, O>;
impl<'a, const O: u8> EVENTS_REPORTRDY_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_REPORTRDY_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_REPORTRDY_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Non-null report ready"]
    #[inline(always)]
    pub fn events_reportrdy(&self) -> EVENTS_REPORTRDY_R {
        EVENTS_REPORTRDY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-null report ready"]
    #[inline(always)]
    #[must_use]
    pub fn events_reportrdy(&mut self) -> EVENTS_REPORTRDY_W<0> {
        EVENTS_REPORTRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-null report ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_reportrdy](index.html) module"]
pub struct EVENTS_REPORTRDY_SPEC;
impl crate::RegisterSpec for EVENTS_REPORTRDY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_reportrdy::R](R) reader structure"]
impl crate::Readable for EVENTS_REPORTRDY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_reportrdy::W](W) writer structure"]
impl crate::Writable for EVENTS_REPORTRDY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_REPORTRDY to value 0"]
impl crate::Resettable for EVENTS_REPORTRDY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
