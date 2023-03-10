#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SP` reader - Speed and power modes"]
pub type SP_R = crate::FieldReader<u8, SP_A>;
#[doc = "Speed and power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SP_A {
    #[doc = "0: Low-power mode"]
    LOW = 0,
    #[doc = "1: Normal mode"]
    NORMAL = 1,
    #[doc = "2: High-speed mode"]
    HIGH = 2,
}
impl From<SP_A> for u8 {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as _
    }
}
impl SP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SP_A> {
        match self.bits {
            0 => Some(SP_A::LOW),
            1 => Some(SP_A::NORMAL),
            2 => Some(SP_A::HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SP_A::LOW
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SP_A::HIGH
    }
}
#[doc = "Field `SP` writer - Speed and power modes"]
pub type SP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, SP_A, 2, O>;
impl<'a, const O: u8> SP_W<'a, O> {
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SP_A::LOW)
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SP_A::NORMAL)
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SP_A::HIGH)
    }
}
#[doc = "Field `MAIN` reader - Main operation modes"]
pub type MAIN_R = crate::BitReader<MAIN_A>;
#[doc = "Main operation modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAIN_A {
    #[doc = "0: Single-ended mode"]
    SE = 0,
    #[doc = "1: Differential mode"]
    DIFF = 1,
}
impl From<MAIN_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_A) -> Self {
        variant as u8 != 0
    }
}
impl MAIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_A {
        match self.bits {
            false => MAIN_A::SE,
            true => MAIN_A::DIFF,
        }
    }
    #[doc = "Checks if the value of the field is `SE`"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        *self == MAIN_A::SE
    }
    #[doc = "Checks if the value of the field is `DIFF`"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        *self == MAIN_A::DIFF
    }
}
#[doc = "Field `MAIN` writer - Main operation modes"]
pub type MAIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, MAIN_A, O>;
impl<'a, const O: u8> MAIN_W<'a, O> {
    #[doc = "Single-ended mode"]
    #[inline(always)]
    pub fn se(self) -> &'a mut W {
        self.variant(MAIN_A::SE)
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut W {
        self.variant(MAIN_A::DIFF)
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    pub fn main(&self) -> MAIN_R {
        MAIN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<0> {
        SP_W::new(self)
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    #[must_use]
    pub fn main(&mut self) -> MAIN_W<8> {
        MAIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
