#[doc = "Register `PSEL` reader"]
pub struct R(crate::R<PSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSEL` writer"]
pub struct W(crate::W<PSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSEL_SPEC>;
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
impl From<crate::W<PSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSEL` reader - Analog pin select"]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "Analog pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: AIN0 selected as analog input"]
    ANALOG_INPUT0 = 0,
    #[doc = "1: AIN1 selected as analog input"]
    ANALOG_INPUT1 = 1,
    #[doc = "2: AIN2 selected as analog input"]
    ANALOG_INPUT2 = 2,
    #[doc = "3: AIN3 selected as analog input"]
    ANALOG_INPUT3 = 3,
    #[doc = "4: AIN4 selected as analog input"]
    ANALOG_INPUT4 = 4,
    #[doc = "5: AIN5 selected as analog input"]
    ANALOG_INPUT5 = 5,
    #[doc = "6: AIN6 selected as analog input"]
    ANALOG_INPUT6 = 6,
    #[doc = "7: AIN7 selected as analog input"]
    ANALOG_INPUT7 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::ANALOG_INPUT0,
            1 => PSEL_A::ANALOG_INPUT1,
            2 => PSEL_A::ANALOG_INPUT2,
            3 => PSEL_A::ANALOG_INPUT3,
            4 => PSEL_A::ANALOG_INPUT4,
            5 => PSEL_A::ANALOG_INPUT5,
            6 => PSEL_A::ANALOG_INPUT6,
            7 => PSEL_A::ANALOG_INPUT7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT0`"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT0
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT1`"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT1
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT2`"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT2
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT3`"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT3
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT4`"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT4
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT5`"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT5
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT6`"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT6
    }
    #[doc = "Checks if the value of the field is `ANALOG_INPUT7`"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        *self == PSEL_A::ANALOG_INPUT7
    }
}
#[doc = "Field `PSEL` writer - Analog pin select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PSEL_SPEC, u8, PSEL_A, 3, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "AIN0 selected as analog input"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT0)
    }
    #[doc = "AIN1 selected as analog input"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT1)
    }
    #[doc = "AIN2 selected as analog input"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT2)
    }
    #[doc = "AIN3 selected as analog input"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT3)
    }
    #[doc = "AIN4 selected as analog input"]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT4)
    }
    #[doc = "AIN5 selected as analog input"]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT5)
    }
    #[doc = "AIN6 selected as analog input"]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT6)
    }
    #[doc = "AIN7 selected as analog input"]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut W {
        self.variant(PSEL_A::ANALOG_INPUT7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Analog pin select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<0> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psel](index.html) module"]
pub struct PSEL_SPEC;
impl crate::RegisterSpec for PSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psel::R](R) reader structure"]
impl crate::Readable for PSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psel::W](W) writer structure"]
impl crate::Writable for PSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSEL to value 0"]
impl crate::Resettable for PSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
