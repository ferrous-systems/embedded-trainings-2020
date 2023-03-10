#[doc = "Register `CONFIG1` reader"]
pub struct R(crate::R<CONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG1` writer"]
pub struct W(crate::W<CONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG1_SPEC>;
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
impl From<crate::W<CONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION32` reader - Enable protection for region 32. Write '0' has no effect."]
pub type REGION32_R = crate::BitReader<REGION32_A>;
#[doc = "Enable protection for region 32. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION32_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION32_A> for bool {
    #[inline(always)]
    fn from(variant: REGION32_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION32_A {
        match self.bits {
            false => REGION32_A::DISABLED,
            true => REGION32_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION32_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION32_A::ENABLED
    }
}
#[doc = "Field `REGION32` writer - Enable protection for region 32. Write '0' has no effect."]
pub type REGION32_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION32_A, O>;
impl<'a, const O: u8> REGION32_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION32_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION32_A::ENABLED)
    }
}
#[doc = "Field `REGION33` reader - Enable protection for region 33. Write '0' has no effect."]
pub type REGION33_R = crate::BitReader<REGION33_A>;
#[doc = "Enable protection for region 33. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION33_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION33_A> for bool {
    #[inline(always)]
    fn from(variant: REGION33_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION33_A {
        match self.bits {
            false => REGION33_A::DISABLED,
            true => REGION33_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION33_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION33_A::ENABLED
    }
}
#[doc = "Field `REGION33` writer - Enable protection for region 33. Write '0' has no effect."]
pub type REGION33_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION33_A, O>;
impl<'a, const O: u8> REGION33_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION33_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION33_A::ENABLED)
    }
}
#[doc = "Field `REGION34` reader - Enable protection for region 34. Write '0' has no effect."]
pub type REGION34_R = crate::BitReader<REGION34_A>;
#[doc = "Enable protection for region 34. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION34_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION34_A> for bool {
    #[inline(always)]
    fn from(variant: REGION34_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION34_A {
        match self.bits {
            false => REGION34_A::DISABLED,
            true => REGION34_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION34_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION34_A::ENABLED
    }
}
#[doc = "Field `REGION34` writer - Enable protection for region 34. Write '0' has no effect."]
pub type REGION34_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION34_A, O>;
impl<'a, const O: u8> REGION34_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION34_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION34_A::ENABLED)
    }
}
#[doc = "Field `REGION35` reader - Enable protection for region 35. Write '0' has no effect."]
pub type REGION35_R = crate::BitReader<REGION35_A>;
#[doc = "Enable protection for region 35. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION35_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION35_A> for bool {
    #[inline(always)]
    fn from(variant: REGION35_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION35_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION35_A {
        match self.bits {
            false => REGION35_A::DISABLED,
            true => REGION35_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION35_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION35_A::ENABLED
    }
}
#[doc = "Field `REGION35` writer - Enable protection for region 35. Write '0' has no effect."]
pub type REGION35_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION35_A, O>;
impl<'a, const O: u8> REGION35_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION35_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION35_A::ENABLED)
    }
}
#[doc = "Field `REGION36` reader - Enable protection for region 36. Write '0' has no effect."]
pub type REGION36_R = crate::BitReader<REGION36_A>;
#[doc = "Enable protection for region 36. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION36_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION36_A> for bool {
    #[inline(always)]
    fn from(variant: REGION36_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION36_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION36_A {
        match self.bits {
            false => REGION36_A::DISABLED,
            true => REGION36_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION36_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION36_A::ENABLED
    }
}
#[doc = "Field `REGION36` writer - Enable protection for region 36. Write '0' has no effect."]
pub type REGION36_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION36_A, O>;
impl<'a, const O: u8> REGION36_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION36_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION36_A::ENABLED)
    }
}
#[doc = "Field `REGION37` reader - Enable protection for region 37. Write '0' has no effect."]
pub type REGION37_R = crate::BitReader<REGION37_A>;
#[doc = "Enable protection for region 37. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION37_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION37_A> for bool {
    #[inline(always)]
    fn from(variant: REGION37_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION37_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION37_A {
        match self.bits {
            false => REGION37_A::DISABLED,
            true => REGION37_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION37_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION37_A::ENABLED
    }
}
#[doc = "Field `REGION37` writer - Enable protection for region 37. Write '0' has no effect."]
pub type REGION37_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION37_A, O>;
impl<'a, const O: u8> REGION37_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION37_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION37_A::ENABLED)
    }
}
#[doc = "Field `REGION38` reader - Enable protection for region 38. Write '0' has no effect."]
pub type REGION38_R = crate::BitReader<REGION38_A>;
#[doc = "Enable protection for region 38. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION38_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION38_A> for bool {
    #[inline(always)]
    fn from(variant: REGION38_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION38_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION38_A {
        match self.bits {
            false => REGION38_A::DISABLED,
            true => REGION38_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION38_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION38_A::ENABLED
    }
}
#[doc = "Field `REGION38` writer - Enable protection for region 38. Write '0' has no effect."]
pub type REGION38_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION38_A, O>;
impl<'a, const O: u8> REGION38_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION38_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION38_A::ENABLED)
    }
}
#[doc = "Field `REGION39` reader - Enable protection for region 39. Write '0' has no effect."]
pub type REGION39_R = crate::BitReader<REGION39_A>;
#[doc = "Enable protection for region 39. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION39_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION39_A> for bool {
    #[inline(always)]
    fn from(variant: REGION39_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION39_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION39_A {
        match self.bits {
            false => REGION39_A::DISABLED,
            true => REGION39_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION39_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION39_A::ENABLED
    }
}
#[doc = "Field `REGION39` writer - Enable protection for region 39. Write '0' has no effect."]
pub type REGION39_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION39_A, O>;
impl<'a, const O: u8> REGION39_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION39_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION39_A::ENABLED)
    }
}
#[doc = "Field `REGION40` reader - Enable protection for region 40. Write '0' has no effect."]
pub type REGION40_R = crate::BitReader<REGION40_A>;
#[doc = "Enable protection for region 40. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION40_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION40_A> for bool {
    #[inline(always)]
    fn from(variant: REGION40_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION40_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION40_A {
        match self.bits {
            false => REGION40_A::DISABLED,
            true => REGION40_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION40_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION40_A::ENABLED
    }
}
#[doc = "Field `REGION40` writer - Enable protection for region 40. Write '0' has no effect."]
pub type REGION40_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION40_A, O>;
impl<'a, const O: u8> REGION40_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION40_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION40_A::ENABLED)
    }
}
#[doc = "Field `REGION41` reader - Enable protection for region 41. Write '0' has no effect."]
pub type REGION41_R = crate::BitReader<REGION41_A>;
#[doc = "Enable protection for region 41. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION41_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION41_A> for bool {
    #[inline(always)]
    fn from(variant: REGION41_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION41_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION41_A {
        match self.bits {
            false => REGION41_A::DISABLED,
            true => REGION41_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION41_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION41_A::ENABLED
    }
}
#[doc = "Field `REGION41` writer - Enable protection for region 41. Write '0' has no effect."]
pub type REGION41_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION41_A, O>;
impl<'a, const O: u8> REGION41_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION41_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION41_A::ENABLED)
    }
}
#[doc = "Field `REGION42` reader - Enable protection for region 42. Write '0' has no effect."]
pub type REGION42_R = crate::BitReader<REGION42_A>;
#[doc = "Enable protection for region 42. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION42_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION42_A> for bool {
    #[inline(always)]
    fn from(variant: REGION42_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION42_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION42_A {
        match self.bits {
            false => REGION42_A::DISABLED,
            true => REGION42_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION42_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION42_A::ENABLED
    }
}
#[doc = "Field `REGION42` writer - Enable protection for region 42. Write '0' has no effect."]
pub type REGION42_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION42_A, O>;
impl<'a, const O: u8> REGION42_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION42_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION42_A::ENABLED)
    }
}
#[doc = "Field `REGION43` reader - Enable protection for region 43. Write '0' has no effect."]
pub type REGION43_R = crate::BitReader<REGION43_A>;
#[doc = "Enable protection for region 43. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION43_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION43_A> for bool {
    #[inline(always)]
    fn from(variant: REGION43_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION43_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION43_A {
        match self.bits {
            false => REGION43_A::DISABLED,
            true => REGION43_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION43_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION43_A::ENABLED
    }
}
#[doc = "Field `REGION43` writer - Enable protection for region 43. Write '0' has no effect."]
pub type REGION43_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION43_A, O>;
impl<'a, const O: u8> REGION43_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION43_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION43_A::ENABLED)
    }
}
#[doc = "Field `REGION44` reader - Enable protection for region 44. Write '0' has no effect."]
pub type REGION44_R = crate::BitReader<REGION44_A>;
#[doc = "Enable protection for region 44. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION44_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION44_A> for bool {
    #[inline(always)]
    fn from(variant: REGION44_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION44_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION44_A {
        match self.bits {
            false => REGION44_A::DISABLED,
            true => REGION44_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION44_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION44_A::ENABLED
    }
}
#[doc = "Field `REGION44` writer - Enable protection for region 44. Write '0' has no effect."]
pub type REGION44_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION44_A, O>;
impl<'a, const O: u8> REGION44_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION44_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION44_A::ENABLED)
    }
}
#[doc = "Field `REGION45` reader - Enable protection for region 45. Write '0' has no effect."]
pub type REGION45_R = crate::BitReader<REGION45_A>;
#[doc = "Enable protection for region 45. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION45_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION45_A> for bool {
    #[inline(always)]
    fn from(variant: REGION45_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION45_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION45_A {
        match self.bits {
            false => REGION45_A::DISABLED,
            true => REGION45_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION45_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION45_A::ENABLED
    }
}
#[doc = "Field `REGION45` writer - Enable protection for region 45. Write '0' has no effect."]
pub type REGION45_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION45_A, O>;
impl<'a, const O: u8> REGION45_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION45_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION45_A::ENABLED)
    }
}
#[doc = "Field `REGION46` reader - Enable protection for region 46. Write '0' has no effect."]
pub type REGION46_R = crate::BitReader<REGION46_A>;
#[doc = "Enable protection for region 46. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION46_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION46_A> for bool {
    #[inline(always)]
    fn from(variant: REGION46_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION46_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION46_A {
        match self.bits {
            false => REGION46_A::DISABLED,
            true => REGION46_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION46_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION46_A::ENABLED
    }
}
#[doc = "Field `REGION46` writer - Enable protection for region 46. Write '0' has no effect."]
pub type REGION46_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION46_A, O>;
impl<'a, const O: u8> REGION46_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION46_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION46_A::ENABLED)
    }
}
#[doc = "Field `REGION47` reader - Enable protection for region 47. Write '0' has no effect."]
pub type REGION47_R = crate::BitReader<REGION47_A>;
#[doc = "Enable protection for region 47. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION47_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION47_A> for bool {
    #[inline(always)]
    fn from(variant: REGION47_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION47_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION47_A {
        match self.bits {
            false => REGION47_A::DISABLED,
            true => REGION47_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION47_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION47_A::ENABLED
    }
}
#[doc = "Field `REGION47` writer - Enable protection for region 47. Write '0' has no effect."]
pub type REGION47_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION47_A, O>;
impl<'a, const O: u8> REGION47_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION47_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION47_A::ENABLED)
    }
}
#[doc = "Field `REGION48` reader - Enable protection for region 48. Write '0' has no effect."]
pub type REGION48_R = crate::BitReader<REGION48_A>;
#[doc = "Enable protection for region 48. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION48_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION48_A> for bool {
    #[inline(always)]
    fn from(variant: REGION48_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION48_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION48_A {
        match self.bits {
            false => REGION48_A::DISABLED,
            true => REGION48_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION48_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION48_A::ENABLED
    }
}
#[doc = "Field `REGION48` writer - Enable protection for region 48. Write '0' has no effect."]
pub type REGION48_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION48_A, O>;
impl<'a, const O: u8> REGION48_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION48_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION48_A::ENABLED)
    }
}
#[doc = "Field `REGION49` reader - Enable protection for region 49. Write '0' has no effect."]
pub type REGION49_R = crate::BitReader<REGION49_A>;
#[doc = "Enable protection for region 49. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION49_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION49_A> for bool {
    #[inline(always)]
    fn from(variant: REGION49_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION49_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION49_A {
        match self.bits {
            false => REGION49_A::DISABLED,
            true => REGION49_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION49_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION49_A::ENABLED
    }
}
#[doc = "Field `REGION49` writer - Enable protection for region 49. Write '0' has no effect."]
pub type REGION49_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION49_A, O>;
impl<'a, const O: u8> REGION49_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION49_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION49_A::ENABLED)
    }
}
#[doc = "Field `REGION50` reader - Enable protection for region 50. Write '0' has no effect."]
pub type REGION50_R = crate::BitReader<REGION50_A>;
#[doc = "Enable protection for region 50. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION50_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION50_A> for bool {
    #[inline(always)]
    fn from(variant: REGION50_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION50_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION50_A {
        match self.bits {
            false => REGION50_A::DISABLED,
            true => REGION50_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION50_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION50_A::ENABLED
    }
}
#[doc = "Field `REGION50` writer - Enable protection for region 50. Write '0' has no effect."]
pub type REGION50_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION50_A, O>;
impl<'a, const O: u8> REGION50_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION50_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION50_A::ENABLED)
    }
}
#[doc = "Field `REGION51` reader - Enable protection for region 51. Write '0' has no effect."]
pub type REGION51_R = crate::BitReader<REGION51_A>;
#[doc = "Enable protection for region 51. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION51_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION51_A> for bool {
    #[inline(always)]
    fn from(variant: REGION51_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION51_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION51_A {
        match self.bits {
            false => REGION51_A::DISABLED,
            true => REGION51_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION51_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION51_A::ENABLED
    }
}
#[doc = "Field `REGION51` writer - Enable protection for region 51. Write '0' has no effect."]
pub type REGION51_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION51_A, O>;
impl<'a, const O: u8> REGION51_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION51_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION51_A::ENABLED)
    }
}
#[doc = "Field `REGION52` reader - Enable protection for region 52. Write '0' has no effect."]
pub type REGION52_R = crate::BitReader<REGION52_A>;
#[doc = "Enable protection for region 52. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION52_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION52_A> for bool {
    #[inline(always)]
    fn from(variant: REGION52_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION52_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION52_A {
        match self.bits {
            false => REGION52_A::DISABLED,
            true => REGION52_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION52_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION52_A::ENABLED
    }
}
#[doc = "Field `REGION52` writer - Enable protection for region 52. Write '0' has no effect."]
pub type REGION52_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION52_A, O>;
impl<'a, const O: u8> REGION52_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION52_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION52_A::ENABLED)
    }
}
#[doc = "Field `REGION53` reader - Enable protection for region 53. Write '0' has no effect."]
pub type REGION53_R = crate::BitReader<REGION53_A>;
#[doc = "Enable protection for region 53. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION53_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION53_A> for bool {
    #[inline(always)]
    fn from(variant: REGION53_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION53_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION53_A {
        match self.bits {
            false => REGION53_A::DISABLED,
            true => REGION53_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION53_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION53_A::ENABLED
    }
}
#[doc = "Field `REGION53` writer - Enable protection for region 53. Write '0' has no effect."]
pub type REGION53_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION53_A, O>;
impl<'a, const O: u8> REGION53_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION53_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION53_A::ENABLED)
    }
}
#[doc = "Field `REGION54` reader - Enable protection for region 54. Write '0' has no effect."]
pub type REGION54_R = crate::BitReader<REGION54_A>;
#[doc = "Enable protection for region 54. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION54_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION54_A> for bool {
    #[inline(always)]
    fn from(variant: REGION54_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION54_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION54_A {
        match self.bits {
            false => REGION54_A::DISABLED,
            true => REGION54_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION54_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION54_A::ENABLED
    }
}
#[doc = "Field `REGION54` writer - Enable protection for region 54. Write '0' has no effect."]
pub type REGION54_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION54_A, O>;
impl<'a, const O: u8> REGION54_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION54_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION54_A::ENABLED)
    }
}
#[doc = "Field `REGION55` reader - Enable protection for region 55. Write '0' has no effect."]
pub type REGION55_R = crate::BitReader<REGION55_A>;
#[doc = "Enable protection for region 55. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION55_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION55_A> for bool {
    #[inline(always)]
    fn from(variant: REGION55_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION55_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION55_A {
        match self.bits {
            false => REGION55_A::DISABLED,
            true => REGION55_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION55_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION55_A::ENABLED
    }
}
#[doc = "Field `REGION55` writer - Enable protection for region 55. Write '0' has no effect."]
pub type REGION55_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION55_A, O>;
impl<'a, const O: u8> REGION55_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION55_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION55_A::ENABLED)
    }
}
#[doc = "Field `REGION56` reader - Enable protection for region 56. Write '0' has no effect."]
pub type REGION56_R = crate::BitReader<REGION56_A>;
#[doc = "Enable protection for region 56. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION56_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION56_A> for bool {
    #[inline(always)]
    fn from(variant: REGION56_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION56_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION56_A {
        match self.bits {
            false => REGION56_A::DISABLED,
            true => REGION56_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION56_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION56_A::ENABLED
    }
}
#[doc = "Field `REGION56` writer - Enable protection for region 56. Write '0' has no effect."]
pub type REGION56_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION56_A, O>;
impl<'a, const O: u8> REGION56_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION56_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION56_A::ENABLED)
    }
}
#[doc = "Field `REGION57` reader - Enable protection for region 57. Write '0' has no effect."]
pub type REGION57_R = crate::BitReader<REGION57_A>;
#[doc = "Enable protection for region 57. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION57_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION57_A> for bool {
    #[inline(always)]
    fn from(variant: REGION57_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION57_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION57_A {
        match self.bits {
            false => REGION57_A::DISABLED,
            true => REGION57_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION57_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION57_A::ENABLED
    }
}
#[doc = "Field `REGION57` writer - Enable protection for region 57. Write '0' has no effect."]
pub type REGION57_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION57_A, O>;
impl<'a, const O: u8> REGION57_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION57_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION57_A::ENABLED)
    }
}
#[doc = "Field `REGION58` reader - Enable protection for region 58. Write '0' has no effect."]
pub type REGION58_R = crate::BitReader<REGION58_A>;
#[doc = "Enable protection for region 58. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION58_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION58_A> for bool {
    #[inline(always)]
    fn from(variant: REGION58_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION58_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION58_A {
        match self.bits {
            false => REGION58_A::DISABLED,
            true => REGION58_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION58_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION58_A::ENABLED
    }
}
#[doc = "Field `REGION58` writer - Enable protection for region 58. Write '0' has no effect."]
pub type REGION58_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION58_A, O>;
impl<'a, const O: u8> REGION58_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION58_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION58_A::ENABLED)
    }
}
#[doc = "Field `REGION59` reader - Enable protection for region 59. Write '0' has no effect."]
pub type REGION59_R = crate::BitReader<REGION59_A>;
#[doc = "Enable protection for region 59. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION59_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION59_A> for bool {
    #[inline(always)]
    fn from(variant: REGION59_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION59_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION59_A {
        match self.bits {
            false => REGION59_A::DISABLED,
            true => REGION59_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION59_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION59_A::ENABLED
    }
}
#[doc = "Field `REGION59` writer - Enable protection for region 59. Write '0' has no effect."]
pub type REGION59_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION59_A, O>;
impl<'a, const O: u8> REGION59_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION59_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION59_A::ENABLED)
    }
}
#[doc = "Field `REGION60` reader - Enable protection for region 60. Write '0' has no effect."]
pub type REGION60_R = crate::BitReader<REGION60_A>;
#[doc = "Enable protection for region 60. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION60_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION60_A> for bool {
    #[inline(always)]
    fn from(variant: REGION60_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION60_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION60_A {
        match self.bits {
            false => REGION60_A::DISABLED,
            true => REGION60_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION60_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION60_A::ENABLED
    }
}
#[doc = "Field `REGION60` writer - Enable protection for region 60. Write '0' has no effect."]
pub type REGION60_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION60_A, O>;
impl<'a, const O: u8> REGION60_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION60_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION60_A::ENABLED)
    }
}
#[doc = "Field `REGION61` reader - Enable protection for region 61. Write '0' has no effect."]
pub type REGION61_R = crate::BitReader<REGION61_A>;
#[doc = "Enable protection for region 61. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION61_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION61_A> for bool {
    #[inline(always)]
    fn from(variant: REGION61_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION61_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION61_A {
        match self.bits {
            false => REGION61_A::DISABLED,
            true => REGION61_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION61_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION61_A::ENABLED
    }
}
#[doc = "Field `REGION61` writer - Enable protection for region 61. Write '0' has no effect."]
pub type REGION61_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION61_A, O>;
impl<'a, const O: u8> REGION61_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION61_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION61_A::ENABLED)
    }
}
#[doc = "Field `REGION62` reader - Enable protection for region 62. Write '0' has no effect."]
pub type REGION62_R = crate::BitReader<REGION62_A>;
#[doc = "Enable protection for region 62. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION62_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION62_A> for bool {
    #[inline(always)]
    fn from(variant: REGION62_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION62_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION62_A {
        match self.bits {
            false => REGION62_A::DISABLED,
            true => REGION62_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION62_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION62_A::ENABLED
    }
}
#[doc = "Field `REGION62` writer - Enable protection for region 62. Write '0' has no effect."]
pub type REGION62_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION62_A, O>;
impl<'a, const O: u8> REGION62_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION62_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION62_A::ENABLED)
    }
}
#[doc = "Field `REGION63` reader - Enable protection for region 63. Write '0' has no effect."]
pub type REGION63_R = crate::BitReader<REGION63_A>;
#[doc = "Enable protection for region 63. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION63_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION63_A> for bool {
    #[inline(always)]
    fn from(variant: REGION63_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION63_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION63_A {
        match self.bits {
            false => REGION63_A::DISABLED,
            true => REGION63_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION63_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION63_A::ENABLED
    }
}
#[doc = "Field `REGION63` writer - Enable protection for region 63. Write '0' has no effect."]
pub type REGION63_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG1_SPEC, REGION63_A, O>;
impl<'a, const O: u8> REGION63_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION63_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION63_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline(always)]
    pub fn region32(&self) -> REGION32_R {
        REGION32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline(always)]
    pub fn region33(&self) -> REGION33_R {
        REGION33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline(always)]
    pub fn region34(&self) -> REGION34_R {
        REGION34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline(always)]
    pub fn region35(&self) -> REGION35_R {
        REGION35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline(always)]
    pub fn region36(&self) -> REGION36_R {
        REGION36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline(always)]
    pub fn region37(&self) -> REGION37_R {
        REGION37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline(always)]
    pub fn region38(&self) -> REGION38_R {
        REGION38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline(always)]
    pub fn region39(&self) -> REGION39_R {
        REGION39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline(always)]
    pub fn region40(&self) -> REGION40_R {
        REGION40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline(always)]
    pub fn region41(&self) -> REGION41_R {
        REGION41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline(always)]
    pub fn region42(&self) -> REGION42_R {
        REGION42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline(always)]
    pub fn region43(&self) -> REGION43_R {
        REGION43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline(always)]
    pub fn region44(&self) -> REGION44_R {
        REGION44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline(always)]
    pub fn region45(&self) -> REGION45_R {
        REGION45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline(always)]
    pub fn region46(&self) -> REGION46_R {
        REGION46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline(always)]
    pub fn region47(&self) -> REGION47_R {
        REGION47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline(always)]
    pub fn region48(&self) -> REGION48_R {
        REGION48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline(always)]
    pub fn region49(&self) -> REGION49_R {
        REGION49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline(always)]
    pub fn region50(&self) -> REGION50_R {
        REGION50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline(always)]
    pub fn region51(&self) -> REGION51_R {
        REGION51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline(always)]
    pub fn region52(&self) -> REGION52_R {
        REGION52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline(always)]
    pub fn region53(&self) -> REGION53_R {
        REGION53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline(always)]
    pub fn region54(&self) -> REGION54_R {
        REGION54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline(always)]
    pub fn region55(&self) -> REGION55_R {
        REGION55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline(always)]
    pub fn region56(&self) -> REGION56_R {
        REGION56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline(always)]
    pub fn region57(&self) -> REGION57_R {
        REGION57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline(always)]
    pub fn region58(&self) -> REGION58_R {
        REGION58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline(always)]
    pub fn region59(&self) -> REGION59_R {
        REGION59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline(always)]
    pub fn region60(&self) -> REGION60_R {
        REGION60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline(always)]
    pub fn region61(&self) -> REGION61_R {
        REGION61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline(always)]
    pub fn region62(&self) -> REGION62_R {
        REGION62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline(always)]
    pub fn region63(&self) -> REGION63_R {
        REGION63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region32(&mut self) -> REGION32_W<0> {
        REGION32_W::new(self)
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region33(&mut self) -> REGION33_W<1> {
        REGION33_W::new(self)
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region34(&mut self) -> REGION34_W<2> {
        REGION34_W::new(self)
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region35(&mut self) -> REGION35_W<3> {
        REGION35_W::new(self)
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region36(&mut self) -> REGION36_W<4> {
        REGION36_W::new(self)
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region37(&mut self) -> REGION37_W<5> {
        REGION37_W::new(self)
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region38(&mut self) -> REGION38_W<6> {
        REGION38_W::new(self)
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region39(&mut self) -> REGION39_W<7> {
        REGION39_W::new(self)
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region40(&mut self) -> REGION40_W<8> {
        REGION40_W::new(self)
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region41(&mut self) -> REGION41_W<9> {
        REGION41_W::new(self)
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region42(&mut self) -> REGION42_W<10> {
        REGION42_W::new(self)
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region43(&mut self) -> REGION43_W<11> {
        REGION43_W::new(self)
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region44(&mut self) -> REGION44_W<12> {
        REGION44_W::new(self)
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region45(&mut self) -> REGION45_W<13> {
        REGION45_W::new(self)
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region46(&mut self) -> REGION46_W<14> {
        REGION46_W::new(self)
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region47(&mut self) -> REGION47_W<15> {
        REGION47_W::new(self)
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region48(&mut self) -> REGION48_W<16> {
        REGION48_W::new(self)
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region49(&mut self) -> REGION49_W<17> {
        REGION49_W::new(self)
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region50(&mut self) -> REGION50_W<18> {
        REGION50_W::new(self)
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region51(&mut self) -> REGION51_W<19> {
        REGION51_W::new(self)
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region52(&mut self) -> REGION52_W<20> {
        REGION52_W::new(self)
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region53(&mut self) -> REGION53_W<21> {
        REGION53_W::new(self)
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region54(&mut self) -> REGION54_W<22> {
        REGION54_W::new(self)
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region55(&mut self) -> REGION55_W<23> {
        REGION55_W::new(self)
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region56(&mut self) -> REGION56_W<24> {
        REGION56_W::new(self)
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region57(&mut self) -> REGION57_W<25> {
        REGION57_W::new(self)
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region58(&mut self) -> REGION58_W<26> {
        REGION58_W::new(self)
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region59(&mut self) -> REGION59_W<27> {
        REGION59_W::new(self)
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region60(&mut self) -> REGION60_W<28> {
        REGION60_W::new(self)
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region61(&mut self) -> REGION61_W<29> {
        REGION61_W::new(self)
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region62(&mut self) -> REGION62_W<30> {
        REGION62_W::new(self)
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region63(&mut self) -> REGION63_W<31> {
        REGION63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block protect configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config1](index.html) module"]
pub struct CONFIG1_SPEC;
impl crate::RegisterSpec for CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config1::R](R) reader structure"]
impl crate::Readable for CONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config1::W](W) writer structure"]
impl crate::Writable for CONFIG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG1 to value 0"]
impl crate::Resettable for CONFIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
