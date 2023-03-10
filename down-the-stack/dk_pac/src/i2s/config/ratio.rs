#[doc = "Register `RATIO` reader"]
pub struct R(crate::R<RATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATIO` writer"]
pub struct W(crate::W<RATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATIO_SPEC>;
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
impl From<crate::W<RATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RATIO` reader - MCK / LRCK ratio."]
pub type RATIO_R = crate::FieldReader<u8, RATIO_A>;
#[doc = "MCK / LRCK ratio.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "0: LRCK = MCK / 32"]
    _32X = 0,
    #[doc = "1: LRCK = MCK / 48"]
    _48X = 1,
    #[doc = "2: LRCK = MCK / 64"]
    _64X = 2,
    #[doc = "3: LRCK = MCK / 96"]
    _96X = 3,
    #[doc = "4: LRCK = MCK / 128"]
    _128X = 4,
    #[doc = "5: LRCK = MCK / 192"]
    _192X = 5,
    #[doc = "6: LRCK = MCK / 256"]
    _256X = 6,
    #[doc = "7: LRCK = MCK / 384"]
    _384X = 7,
    #[doc = "8: LRCK = MCK / 512"]
    _512X = 8,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
impl RATIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RATIO_A> {
        match self.bits {
            0 => Some(RATIO_A::_32X),
            1 => Some(RATIO_A::_48X),
            2 => Some(RATIO_A::_64X),
            3 => Some(RATIO_A::_96X),
            4 => Some(RATIO_A::_128X),
            5 => Some(RATIO_A::_192X),
            6 => Some(RATIO_A::_256X),
            7 => Some(RATIO_A::_384X),
            8 => Some(RATIO_A::_512X),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_32X`"]
    #[inline(always)]
    pub fn is_32x(&self) -> bool {
        *self == RATIO_A::_32X
    }
    #[doc = "Checks if the value of the field is `_48X`"]
    #[inline(always)]
    pub fn is_48x(&self) -> bool {
        *self == RATIO_A::_48X
    }
    #[doc = "Checks if the value of the field is `_64X`"]
    #[inline(always)]
    pub fn is_64x(&self) -> bool {
        *self == RATIO_A::_64X
    }
    #[doc = "Checks if the value of the field is `_96X`"]
    #[inline(always)]
    pub fn is_96x(&self) -> bool {
        *self == RATIO_A::_96X
    }
    #[doc = "Checks if the value of the field is `_128X`"]
    #[inline(always)]
    pub fn is_128x(&self) -> bool {
        *self == RATIO_A::_128X
    }
    #[doc = "Checks if the value of the field is `_192X`"]
    #[inline(always)]
    pub fn is_192x(&self) -> bool {
        *self == RATIO_A::_192X
    }
    #[doc = "Checks if the value of the field is `_256X`"]
    #[inline(always)]
    pub fn is_256x(&self) -> bool {
        *self == RATIO_A::_256X
    }
    #[doc = "Checks if the value of the field is `_384X`"]
    #[inline(always)]
    pub fn is_384x(&self) -> bool {
        *self == RATIO_A::_384X
    }
    #[doc = "Checks if the value of the field is `_512X`"]
    #[inline(always)]
    pub fn is_512x(&self) -> bool {
        *self == RATIO_A::_512X
    }
}
#[doc = "Field `RATIO` writer - MCK / LRCK ratio."]
pub type RATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RATIO_SPEC, u8, RATIO_A, 4, O>;
impl<'a, const O: u8> RATIO_W<'a, O> {
    #[doc = "LRCK = MCK / 32"]
    #[inline(always)]
    pub fn _32x(self) -> &'a mut W {
        self.variant(RATIO_A::_32X)
    }
    #[doc = "LRCK = MCK / 48"]
    #[inline(always)]
    pub fn _48x(self) -> &'a mut W {
        self.variant(RATIO_A::_48X)
    }
    #[doc = "LRCK = MCK / 64"]
    #[inline(always)]
    pub fn _64x(self) -> &'a mut W {
        self.variant(RATIO_A::_64X)
    }
    #[doc = "LRCK = MCK / 96"]
    #[inline(always)]
    pub fn _96x(self) -> &'a mut W {
        self.variant(RATIO_A::_96X)
    }
    #[doc = "LRCK = MCK / 128"]
    #[inline(always)]
    pub fn _128x(self) -> &'a mut W {
        self.variant(RATIO_A::_128X)
    }
    #[doc = "LRCK = MCK / 192"]
    #[inline(always)]
    pub fn _192x(self) -> &'a mut W {
        self.variant(RATIO_A::_192X)
    }
    #[doc = "LRCK = MCK / 256"]
    #[inline(always)]
    pub fn _256x(self) -> &'a mut W {
        self.variant(RATIO_A::_256X)
    }
    #[doc = "LRCK = MCK / 384"]
    #[inline(always)]
    pub fn _384x(self) -> &'a mut W {
        self.variant(RATIO_A::_384X)
    }
    #[doc = "LRCK = MCK / 512"]
    #[inline(always)]
    pub fn _512x(self) -> &'a mut W {
        self.variant(RATIO_A::_512X)
    }
}
impl R {
    #[doc = "Bits 0:3 - MCK / LRCK ratio."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCK / LRCK ratio."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RATIO_W<0> {
        RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCK / LRCK ratio.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratio](index.html) module"]
pub struct RATIO_SPEC;
impl crate::RegisterSpec for RATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ratio::R](R) reader structure"]
impl crate::Readable for RATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ratio::W](W) writer structure"]
impl crate::Writable for RATIO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RATIO to value 0x06"]
impl crate::Resettable for RATIO_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
