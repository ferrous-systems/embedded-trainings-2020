#[doc = "Register `DIR` reader"]
pub struct R(crate::R<DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIR` writer"]
pub struct W(crate::W<DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIR_SPEC>;
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
impl From<crate::W<DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIN0` reader - Pin 0"]
pub type PIN0_R = crate::BitReader<PIN0_A>;
#[doc = "Pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN0_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN0_A> for bool {
    #[inline(always)]
    fn from(variant: PIN0_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN0_A {
        match self.bits {
            false => PIN0_A::INPUT,
            true => PIN0_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN0_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN0_A::OUTPUT
    }
}
#[doc = "Field `PIN0` writer - Pin 0"]
pub type PIN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN0_A, O>;
impl<'a, const O: u8> PIN0_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN0_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN0_A::OUTPUT)
    }
}
#[doc = "Field `PIN1` reader - Pin 1"]
pub type PIN1_R = crate::BitReader<PIN1_A>;
#[doc = "Pin 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN1_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN1_A> for bool {
    #[inline(always)]
    fn from(variant: PIN1_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN1_A {
        match self.bits {
            false => PIN1_A::INPUT,
            true => PIN1_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN1_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN1_A::OUTPUT
    }
}
#[doc = "Field `PIN1` writer - Pin 1"]
pub type PIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN1_A, O>;
impl<'a, const O: u8> PIN1_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN1_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN1_A::OUTPUT)
    }
}
#[doc = "Field `PIN2` reader - Pin 2"]
pub type PIN2_R = crate::BitReader<PIN2_A>;
#[doc = "Pin 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN2_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN2_A> for bool {
    #[inline(always)]
    fn from(variant: PIN2_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN2_A {
        match self.bits {
            false => PIN2_A::INPUT,
            true => PIN2_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN2_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN2_A::OUTPUT
    }
}
#[doc = "Field `PIN2` writer - Pin 2"]
pub type PIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN2_A, O>;
impl<'a, const O: u8> PIN2_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN2_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN2_A::OUTPUT)
    }
}
#[doc = "Field `PIN3` reader - Pin 3"]
pub type PIN3_R = crate::BitReader<PIN3_A>;
#[doc = "Pin 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN3_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN3_A> for bool {
    #[inline(always)]
    fn from(variant: PIN3_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN3_A {
        match self.bits {
            false => PIN3_A::INPUT,
            true => PIN3_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN3_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN3_A::OUTPUT
    }
}
#[doc = "Field `PIN3` writer - Pin 3"]
pub type PIN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN3_A, O>;
impl<'a, const O: u8> PIN3_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN3_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN3_A::OUTPUT)
    }
}
#[doc = "Field `PIN4` reader - Pin 4"]
pub type PIN4_R = crate::BitReader<PIN4_A>;
#[doc = "Pin 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN4_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN4_A> for bool {
    #[inline(always)]
    fn from(variant: PIN4_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN4_A {
        match self.bits {
            false => PIN4_A::INPUT,
            true => PIN4_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN4_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN4_A::OUTPUT
    }
}
#[doc = "Field `PIN4` writer - Pin 4"]
pub type PIN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN4_A, O>;
impl<'a, const O: u8> PIN4_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN4_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN4_A::OUTPUT)
    }
}
#[doc = "Field `PIN5` reader - Pin 5"]
pub type PIN5_R = crate::BitReader<PIN5_A>;
#[doc = "Pin 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN5_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN5_A> for bool {
    #[inline(always)]
    fn from(variant: PIN5_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN5_A {
        match self.bits {
            false => PIN5_A::INPUT,
            true => PIN5_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN5_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN5_A::OUTPUT
    }
}
#[doc = "Field `PIN5` writer - Pin 5"]
pub type PIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN5_A, O>;
impl<'a, const O: u8> PIN5_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN5_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN5_A::OUTPUT)
    }
}
#[doc = "Field `PIN6` reader - Pin 6"]
pub type PIN6_R = crate::BitReader<PIN6_A>;
#[doc = "Pin 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN6_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN6_A> for bool {
    #[inline(always)]
    fn from(variant: PIN6_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN6_A {
        match self.bits {
            false => PIN6_A::INPUT,
            true => PIN6_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN6_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN6_A::OUTPUT
    }
}
#[doc = "Field `PIN6` writer - Pin 6"]
pub type PIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN6_A, O>;
impl<'a, const O: u8> PIN6_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN6_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN6_A::OUTPUT)
    }
}
#[doc = "Field `PIN7` reader - Pin 7"]
pub type PIN7_R = crate::BitReader<PIN7_A>;
#[doc = "Pin 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN7_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN7_A> for bool {
    #[inline(always)]
    fn from(variant: PIN7_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN7_A {
        match self.bits {
            false => PIN7_A::INPUT,
            true => PIN7_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN7_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN7_A::OUTPUT
    }
}
#[doc = "Field `PIN7` writer - Pin 7"]
pub type PIN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN7_A, O>;
impl<'a, const O: u8> PIN7_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN7_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN7_A::OUTPUT)
    }
}
#[doc = "Field `PIN8` reader - Pin 8"]
pub type PIN8_R = crate::BitReader<PIN8_A>;
#[doc = "Pin 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN8_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN8_A> for bool {
    #[inline(always)]
    fn from(variant: PIN8_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN8_A {
        match self.bits {
            false => PIN8_A::INPUT,
            true => PIN8_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN8_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN8_A::OUTPUT
    }
}
#[doc = "Field `PIN8` writer - Pin 8"]
pub type PIN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN8_A, O>;
impl<'a, const O: u8> PIN8_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN8_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN8_A::OUTPUT)
    }
}
#[doc = "Field `PIN9` reader - Pin 9"]
pub type PIN9_R = crate::BitReader<PIN9_A>;
#[doc = "Pin 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN9_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN9_A> for bool {
    #[inline(always)]
    fn from(variant: PIN9_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN9_A {
        match self.bits {
            false => PIN9_A::INPUT,
            true => PIN9_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN9_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN9_A::OUTPUT
    }
}
#[doc = "Field `PIN9` writer - Pin 9"]
pub type PIN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN9_A, O>;
impl<'a, const O: u8> PIN9_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN9_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN9_A::OUTPUT)
    }
}
#[doc = "Field `PIN10` reader - Pin 10"]
pub type PIN10_R = crate::BitReader<PIN10_A>;
#[doc = "Pin 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN10_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN10_A> for bool {
    #[inline(always)]
    fn from(variant: PIN10_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN10_A {
        match self.bits {
            false => PIN10_A::INPUT,
            true => PIN10_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN10_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN10_A::OUTPUT
    }
}
#[doc = "Field `PIN10` writer - Pin 10"]
pub type PIN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN10_A, O>;
impl<'a, const O: u8> PIN10_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN10_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN10_A::OUTPUT)
    }
}
#[doc = "Field `PIN11` reader - Pin 11"]
pub type PIN11_R = crate::BitReader<PIN11_A>;
#[doc = "Pin 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN11_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN11_A> for bool {
    #[inline(always)]
    fn from(variant: PIN11_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN11_A {
        match self.bits {
            false => PIN11_A::INPUT,
            true => PIN11_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN11_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN11_A::OUTPUT
    }
}
#[doc = "Field `PIN11` writer - Pin 11"]
pub type PIN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN11_A, O>;
impl<'a, const O: u8> PIN11_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN11_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN11_A::OUTPUT)
    }
}
#[doc = "Field `PIN12` reader - Pin 12"]
pub type PIN12_R = crate::BitReader<PIN12_A>;
#[doc = "Pin 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN12_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN12_A> for bool {
    #[inline(always)]
    fn from(variant: PIN12_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN12_A {
        match self.bits {
            false => PIN12_A::INPUT,
            true => PIN12_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN12_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN12_A::OUTPUT
    }
}
#[doc = "Field `PIN12` writer - Pin 12"]
pub type PIN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN12_A, O>;
impl<'a, const O: u8> PIN12_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN12_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN12_A::OUTPUT)
    }
}
#[doc = "Field `PIN13` reader - Pin 13"]
pub type PIN13_R = crate::BitReader<PIN13_A>;
#[doc = "Pin 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN13_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN13_A> for bool {
    #[inline(always)]
    fn from(variant: PIN13_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN13_A {
        match self.bits {
            false => PIN13_A::INPUT,
            true => PIN13_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN13_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN13_A::OUTPUT
    }
}
#[doc = "Field `PIN13` writer - Pin 13"]
pub type PIN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN13_A, O>;
impl<'a, const O: u8> PIN13_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN13_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN13_A::OUTPUT)
    }
}
#[doc = "Field `PIN14` reader - Pin 14"]
pub type PIN14_R = crate::BitReader<PIN14_A>;
#[doc = "Pin 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN14_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN14_A> for bool {
    #[inline(always)]
    fn from(variant: PIN14_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN14_A {
        match self.bits {
            false => PIN14_A::INPUT,
            true => PIN14_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN14_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN14_A::OUTPUT
    }
}
#[doc = "Field `PIN14` writer - Pin 14"]
pub type PIN14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN14_A, O>;
impl<'a, const O: u8> PIN14_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN14_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN14_A::OUTPUT)
    }
}
#[doc = "Field `PIN15` reader - Pin 15"]
pub type PIN15_R = crate::BitReader<PIN15_A>;
#[doc = "Pin 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN15_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN15_A> for bool {
    #[inline(always)]
    fn from(variant: PIN15_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN15_A {
        match self.bits {
            false => PIN15_A::INPUT,
            true => PIN15_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN15_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN15_A::OUTPUT
    }
}
#[doc = "Field `PIN15` writer - Pin 15"]
pub type PIN15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN15_A, O>;
impl<'a, const O: u8> PIN15_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN15_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN15_A::OUTPUT)
    }
}
#[doc = "Field `PIN16` reader - Pin 16"]
pub type PIN16_R = crate::BitReader<PIN16_A>;
#[doc = "Pin 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN16_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN16_A> for bool {
    #[inline(always)]
    fn from(variant: PIN16_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN16_A {
        match self.bits {
            false => PIN16_A::INPUT,
            true => PIN16_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN16_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN16_A::OUTPUT
    }
}
#[doc = "Field `PIN16` writer - Pin 16"]
pub type PIN16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN16_A, O>;
impl<'a, const O: u8> PIN16_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN16_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN16_A::OUTPUT)
    }
}
#[doc = "Field `PIN17` reader - Pin 17"]
pub type PIN17_R = crate::BitReader<PIN17_A>;
#[doc = "Pin 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN17_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN17_A> for bool {
    #[inline(always)]
    fn from(variant: PIN17_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN17_A {
        match self.bits {
            false => PIN17_A::INPUT,
            true => PIN17_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN17_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN17_A::OUTPUT
    }
}
#[doc = "Field `PIN17` writer - Pin 17"]
pub type PIN17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN17_A, O>;
impl<'a, const O: u8> PIN17_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN17_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN17_A::OUTPUT)
    }
}
#[doc = "Field `PIN18` reader - Pin 18"]
pub type PIN18_R = crate::BitReader<PIN18_A>;
#[doc = "Pin 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN18_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN18_A> for bool {
    #[inline(always)]
    fn from(variant: PIN18_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN18_A {
        match self.bits {
            false => PIN18_A::INPUT,
            true => PIN18_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN18_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN18_A::OUTPUT
    }
}
#[doc = "Field `PIN18` writer - Pin 18"]
pub type PIN18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN18_A, O>;
impl<'a, const O: u8> PIN18_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN18_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN18_A::OUTPUT)
    }
}
#[doc = "Field `PIN19` reader - Pin 19"]
pub type PIN19_R = crate::BitReader<PIN19_A>;
#[doc = "Pin 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN19_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN19_A> for bool {
    #[inline(always)]
    fn from(variant: PIN19_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN19_A {
        match self.bits {
            false => PIN19_A::INPUT,
            true => PIN19_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN19_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN19_A::OUTPUT
    }
}
#[doc = "Field `PIN19` writer - Pin 19"]
pub type PIN19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN19_A, O>;
impl<'a, const O: u8> PIN19_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN19_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN19_A::OUTPUT)
    }
}
#[doc = "Field `PIN20` reader - Pin 20"]
pub type PIN20_R = crate::BitReader<PIN20_A>;
#[doc = "Pin 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN20_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN20_A> for bool {
    #[inline(always)]
    fn from(variant: PIN20_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN20_A {
        match self.bits {
            false => PIN20_A::INPUT,
            true => PIN20_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN20_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN20_A::OUTPUT
    }
}
#[doc = "Field `PIN20` writer - Pin 20"]
pub type PIN20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN20_A, O>;
impl<'a, const O: u8> PIN20_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN20_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN20_A::OUTPUT)
    }
}
#[doc = "Field `PIN21` reader - Pin 21"]
pub type PIN21_R = crate::BitReader<PIN21_A>;
#[doc = "Pin 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN21_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN21_A> for bool {
    #[inline(always)]
    fn from(variant: PIN21_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN21_A {
        match self.bits {
            false => PIN21_A::INPUT,
            true => PIN21_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN21_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN21_A::OUTPUT
    }
}
#[doc = "Field `PIN21` writer - Pin 21"]
pub type PIN21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN21_A, O>;
impl<'a, const O: u8> PIN21_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN21_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN21_A::OUTPUT)
    }
}
#[doc = "Field `PIN22` reader - Pin 22"]
pub type PIN22_R = crate::BitReader<PIN22_A>;
#[doc = "Pin 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN22_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN22_A> for bool {
    #[inline(always)]
    fn from(variant: PIN22_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN22_A {
        match self.bits {
            false => PIN22_A::INPUT,
            true => PIN22_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN22_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN22_A::OUTPUT
    }
}
#[doc = "Field `PIN22` writer - Pin 22"]
pub type PIN22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN22_A, O>;
impl<'a, const O: u8> PIN22_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN22_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN22_A::OUTPUT)
    }
}
#[doc = "Field `PIN23` reader - Pin 23"]
pub type PIN23_R = crate::BitReader<PIN23_A>;
#[doc = "Pin 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN23_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN23_A> for bool {
    #[inline(always)]
    fn from(variant: PIN23_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN23_A {
        match self.bits {
            false => PIN23_A::INPUT,
            true => PIN23_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN23_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN23_A::OUTPUT
    }
}
#[doc = "Field `PIN23` writer - Pin 23"]
pub type PIN23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN23_A, O>;
impl<'a, const O: u8> PIN23_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN23_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN23_A::OUTPUT)
    }
}
#[doc = "Field `PIN24` reader - Pin 24"]
pub type PIN24_R = crate::BitReader<PIN24_A>;
#[doc = "Pin 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN24_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN24_A> for bool {
    #[inline(always)]
    fn from(variant: PIN24_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN24_A {
        match self.bits {
            false => PIN24_A::INPUT,
            true => PIN24_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN24_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN24_A::OUTPUT
    }
}
#[doc = "Field `PIN24` writer - Pin 24"]
pub type PIN24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN24_A, O>;
impl<'a, const O: u8> PIN24_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN24_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN24_A::OUTPUT)
    }
}
#[doc = "Field `PIN25` reader - Pin 25"]
pub type PIN25_R = crate::BitReader<PIN25_A>;
#[doc = "Pin 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN25_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN25_A> for bool {
    #[inline(always)]
    fn from(variant: PIN25_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN25_A {
        match self.bits {
            false => PIN25_A::INPUT,
            true => PIN25_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN25_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN25_A::OUTPUT
    }
}
#[doc = "Field `PIN25` writer - Pin 25"]
pub type PIN25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN25_A, O>;
impl<'a, const O: u8> PIN25_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN25_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN25_A::OUTPUT)
    }
}
#[doc = "Field `PIN26` reader - Pin 26"]
pub type PIN26_R = crate::BitReader<PIN26_A>;
#[doc = "Pin 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN26_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN26_A> for bool {
    #[inline(always)]
    fn from(variant: PIN26_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN26_A {
        match self.bits {
            false => PIN26_A::INPUT,
            true => PIN26_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN26_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN26_A::OUTPUT
    }
}
#[doc = "Field `PIN26` writer - Pin 26"]
pub type PIN26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN26_A, O>;
impl<'a, const O: u8> PIN26_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN26_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN26_A::OUTPUT)
    }
}
#[doc = "Field `PIN27` reader - Pin 27"]
pub type PIN27_R = crate::BitReader<PIN27_A>;
#[doc = "Pin 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN27_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN27_A> for bool {
    #[inline(always)]
    fn from(variant: PIN27_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN27_A {
        match self.bits {
            false => PIN27_A::INPUT,
            true => PIN27_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN27_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN27_A::OUTPUT
    }
}
#[doc = "Field `PIN27` writer - Pin 27"]
pub type PIN27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN27_A, O>;
impl<'a, const O: u8> PIN27_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN27_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN27_A::OUTPUT)
    }
}
#[doc = "Field `PIN28` reader - Pin 28"]
pub type PIN28_R = crate::BitReader<PIN28_A>;
#[doc = "Pin 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN28_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN28_A> for bool {
    #[inline(always)]
    fn from(variant: PIN28_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN28_A {
        match self.bits {
            false => PIN28_A::INPUT,
            true => PIN28_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN28_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN28_A::OUTPUT
    }
}
#[doc = "Field `PIN28` writer - Pin 28"]
pub type PIN28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN28_A, O>;
impl<'a, const O: u8> PIN28_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN28_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN28_A::OUTPUT)
    }
}
#[doc = "Field `PIN29` reader - Pin 29"]
pub type PIN29_R = crate::BitReader<PIN29_A>;
#[doc = "Pin 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN29_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN29_A> for bool {
    #[inline(always)]
    fn from(variant: PIN29_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN29_A {
        match self.bits {
            false => PIN29_A::INPUT,
            true => PIN29_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN29_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN29_A::OUTPUT
    }
}
#[doc = "Field `PIN29` writer - Pin 29"]
pub type PIN29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN29_A, O>;
impl<'a, const O: u8> PIN29_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN29_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN29_A::OUTPUT)
    }
}
#[doc = "Field `PIN30` reader - Pin 30"]
pub type PIN30_R = crate::BitReader<PIN30_A>;
#[doc = "Pin 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN30_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN30_A> for bool {
    #[inline(always)]
    fn from(variant: PIN30_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN30_A {
        match self.bits {
            false => PIN30_A::INPUT,
            true => PIN30_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN30_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN30_A::OUTPUT
    }
}
#[doc = "Field `PIN30` writer - Pin 30"]
pub type PIN30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN30_A, O>;
impl<'a, const O: u8> PIN30_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN30_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN30_A::OUTPUT)
    }
}
#[doc = "Field `PIN31` reader - Pin 31"]
pub type PIN31_R = crate::BitReader<PIN31_A>;
#[doc = "Pin 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIN31_A {
    #[doc = "0: Pin set as input"]
    INPUT = 0,
    #[doc = "1: Pin set as output"]
    OUTPUT = 1,
}
impl From<PIN31_A> for bool {
    #[inline(always)]
    fn from(variant: PIN31_A) -> Self {
        variant as u8 != 0
    }
}
impl PIN31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN31_A {
        match self.bits {
            false => PIN31_A::INPUT,
            true => PIN31_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PIN31_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PIN31_A::OUTPUT
    }
}
#[doc = "Field `PIN31` writer - Pin 31"]
pub type PIN31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIR_SPEC, PIN31_A, O>;
impl<'a, const O: u8> PIN31_W<'a, O> {
    #[doc = "Pin set as input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(PIN31_A::INPUT)
    }
    #[doc = "Pin set as output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PIN31_A::OUTPUT)
    }
}
impl R {
    #[doc = "Bit 0 - Pin 0"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin 1"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin 2"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin 3"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pin 4"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pin 5"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin 6"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin 7"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pin 8"]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pin 9"]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pin 10"]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pin 11"]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pin 12"]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin 13"]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pin 14"]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pin 15"]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pin 16"]
    #[inline(always)]
    pub fn pin16(&self) -> PIN16_R {
        PIN16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pin 17"]
    #[inline(always)]
    pub fn pin17(&self) -> PIN17_R {
        PIN17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pin 18"]
    #[inline(always)]
    pub fn pin18(&self) -> PIN18_R {
        PIN18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pin 19"]
    #[inline(always)]
    pub fn pin19(&self) -> PIN19_R {
        PIN19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pin 20"]
    #[inline(always)]
    pub fn pin20(&self) -> PIN20_R {
        PIN20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pin 21"]
    #[inline(always)]
    pub fn pin21(&self) -> PIN21_R {
        PIN21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pin 22"]
    #[inline(always)]
    pub fn pin22(&self) -> PIN22_R {
        PIN22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pin 23"]
    #[inline(always)]
    pub fn pin23(&self) -> PIN23_R {
        PIN23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pin 24"]
    #[inline(always)]
    pub fn pin24(&self) -> PIN24_R {
        PIN24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Pin 25"]
    #[inline(always)]
    pub fn pin25(&self) -> PIN25_R {
        PIN25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Pin 26"]
    #[inline(always)]
    pub fn pin26(&self) -> PIN26_R {
        PIN26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Pin 27"]
    #[inline(always)]
    pub fn pin27(&self) -> PIN27_R {
        PIN27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Pin 28"]
    #[inline(always)]
    pub fn pin28(&self) -> PIN28_R {
        PIN28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Pin 29"]
    #[inline(always)]
    pub fn pin29(&self) -> PIN29_R {
        PIN29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Pin 30"]
    #[inline(always)]
    pub fn pin30(&self) -> PIN30_R {
        PIN30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Pin 31"]
    #[inline(always)]
    pub fn pin31(&self) -> PIN31_R {
        PIN31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn pin0(&mut self) -> PIN0_W<0> {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1 - Pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn pin1(&mut self) -> PIN1_W<1> {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2 - Pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn pin2(&mut self) -> PIN2_W<2> {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3 - Pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn pin3(&mut self) -> PIN3_W<3> {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4 - Pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn pin4(&mut self) -> PIN4_W<4> {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5 - Pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn pin5(&mut self) -> PIN5_W<5> {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6 - Pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn pin6(&mut self) -> PIN6_W<6> {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7 - Pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn pin7(&mut self) -> PIN7_W<7> {
        PIN7_W::new(self)
    }
    #[doc = "Bit 8 - Pin 8"]
    #[inline(always)]
    #[must_use]
    pub fn pin8(&mut self) -> PIN8_W<8> {
        PIN8_W::new(self)
    }
    #[doc = "Bit 9 - Pin 9"]
    #[inline(always)]
    #[must_use]
    pub fn pin9(&mut self) -> PIN9_W<9> {
        PIN9_W::new(self)
    }
    #[doc = "Bit 10 - Pin 10"]
    #[inline(always)]
    #[must_use]
    pub fn pin10(&mut self) -> PIN10_W<10> {
        PIN10_W::new(self)
    }
    #[doc = "Bit 11 - Pin 11"]
    #[inline(always)]
    #[must_use]
    pub fn pin11(&mut self) -> PIN11_W<11> {
        PIN11_W::new(self)
    }
    #[doc = "Bit 12 - Pin 12"]
    #[inline(always)]
    #[must_use]
    pub fn pin12(&mut self) -> PIN12_W<12> {
        PIN12_W::new(self)
    }
    #[doc = "Bit 13 - Pin 13"]
    #[inline(always)]
    #[must_use]
    pub fn pin13(&mut self) -> PIN13_W<13> {
        PIN13_W::new(self)
    }
    #[doc = "Bit 14 - Pin 14"]
    #[inline(always)]
    #[must_use]
    pub fn pin14(&mut self) -> PIN14_W<14> {
        PIN14_W::new(self)
    }
    #[doc = "Bit 15 - Pin 15"]
    #[inline(always)]
    #[must_use]
    pub fn pin15(&mut self) -> PIN15_W<15> {
        PIN15_W::new(self)
    }
    #[doc = "Bit 16 - Pin 16"]
    #[inline(always)]
    #[must_use]
    pub fn pin16(&mut self) -> PIN16_W<16> {
        PIN16_W::new(self)
    }
    #[doc = "Bit 17 - Pin 17"]
    #[inline(always)]
    #[must_use]
    pub fn pin17(&mut self) -> PIN17_W<17> {
        PIN17_W::new(self)
    }
    #[doc = "Bit 18 - Pin 18"]
    #[inline(always)]
    #[must_use]
    pub fn pin18(&mut self) -> PIN18_W<18> {
        PIN18_W::new(self)
    }
    #[doc = "Bit 19 - Pin 19"]
    #[inline(always)]
    #[must_use]
    pub fn pin19(&mut self) -> PIN19_W<19> {
        PIN19_W::new(self)
    }
    #[doc = "Bit 20 - Pin 20"]
    #[inline(always)]
    #[must_use]
    pub fn pin20(&mut self) -> PIN20_W<20> {
        PIN20_W::new(self)
    }
    #[doc = "Bit 21 - Pin 21"]
    #[inline(always)]
    #[must_use]
    pub fn pin21(&mut self) -> PIN21_W<21> {
        PIN21_W::new(self)
    }
    #[doc = "Bit 22 - Pin 22"]
    #[inline(always)]
    #[must_use]
    pub fn pin22(&mut self) -> PIN22_W<22> {
        PIN22_W::new(self)
    }
    #[doc = "Bit 23 - Pin 23"]
    #[inline(always)]
    #[must_use]
    pub fn pin23(&mut self) -> PIN23_W<23> {
        PIN23_W::new(self)
    }
    #[doc = "Bit 24 - Pin 24"]
    #[inline(always)]
    #[must_use]
    pub fn pin24(&mut self) -> PIN24_W<24> {
        PIN24_W::new(self)
    }
    #[doc = "Bit 25 - Pin 25"]
    #[inline(always)]
    #[must_use]
    pub fn pin25(&mut self) -> PIN25_W<25> {
        PIN25_W::new(self)
    }
    #[doc = "Bit 26 - Pin 26"]
    #[inline(always)]
    #[must_use]
    pub fn pin26(&mut self) -> PIN26_W<26> {
        PIN26_W::new(self)
    }
    #[doc = "Bit 27 - Pin 27"]
    #[inline(always)]
    #[must_use]
    pub fn pin27(&mut self) -> PIN27_W<27> {
        PIN27_W::new(self)
    }
    #[doc = "Bit 28 - Pin 28"]
    #[inline(always)]
    #[must_use]
    pub fn pin28(&mut self) -> PIN28_W<28> {
        PIN28_W::new(self)
    }
    #[doc = "Bit 29 - Pin 29"]
    #[inline(always)]
    #[must_use]
    pub fn pin29(&mut self) -> PIN29_W<29> {
        PIN29_W::new(self)
    }
    #[doc = "Bit 30 - Pin 30"]
    #[inline(always)]
    #[must_use]
    pub fn pin30(&mut self) -> PIN30_W<30> {
        PIN30_W::new(self)
    }
    #[doc = "Bit 31 - Pin 31"]
    #[inline(always)]
    #[must_use]
    pub fn pin31(&mut self) -> PIN31_W<31> {
        PIN31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direction of GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](index.html) module"]
pub struct DIR_SPEC;
impl crate::RegisterSpec for DIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dir::R](R) reader structure"]
impl crate::Readable for DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dir::W](W) writer structure"]
impl crate::Writable for DIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
