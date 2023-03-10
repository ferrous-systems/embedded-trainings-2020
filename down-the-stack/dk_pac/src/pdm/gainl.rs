#[doc = "Register `GAINL` reader"]
pub struct R(crate::R<GAINL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAINL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAINL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAINL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAINL` writer"]
pub struct W(crate::W<GAINL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAINL_SPEC>;
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
impl From<crate::W<GAINL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAINL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAINL` reader - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
pub type GAINL_R = crate::FieldReader<u8, GAINL_A>;
#[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAINL_A {
    #[doc = "0: -20dB gain adjustment (minimum)"]
    MIN_GAIN = 0,
    #[doc = "40: 0dB gain adjustment ('2500 RMS' requirement)"]
    DEFAULT_GAIN = 40,
    #[doc = "80: +20dB gain adjustment (maximum)"]
    MAX_GAIN = 80,
}
impl From<GAINL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINL_A) -> Self {
        variant as _
    }
}
impl GAINL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAINL_A> {
        match self.bits {
            0 => Some(GAINL_A::MIN_GAIN),
            40 => Some(GAINL_A::DEFAULT_GAIN),
            80 => Some(GAINL_A::MAX_GAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN_GAIN`"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        *self == GAINL_A::MIN_GAIN
    }
    #[doc = "Checks if the value of the field is `DEFAULT_GAIN`"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        *self == GAINL_A::DEFAULT_GAIN
    }
    #[doc = "Checks if the value of the field is `MAX_GAIN`"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        *self == GAINL_A::MAX_GAIN
    }
}
#[doc = "Field `GAINL` writer - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
pub type GAINL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAINL_SPEC, u8, GAINL_A, 7, O>;
impl<'a, const O: u8> GAINL_W<'a, O> {
    #[doc = "-20dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut W {
        self.variant(GAINL_A::MIN_GAIN)
    }
    #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut W {
        self.variant(GAINL_A::DEFAULT_GAIN)
    }
    #[doc = "+20dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut W {
        self.variant(GAINL_A::MAX_GAIN)
    }
}
impl R {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn gainl(&self) -> GAINL_R {
        GAINL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    #[must_use]
    pub fn gainl(&mut self) -> GAINL_W<0> {
        GAINL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Left output gain adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gainl](index.html) module"]
pub struct GAINL_SPEC;
impl crate::RegisterSpec for GAINL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gainl::R](R) reader structure"]
impl crate::Readable for GAINL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gainl::W](W) writer structure"]
impl crate::Writable for GAINL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAINL to value 0x28"]
impl crate::Resettable for GAINL_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
