#[doc = "Register `GAINR` reader"]
pub struct R(crate::R<GAINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAINR` writer"]
pub struct W(crate::W<GAINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAINR_SPEC>;
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
impl From<crate::W<GAINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAINR` reader - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
pub type GAINR_R = crate::FieldReader<u8, GAINR_A>;
#[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAINR_A {
    #[doc = "0: -20dB gain adjustment (minimum)"]
    MIN_GAIN = 0,
    #[doc = "40: 0dB gain adjustment ('2500 RMS' requirement)"]
    DEFAULT_GAIN = 40,
    #[doc = "80: +20dB gain adjustment (maximum)"]
    MAX_GAIN = 80,
}
impl From<GAINR_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINR_A) -> Self {
        variant as _
    }
}
impl GAINR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAINR_A> {
        match self.bits {
            0 => Some(GAINR_A::MIN_GAIN),
            40 => Some(GAINR_A::DEFAULT_GAIN),
            80 => Some(GAINR_A::MAX_GAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN_GAIN`"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        *self == GAINR_A::MIN_GAIN
    }
    #[doc = "Checks if the value of the field is `DEFAULT_GAIN`"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        *self == GAINR_A::DEFAULT_GAIN
    }
    #[doc = "Checks if the value of the field is `MAX_GAIN`"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        *self == GAINR_A::MAX_GAIN
    }
}
#[doc = "Field `GAINR` writer - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
pub type GAINR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAINR_SPEC, u8, GAINR_A, 8, O>;
impl<'a, const O: u8> GAINR_W<'a, O> {
    #[doc = "-20dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut W {
        self.variant(GAINR_A::MIN_GAIN)
    }
    #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut W {
        self.variant(GAINR_A::DEFAULT_GAIN)
    }
    #[doc = "+20dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut W {
        self.variant(GAINR_A::MAX_GAIN)
    }
}
impl R {
    #[doc = "Bits 0:7 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn gainr(&self) -> GAINR_R {
        GAINR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    #[must_use]
    pub fn gainr(&mut self) -> GAINR_W<0> {
        GAINR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Right output gain adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gainr](index.html) module"]
pub struct GAINR_SPEC;
impl crate::RegisterSpec for GAINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gainr::R](R) reader structure"]
impl crate::Readable for GAINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gainr::W](W) writer structure"]
impl crate::Writable for GAINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAINR to value 0x28"]
impl crate::Resettable for GAINR_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
