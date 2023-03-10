#[doc = "Register `PRESCALER` reader"]
pub struct R(crate::R<PRESCALER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESCALER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESCALER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESCALER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESCALER` writer"]
pub struct W(crate::W<PRESCALER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESCALER_SPEC>;
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
impl From<crate::W<PRESCALER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESCALER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALER` reader - Pre-scaler of PWM_CLK"]
pub type PRESCALER_R = crate::FieldReader<u8, PRESCALER_A>;
#[doc = "Pre-scaler of PWM_CLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: Divide by 1 (16MHz)"]
    DIV_1 = 0,
    #[doc = "1: Divide by 2 ( 8MHz)"]
    DIV_2 = 1,
    #[doc = "2: Divide by 4 ( 4MHz)"]
    DIV_4 = 2,
    #[doc = "3: Divide by 8 ( 2MHz)"]
    DIV_8 = 3,
    #[doc = "4: Divide by 16 ( 1MHz)"]
    DIV_16 = 4,
    #[doc = "5: Divide by 32 ( 500kHz)"]
    DIV_32 = 5,
    #[doc = "6: Divide by 64 ( 250kHz)"]
    DIV_64 = 6,
    #[doc = "7: Divide by 128 ( 125kHz)"]
    DIV_128 = 7,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER_A {
        match self.bits {
            0 => PRESCALER_A::DIV_1,
            1 => PRESCALER_A::DIV_2,
            2 => PRESCALER_A::DIV_4,
            3 => PRESCALER_A::DIV_8,
            4 => PRESCALER_A::DIV_16,
            5 => PRESCALER_A::DIV_32,
            6 => PRESCALER_A::DIV_64,
            7 => PRESCALER_A::DIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV_1`"]
    #[inline(always)]
    pub fn is_div_1(&self) -> bool {
        *self == PRESCALER_A::DIV_1
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == PRESCALER_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == PRESCALER_A::DIV_4
    }
    #[doc = "Checks if the value of the field is `DIV_8`"]
    #[inline(always)]
    pub fn is_div_8(&self) -> bool {
        *self == PRESCALER_A::DIV_8
    }
    #[doc = "Checks if the value of the field is `DIV_16`"]
    #[inline(always)]
    pub fn is_div_16(&self) -> bool {
        *self == PRESCALER_A::DIV_16
    }
    #[doc = "Checks if the value of the field is `DIV_32`"]
    #[inline(always)]
    pub fn is_div_32(&self) -> bool {
        *self == PRESCALER_A::DIV_32
    }
    #[doc = "Checks if the value of the field is `DIV_64`"]
    #[inline(always)]
    pub fn is_div_64(&self) -> bool {
        *self == PRESCALER_A::DIV_64
    }
    #[doc = "Checks if the value of the field is `DIV_128`"]
    #[inline(always)]
    pub fn is_div_128(&self) -> bool {
        *self == PRESCALER_A::DIV_128
    }
}
#[doc = "Field `PRESCALER` writer - Pre-scaler of PWM_CLK"]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PRESCALER_SPEC, u8, PRESCALER_A, 3, O>;
impl<'a, const O: u8> PRESCALER_W<'a, O> {
    #[doc = "Divide by 1 (16MHz)"]
    #[inline(always)]
    pub fn div_1(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_1)
    }
    #[doc = "Divide by 2 ( 8MHz)"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_2)
    }
    #[doc = "Divide by 4 ( 4MHz)"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_4)
    }
    #[doc = "Divide by 8 ( 2MHz)"]
    #[inline(always)]
    pub fn div_8(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_8)
    }
    #[doc = "Divide by 16 ( 1MHz)"]
    #[inline(always)]
    pub fn div_16(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_16)
    }
    #[doc = "Divide by 32 ( 500kHz)"]
    #[inline(always)]
    pub fn div_32(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_32)
    }
    #[doc = "Divide by 64 ( 250kHz)"]
    #[inline(always)]
    pub fn div_64(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_64)
    }
    #[doc = "Divide by 128 ( 125kHz)"]
    #[inline(always)]
    pub fn div_128(self) -> &'a mut W {
        self.variant(PRESCALER_A::DIV_128)
    }
}
impl R {
    #[doc = "Bits 0:2 - Pre-scaler of PWM_CLK"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pre-scaler of PWM_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<0> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for PWM_CLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescaler](index.html) module"]
pub struct PRESCALER_SPEC;
impl crate::RegisterSpec for PRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prescaler::R](R) reader structure"]
impl crate::Readable for PRESCALER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prescaler::W](W) writer structure"]
impl crate::Writable for PRESCALER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRESCALER to value 0"]
impl crate::Resettable for PRESCALER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
