#[doc = "Register `OVERSAMPLE` reader"]
pub struct R(crate::R<OVERSAMPLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERSAMPLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVERSAMPLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVERSAMPLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVERSAMPLE` writer"]
pub struct W(crate::W<OVERSAMPLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVERSAMPLE_SPEC>;
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
impl From<crate::W<OVERSAMPLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVERSAMPLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERSAMPLE` reader - Oversample control"]
pub type OVERSAMPLE_R = crate::FieldReader<u8, OVERSAMPLE_A>;
#[doc = "Oversample control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVERSAMPLE_A {
    #[doc = "0: Bypass oversampling"]
    BYPASS = 0,
    #[doc = "1: Oversample 2x"]
    OVER2X = 1,
    #[doc = "2: Oversample 4x"]
    OVER4X = 2,
    #[doc = "3: Oversample 8x"]
    OVER8X = 3,
    #[doc = "4: Oversample 16x"]
    OVER16X = 4,
    #[doc = "5: Oversample 32x"]
    OVER32X = 5,
    #[doc = "6: Oversample 64x"]
    OVER64X = 6,
    #[doc = "7: Oversample 128x"]
    OVER128X = 7,
    #[doc = "8: Oversample 256x"]
    OVER256X = 8,
}
impl From<OVERSAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERSAMPLE_A) -> Self {
        variant as _
    }
}
impl OVERSAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OVERSAMPLE_A> {
        match self.bits {
            0 => Some(OVERSAMPLE_A::BYPASS),
            1 => Some(OVERSAMPLE_A::OVER2X),
            2 => Some(OVERSAMPLE_A::OVER4X),
            3 => Some(OVERSAMPLE_A::OVER8X),
            4 => Some(OVERSAMPLE_A::OVER16X),
            5 => Some(OVERSAMPLE_A::OVER32X),
            6 => Some(OVERSAMPLE_A::OVER64X),
            7 => Some(OVERSAMPLE_A::OVER128X),
            8 => Some(OVERSAMPLE_A::OVER256X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == OVERSAMPLE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `OVER2X`"]
    #[inline(always)]
    pub fn is_over2x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER2X
    }
    #[doc = "Checks if the value of the field is `OVER4X`"]
    #[inline(always)]
    pub fn is_over4x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER4X
    }
    #[doc = "Checks if the value of the field is `OVER8X`"]
    #[inline(always)]
    pub fn is_over8x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER8X
    }
    #[doc = "Checks if the value of the field is `OVER16X`"]
    #[inline(always)]
    pub fn is_over16x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER16X
    }
    #[doc = "Checks if the value of the field is `OVER32X`"]
    #[inline(always)]
    pub fn is_over32x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER32X
    }
    #[doc = "Checks if the value of the field is `OVER64X`"]
    #[inline(always)]
    pub fn is_over64x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER64X
    }
    #[doc = "Checks if the value of the field is `OVER128X`"]
    #[inline(always)]
    pub fn is_over128x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER128X
    }
    #[doc = "Checks if the value of the field is `OVER256X`"]
    #[inline(always)]
    pub fn is_over256x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER256X
    }
}
#[doc = "Field `OVERSAMPLE` writer - Oversample control"]
pub type OVERSAMPLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OVERSAMPLE_SPEC, u8, OVERSAMPLE_A, 4, O>;
impl<'a, const O: u8> OVERSAMPLE_W<'a, O> {
    #[doc = "Bypass oversampling"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::BYPASS)
    }
    #[doc = "Oversample 2x"]
    #[inline(always)]
    pub fn over2x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER2X)
    }
    #[doc = "Oversample 4x"]
    #[inline(always)]
    pub fn over4x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER4X)
    }
    #[doc = "Oversample 8x"]
    #[inline(always)]
    pub fn over8x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER8X)
    }
    #[doc = "Oversample 16x"]
    #[inline(always)]
    pub fn over16x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER16X)
    }
    #[doc = "Oversample 32x"]
    #[inline(always)]
    pub fn over32x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER32X)
    }
    #[doc = "Oversample 64x"]
    #[inline(always)]
    pub fn over64x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER64X)
    }
    #[doc = "Oversample 128x"]
    #[inline(always)]
    pub fn over128x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER128X)
    }
    #[doc = "Oversample 256x"]
    #[inline(always)]
    pub fn over256x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER256X)
    }
}
impl R {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    pub fn oversample(&self) -> OVERSAMPLE_R {
        OVERSAMPLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    #[must_use]
    pub fn oversample(&mut self) -> OVERSAMPLE_W<0> {
        OVERSAMPLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oversampling configuration. OVERSAMPLE should not be combined with SCAN. The RESOLUTION is applied before averaging, thus for high OVERSAMPLE a higher RESOLUTION should be used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oversample](index.html) module"]
pub struct OVERSAMPLE_SPEC;
impl crate::RegisterSpec for OVERSAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oversample::R](R) reader structure"]
impl crate::Readable for OVERSAMPLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oversample::W](W) writer structure"]
impl crate::Writable for OVERSAMPLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OVERSAMPLE to value 0"]
impl crate::Resettable for OVERSAMPLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
