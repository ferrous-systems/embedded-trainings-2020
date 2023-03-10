#[doc = "Register `LEDPOL` reader"]
pub struct R(crate::R<LEDPOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEDPOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEDPOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEDPOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEDPOL` writer"]
pub struct W(crate::W<LEDPOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEDPOL_SPEC>;
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
impl From<crate::W<LEDPOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEDPOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEDPOL` reader - LED output pin polarity"]
pub type LEDPOL_R = crate::BitReader<LEDPOL_A>;
#[doc = "LED output pin polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEDPOL_A {
    #[doc = "0: Led active on output pin low"]
    ACTIVE_LOW = 0,
    #[doc = "1: Led active on output pin high"]
    ACTIVE_HIGH = 1,
}
impl From<LEDPOL_A> for bool {
    #[inline(always)]
    fn from(variant: LEDPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl LEDPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEDPOL_A {
        match self.bits {
            false => LEDPOL_A::ACTIVE_LOW,
            true => LEDPOL_A::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == LEDPOL_A::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == LEDPOL_A::ACTIVE_HIGH
    }
}
#[doc = "Field `LEDPOL` writer - LED output pin polarity"]
pub type LEDPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEDPOL_SPEC, LEDPOL_A, O>;
impl<'a, const O: u8> LEDPOL_W<'a, O> {
    #[doc = "Led active on output pin low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(LEDPOL_A::ACTIVE_LOW)
    }
    #[doc = "Led active on output pin high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(LEDPOL_A::ACTIVE_HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    pub fn ledpol(&self) -> LEDPOL_R {
        LEDPOL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LED output pin polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ledpol(&mut self) -> LEDPOL_W<0> {
        LEDPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LED output pin polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledpol](index.html) module"]
pub struct LEDPOL_SPEC;
impl crate::RegisterSpec for LEDPOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ledpol::R](R) reader structure"]
impl crate::Readable for LEDPOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ledpol::W](W) writer structure"]
impl crate::Writable for LEDPOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LEDPOL to value 0"]
impl crate::Resettable for LEDPOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
