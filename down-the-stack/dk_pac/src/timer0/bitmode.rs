#[doc = "Register `BITMODE` reader"]
pub struct R(crate::R<BITMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BITMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BITMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BITMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BITMODE` writer"]
pub struct W(crate::W<BITMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BITMODE_SPEC>;
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
impl From<crate::W<BITMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BITMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITMODE` reader - Timer bit width"]
pub type BITMODE_R = crate::FieldReader<u8, BITMODE_A>;
#[doc = "Timer bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITMODE_A {
    #[doc = "0: 16 bit timer bit width"]
    _16BIT = 0,
    #[doc = "1: 8 bit timer bit width"]
    _08BIT = 1,
    #[doc = "2: 24 bit timer bit width"]
    _24BIT = 2,
    #[doc = "3: 32 bit timer bit width"]
    _32BIT = 3,
}
impl From<BITMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BITMODE_A) -> Self {
        variant as _
    }
}
impl BITMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITMODE_A {
        match self.bits {
            0 => BITMODE_A::_16BIT,
            1 => BITMODE_A::_08BIT,
            2 => BITMODE_A::_24BIT,
            3 => BITMODE_A::_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == BITMODE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_08BIT`"]
    #[inline(always)]
    pub fn is_08bit(&self) -> bool {
        *self == BITMODE_A::_08BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == BITMODE_A::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == BITMODE_A::_32BIT
    }
}
#[doc = "Field `BITMODE` writer - Timer bit width"]
pub type BITMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, BITMODE_SPEC, u8, BITMODE_A, 2, O>;
impl<'a, const O: u8> BITMODE_W<'a, O> {
    #[doc = "16 bit timer bit width"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_16BIT)
    }
    #[doc = "8 bit timer bit width"]
    #[inline(always)]
    pub fn _08bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_08BIT)
    }
    #[doc = "24 bit timer bit width"]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_24BIT)
    }
    #[doc = "32 bit timer bit width"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_32BIT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline(always)]
    pub fn bitmode(&self) -> BITMODE_R {
        BITMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline(always)]
    #[must_use]
    pub fn bitmode(&mut self) -> BITMODE_W<0> {
        BITMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure the number of bits used by the TIMER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bitmode](index.html) module"]
pub struct BITMODE_SPEC;
impl crate::RegisterSpec for BITMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bitmode::R](R) reader structure"]
impl crate::Readable for BITMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bitmode::W](W) writer structure"]
impl crate::Writable for BITMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BITMODE to value 0"]
impl crate::Resettable for BITMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
