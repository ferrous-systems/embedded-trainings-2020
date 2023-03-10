#[doc = "Register `CONFIG2` reader"]
pub struct R(crate::R<CONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG2` writer"]
pub struct W(crate::W<CONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG2_SPEC>;
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
impl From<crate::W<CONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION64` reader - Enable protection for region 64. Write '0' has no effect."]
pub type REGION64_R = crate::BitReader<REGION64_A>;
#[doc = "Enable protection for region 64. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION64_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION64_A> for bool {
    #[inline(always)]
    fn from(variant: REGION64_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION64_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION64_A {
        match self.bits {
            false => REGION64_A::DISABLED,
            true => REGION64_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION64_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION64_A::ENABLED
    }
}
#[doc = "Field `REGION64` writer - Enable protection for region 64. Write '0' has no effect."]
pub type REGION64_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION64_A, O>;
impl<'a, const O: u8> REGION64_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION64_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION64_A::ENABLED)
    }
}
#[doc = "Field `REGION65` reader - Enable protection for region 65. Write '0' has no effect."]
pub type REGION65_R = crate::BitReader<REGION65_A>;
#[doc = "Enable protection for region 65. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION65_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION65_A> for bool {
    #[inline(always)]
    fn from(variant: REGION65_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION65_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION65_A {
        match self.bits {
            false => REGION65_A::DISABLED,
            true => REGION65_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION65_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION65_A::ENABLED
    }
}
#[doc = "Field `REGION65` writer - Enable protection for region 65. Write '0' has no effect."]
pub type REGION65_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION65_A, O>;
impl<'a, const O: u8> REGION65_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION65_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION65_A::ENABLED)
    }
}
#[doc = "Field `REGION66` reader - Enable protection for region 66. Write '0' has no effect."]
pub type REGION66_R = crate::BitReader<REGION66_A>;
#[doc = "Enable protection for region 66. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION66_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION66_A> for bool {
    #[inline(always)]
    fn from(variant: REGION66_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION66_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION66_A {
        match self.bits {
            false => REGION66_A::DISABLED,
            true => REGION66_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION66_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION66_A::ENABLED
    }
}
#[doc = "Field `REGION66` writer - Enable protection for region 66. Write '0' has no effect."]
pub type REGION66_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION66_A, O>;
impl<'a, const O: u8> REGION66_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION66_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION66_A::ENABLED)
    }
}
#[doc = "Field `REGION67` reader - Enable protection for region 67. Write '0' has no effect."]
pub type REGION67_R = crate::BitReader<REGION67_A>;
#[doc = "Enable protection for region 67. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION67_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION67_A> for bool {
    #[inline(always)]
    fn from(variant: REGION67_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION67_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION67_A {
        match self.bits {
            false => REGION67_A::DISABLED,
            true => REGION67_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION67_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION67_A::ENABLED
    }
}
#[doc = "Field `REGION67` writer - Enable protection for region 67. Write '0' has no effect."]
pub type REGION67_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION67_A, O>;
impl<'a, const O: u8> REGION67_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION67_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION67_A::ENABLED)
    }
}
#[doc = "Field `REGION68` reader - Enable protection for region 68. Write '0' has no effect."]
pub type REGION68_R = crate::BitReader<REGION68_A>;
#[doc = "Enable protection for region 68. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION68_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION68_A> for bool {
    #[inline(always)]
    fn from(variant: REGION68_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION68_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION68_A {
        match self.bits {
            false => REGION68_A::DISABLED,
            true => REGION68_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION68_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION68_A::ENABLED
    }
}
#[doc = "Field `REGION68` writer - Enable protection for region 68. Write '0' has no effect."]
pub type REGION68_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION68_A, O>;
impl<'a, const O: u8> REGION68_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION68_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION68_A::ENABLED)
    }
}
#[doc = "Field `REGION69` reader - Enable protection for region 69. Write '0' has no effect."]
pub type REGION69_R = crate::BitReader<REGION69_A>;
#[doc = "Enable protection for region 69. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION69_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION69_A> for bool {
    #[inline(always)]
    fn from(variant: REGION69_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION69_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION69_A {
        match self.bits {
            false => REGION69_A::DISABLED,
            true => REGION69_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION69_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION69_A::ENABLED
    }
}
#[doc = "Field `REGION69` writer - Enable protection for region 69. Write '0' has no effect."]
pub type REGION69_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION69_A, O>;
impl<'a, const O: u8> REGION69_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION69_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION69_A::ENABLED)
    }
}
#[doc = "Field `REGION70` reader - Enable protection for region 70. Write '0' has no effect."]
pub type REGION70_R = crate::BitReader<REGION70_A>;
#[doc = "Enable protection for region 70. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION70_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION70_A> for bool {
    #[inline(always)]
    fn from(variant: REGION70_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION70_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION70_A {
        match self.bits {
            false => REGION70_A::DISABLED,
            true => REGION70_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION70_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION70_A::ENABLED
    }
}
#[doc = "Field `REGION70` writer - Enable protection for region 70. Write '0' has no effect."]
pub type REGION70_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION70_A, O>;
impl<'a, const O: u8> REGION70_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION70_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION70_A::ENABLED)
    }
}
#[doc = "Field `REGION71` reader - Enable protection for region 71. Write '0' has no effect."]
pub type REGION71_R = crate::BitReader<REGION71_A>;
#[doc = "Enable protection for region 71. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION71_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION71_A> for bool {
    #[inline(always)]
    fn from(variant: REGION71_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION71_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION71_A {
        match self.bits {
            false => REGION71_A::DISABLED,
            true => REGION71_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION71_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION71_A::ENABLED
    }
}
#[doc = "Field `REGION71` writer - Enable protection for region 71. Write '0' has no effect."]
pub type REGION71_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION71_A, O>;
impl<'a, const O: u8> REGION71_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION71_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION71_A::ENABLED)
    }
}
#[doc = "Field `REGION72` reader - Enable protection for region 72. Write '0' has no effect."]
pub type REGION72_R = crate::BitReader<REGION72_A>;
#[doc = "Enable protection for region 72. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION72_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION72_A> for bool {
    #[inline(always)]
    fn from(variant: REGION72_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION72_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION72_A {
        match self.bits {
            false => REGION72_A::DISABLED,
            true => REGION72_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION72_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION72_A::ENABLED
    }
}
#[doc = "Field `REGION72` writer - Enable protection for region 72. Write '0' has no effect."]
pub type REGION72_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION72_A, O>;
impl<'a, const O: u8> REGION72_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION72_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION72_A::ENABLED)
    }
}
#[doc = "Field `REGION73` reader - Enable protection for region 73. Write '0' has no effect."]
pub type REGION73_R = crate::BitReader<REGION73_A>;
#[doc = "Enable protection for region 73. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION73_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION73_A> for bool {
    #[inline(always)]
    fn from(variant: REGION73_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION73_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION73_A {
        match self.bits {
            false => REGION73_A::DISABLED,
            true => REGION73_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION73_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION73_A::ENABLED
    }
}
#[doc = "Field `REGION73` writer - Enable protection for region 73. Write '0' has no effect."]
pub type REGION73_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION73_A, O>;
impl<'a, const O: u8> REGION73_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION73_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION73_A::ENABLED)
    }
}
#[doc = "Field `REGION74` reader - Enable protection for region 74. Write '0' has no effect."]
pub type REGION74_R = crate::BitReader<REGION74_A>;
#[doc = "Enable protection for region 74. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION74_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION74_A> for bool {
    #[inline(always)]
    fn from(variant: REGION74_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION74_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION74_A {
        match self.bits {
            false => REGION74_A::DISABLED,
            true => REGION74_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION74_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION74_A::ENABLED
    }
}
#[doc = "Field `REGION74` writer - Enable protection for region 74. Write '0' has no effect."]
pub type REGION74_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION74_A, O>;
impl<'a, const O: u8> REGION74_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION74_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION74_A::ENABLED)
    }
}
#[doc = "Field `REGION75` reader - Enable protection for region 75. Write '0' has no effect."]
pub type REGION75_R = crate::BitReader<REGION75_A>;
#[doc = "Enable protection for region 75. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION75_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION75_A> for bool {
    #[inline(always)]
    fn from(variant: REGION75_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION75_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION75_A {
        match self.bits {
            false => REGION75_A::DISABLED,
            true => REGION75_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION75_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION75_A::ENABLED
    }
}
#[doc = "Field `REGION75` writer - Enable protection for region 75. Write '0' has no effect."]
pub type REGION75_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION75_A, O>;
impl<'a, const O: u8> REGION75_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION75_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION75_A::ENABLED)
    }
}
#[doc = "Field `REGION76` reader - Enable protection for region 76. Write '0' has no effect."]
pub type REGION76_R = crate::BitReader<REGION76_A>;
#[doc = "Enable protection for region 76. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION76_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION76_A> for bool {
    #[inline(always)]
    fn from(variant: REGION76_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION76_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION76_A {
        match self.bits {
            false => REGION76_A::DISABLED,
            true => REGION76_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION76_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION76_A::ENABLED
    }
}
#[doc = "Field `REGION76` writer - Enable protection for region 76. Write '0' has no effect."]
pub type REGION76_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION76_A, O>;
impl<'a, const O: u8> REGION76_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION76_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION76_A::ENABLED)
    }
}
#[doc = "Field `REGION77` reader - Enable protection for region 77. Write '0' has no effect."]
pub type REGION77_R = crate::BitReader<REGION77_A>;
#[doc = "Enable protection for region 77. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION77_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION77_A> for bool {
    #[inline(always)]
    fn from(variant: REGION77_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION77_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION77_A {
        match self.bits {
            false => REGION77_A::DISABLED,
            true => REGION77_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION77_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION77_A::ENABLED
    }
}
#[doc = "Field `REGION77` writer - Enable protection for region 77. Write '0' has no effect."]
pub type REGION77_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION77_A, O>;
impl<'a, const O: u8> REGION77_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION77_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION77_A::ENABLED)
    }
}
#[doc = "Field `REGION78` reader - Enable protection for region 78. Write '0' has no effect."]
pub type REGION78_R = crate::BitReader<REGION78_A>;
#[doc = "Enable protection for region 78. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION78_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION78_A> for bool {
    #[inline(always)]
    fn from(variant: REGION78_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION78_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION78_A {
        match self.bits {
            false => REGION78_A::DISABLED,
            true => REGION78_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION78_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION78_A::ENABLED
    }
}
#[doc = "Field `REGION78` writer - Enable protection for region 78. Write '0' has no effect."]
pub type REGION78_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION78_A, O>;
impl<'a, const O: u8> REGION78_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION78_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION78_A::ENABLED)
    }
}
#[doc = "Field `REGION79` reader - Enable protection for region 79. Write '0' has no effect."]
pub type REGION79_R = crate::BitReader<REGION79_A>;
#[doc = "Enable protection for region 79. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION79_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION79_A> for bool {
    #[inline(always)]
    fn from(variant: REGION79_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION79_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION79_A {
        match self.bits {
            false => REGION79_A::DISABLED,
            true => REGION79_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION79_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION79_A::ENABLED
    }
}
#[doc = "Field `REGION79` writer - Enable protection for region 79. Write '0' has no effect."]
pub type REGION79_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION79_A, O>;
impl<'a, const O: u8> REGION79_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION79_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION79_A::ENABLED)
    }
}
#[doc = "Field `REGION80` reader - Enable protection for region 80. Write '0' has no effect."]
pub type REGION80_R = crate::BitReader<REGION80_A>;
#[doc = "Enable protection for region 80. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION80_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION80_A> for bool {
    #[inline(always)]
    fn from(variant: REGION80_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION80_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION80_A {
        match self.bits {
            false => REGION80_A::DISABLED,
            true => REGION80_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION80_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION80_A::ENABLED
    }
}
#[doc = "Field `REGION80` writer - Enable protection for region 80. Write '0' has no effect."]
pub type REGION80_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION80_A, O>;
impl<'a, const O: u8> REGION80_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION80_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION80_A::ENABLED)
    }
}
#[doc = "Field `REGION81` reader - Enable protection for region 81. Write '0' has no effect."]
pub type REGION81_R = crate::BitReader<REGION81_A>;
#[doc = "Enable protection for region 81. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION81_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION81_A> for bool {
    #[inline(always)]
    fn from(variant: REGION81_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION81_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION81_A {
        match self.bits {
            false => REGION81_A::DISABLED,
            true => REGION81_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION81_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION81_A::ENABLED
    }
}
#[doc = "Field `REGION81` writer - Enable protection for region 81. Write '0' has no effect."]
pub type REGION81_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION81_A, O>;
impl<'a, const O: u8> REGION81_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION81_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION81_A::ENABLED)
    }
}
#[doc = "Field `REGION82` reader - Enable protection for region 82. Write '0' has no effect."]
pub type REGION82_R = crate::BitReader<REGION82_A>;
#[doc = "Enable protection for region 82. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION82_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION82_A> for bool {
    #[inline(always)]
    fn from(variant: REGION82_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION82_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION82_A {
        match self.bits {
            false => REGION82_A::DISABLED,
            true => REGION82_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION82_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION82_A::ENABLED
    }
}
#[doc = "Field `REGION82` writer - Enable protection for region 82. Write '0' has no effect."]
pub type REGION82_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION82_A, O>;
impl<'a, const O: u8> REGION82_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION82_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION82_A::ENABLED)
    }
}
#[doc = "Field `REGION83` reader - Enable protection for region 83. Write '0' has no effect."]
pub type REGION83_R = crate::BitReader<REGION83_A>;
#[doc = "Enable protection for region 83. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION83_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION83_A> for bool {
    #[inline(always)]
    fn from(variant: REGION83_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION83_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION83_A {
        match self.bits {
            false => REGION83_A::DISABLED,
            true => REGION83_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION83_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION83_A::ENABLED
    }
}
#[doc = "Field `REGION83` writer - Enable protection for region 83. Write '0' has no effect."]
pub type REGION83_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION83_A, O>;
impl<'a, const O: u8> REGION83_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION83_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION83_A::ENABLED)
    }
}
#[doc = "Field `REGION84` reader - Enable protection for region 84. Write '0' has no effect."]
pub type REGION84_R = crate::BitReader<REGION84_A>;
#[doc = "Enable protection for region 84. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION84_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION84_A> for bool {
    #[inline(always)]
    fn from(variant: REGION84_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION84_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION84_A {
        match self.bits {
            false => REGION84_A::DISABLED,
            true => REGION84_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION84_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION84_A::ENABLED
    }
}
#[doc = "Field `REGION84` writer - Enable protection for region 84. Write '0' has no effect."]
pub type REGION84_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION84_A, O>;
impl<'a, const O: u8> REGION84_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION84_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION84_A::ENABLED)
    }
}
#[doc = "Field `REGION85` reader - Enable protection for region 85. Write '0' has no effect."]
pub type REGION85_R = crate::BitReader<REGION85_A>;
#[doc = "Enable protection for region 85. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION85_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION85_A> for bool {
    #[inline(always)]
    fn from(variant: REGION85_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION85_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION85_A {
        match self.bits {
            false => REGION85_A::DISABLED,
            true => REGION85_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION85_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION85_A::ENABLED
    }
}
#[doc = "Field `REGION85` writer - Enable protection for region 85. Write '0' has no effect."]
pub type REGION85_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION85_A, O>;
impl<'a, const O: u8> REGION85_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION85_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION85_A::ENABLED)
    }
}
#[doc = "Field `REGION86` reader - Enable protection for region 86. Write '0' has no effect."]
pub type REGION86_R = crate::BitReader<REGION86_A>;
#[doc = "Enable protection for region 86. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION86_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION86_A> for bool {
    #[inline(always)]
    fn from(variant: REGION86_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION86_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION86_A {
        match self.bits {
            false => REGION86_A::DISABLED,
            true => REGION86_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION86_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION86_A::ENABLED
    }
}
#[doc = "Field `REGION86` writer - Enable protection for region 86. Write '0' has no effect."]
pub type REGION86_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION86_A, O>;
impl<'a, const O: u8> REGION86_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION86_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION86_A::ENABLED)
    }
}
#[doc = "Field `REGION87` reader - Enable protection for region 87. Write '0' has no effect."]
pub type REGION87_R = crate::BitReader<REGION87_A>;
#[doc = "Enable protection for region 87. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION87_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION87_A> for bool {
    #[inline(always)]
    fn from(variant: REGION87_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION87_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION87_A {
        match self.bits {
            false => REGION87_A::DISABLED,
            true => REGION87_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION87_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION87_A::ENABLED
    }
}
#[doc = "Field `REGION87` writer - Enable protection for region 87. Write '0' has no effect."]
pub type REGION87_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION87_A, O>;
impl<'a, const O: u8> REGION87_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION87_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION87_A::ENABLED)
    }
}
#[doc = "Field `REGION88` reader - Enable protection for region 88. Write '0' has no effect."]
pub type REGION88_R = crate::BitReader<REGION88_A>;
#[doc = "Enable protection for region 88. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION88_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION88_A> for bool {
    #[inline(always)]
    fn from(variant: REGION88_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION88_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION88_A {
        match self.bits {
            false => REGION88_A::DISABLED,
            true => REGION88_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION88_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION88_A::ENABLED
    }
}
#[doc = "Field `REGION88` writer - Enable protection for region 88. Write '0' has no effect."]
pub type REGION88_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION88_A, O>;
impl<'a, const O: u8> REGION88_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION88_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION88_A::ENABLED)
    }
}
#[doc = "Field `REGION89` reader - Enable protection for region 89. Write '0' has no effect."]
pub type REGION89_R = crate::BitReader<REGION89_A>;
#[doc = "Enable protection for region 89. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION89_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION89_A> for bool {
    #[inline(always)]
    fn from(variant: REGION89_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION89_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION89_A {
        match self.bits {
            false => REGION89_A::DISABLED,
            true => REGION89_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION89_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION89_A::ENABLED
    }
}
#[doc = "Field `REGION89` writer - Enable protection for region 89. Write '0' has no effect."]
pub type REGION89_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION89_A, O>;
impl<'a, const O: u8> REGION89_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION89_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION89_A::ENABLED)
    }
}
#[doc = "Field `REGION90` reader - Enable protection for region 90. Write '0' has no effect."]
pub type REGION90_R = crate::BitReader<REGION90_A>;
#[doc = "Enable protection for region 90. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION90_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION90_A> for bool {
    #[inline(always)]
    fn from(variant: REGION90_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION90_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION90_A {
        match self.bits {
            false => REGION90_A::DISABLED,
            true => REGION90_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION90_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION90_A::ENABLED
    }
}
#[doc = "Field `REGION90` writer - Enable protection for region 90. Write '0' has no effect."]
pub type REGION90_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION90_A, O>;
impl<'a, const O: u8> REGION90_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION90_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION90_A::ENABLED)
    }
}
#[doc = "Field `REGION91` reader - Enable protection for region 91. Write '0' has no effect."]
pub type REGION91_R = crate::BitReader<REGION91_A>;
#[doc = "Enable protection for region 91. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION91_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION91_A> for bool {
    #[inline(always)]
    fn from(variant: REGION91_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION91_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION91_A {
        match self.bits {
            false => REGION91_A::DISABLED,
            true => REGION91_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION91_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION91_A::ENABLED
    }
}
#[doc = "Field `REGION91` writer - Enable protection for region 91. Write '0' has no effect."]
pub type REGION91_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION91_A, O>;
impl<'a, const O: u8> REGION91_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION91_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION91_A::ENABLED)
    }
}
#[doc = "Field `REGION92` reader - Enable protection for region 92. Write '0' has no effect."]
pub type REGION92_R = crate::BitReader<REGION92_A>;
#[doc = "Enable protection for region 92. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION92_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION92_A> for bool {
    #[inline(always)]
    fn from(variant: REGION92_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION92_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION92_A {
        match self.bits {
            false => REGION92_A::DISABLED,
            true => REGION92_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION92_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION92_A::ENABLED
    }
}
#[doc = "Field `REGION92` writer - Enable protection for region 92. Write '0' has no effect."]
pub type REGION92_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION92_A, O>;
impl<'a, const O: u8> REGION92_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION92_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION92_A::ENABLED)
    }
}
#[doc = "Field `REGION93` reader - Enable protection for region 93. Write '0' has no effect."]
pub type REGION93_R = crate::BitReader<REGION93_A>;
#[doc = "Enable protection for region 93. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION93_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION93_A> for bool {
    #[inline(always)]
    fn from(variant: REGION93_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION93_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION93_A {
        match self.bits {
            false => REGION93_A::DISABLED,
            true => REGION93_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION93_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION93_A::ENABLED
    }
}
#[doc = "Field `REGION93` writer - Enable protection for region 93. Write '0' has no effect."]
pub type REGION93_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION93_A, O>;
impl<'a, const O: u8> REGION93_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION93_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION93_A::ENABLED)
    }
}
#[doc = "Field `REGION94` reader - Enable protection for region 94. Write '0' has no effect."]
pub type REGION94_R = crate::BitReader<REGION94_A>;
#[doc = "Enable protection for region 94. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION94_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION94_A> for bool {
    #[inline(always)]
    fn from(variant: REGION94_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION94_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION94_A {
        match self.bits {
            false => REGION94_A::DISABLED,
            true => REGION94_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION94_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION94_A::ENABLED
    }
}
#[doc = "Field `REGION94` writer - Enable protection for region 94. Write '0' has no effect."]
pub type REGION94_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION94_A, O>;
impl<'a, const O: u8> REGION94_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION94_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION94_A::ENABLED)
    }
}
#[doc = "Field `REGION95` reader - Enable protection for region 95. Write '0' has no effect."]
pub type REGION95_R = crate::BitReader<REGION95_A>;
#[doc = "Enable protection for region 95. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION95_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION95_A> for bool {
    #[inline(always)]
    fn from(variant: REGION95_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION95_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION95_A {
        match self.bits {
            false => REGION95_A::DISABLED,
            true => REGION95_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION95_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION95_A::ENABLED
    }
}
#[doc = "Field `REGION95` writer - Enable protection for region 95. Write '0' has no effect."]
pub type REGION95_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG2_SPEC, REGION95_A, O>;
impl<'a, const O: u8> REGION95_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION95_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION95_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    pub fn region64(&self) -> REGION64_R {
        REGION64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    pub fn region65(&self) -> REGION65_R {
        REGION65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    pub fn region66(&self) -> REGION66_R {
        REGION66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    pub fn region67(&self) -> REGION67_R {
        REGION67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    pub fn region68(&self) -> REGION68_R {
        REGION68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    pub fn region69(&self) -> REGION69_R {
        REGION69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    pub fn region70(&self) -> REGION70_R {
        REGION70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    pub fn region71(&self) -> REGION71_R {
        REGION71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    pub fn region72(&self) -> REGION72_R {
        REGION72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    pub fn region73(&self) -> REGION73_R {
        REGION73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    pub fn region74(&self) -> REGION74_R {
        REGION74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    pub fn region75(&self) -> REGION75_R {
        REGION75_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    pub fn region76(&self) -> REGION76_R {
        REGION76_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    pub fn region77(&self) -> REGION77_R {
        REGION77_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    pub fn region78(&self) -> REGION78_R {
        REGION78_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    pub fn region79(&self) -> REGION79_R {
        REGION79_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    pub fn region80(&self) -> REGION80_R {
        REGION80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    pub fn region81(&self) -> REGION81_R {
        REGION81_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    pub fn region82(&self) -> REGION82_R {
        REGION82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    pub fn region83(&self) -> REGION83_R {
        REGION83_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    pub fn region84(&self) -> REGION84_R {
        REGION84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    pub fn region85(&self) -> REGION85_R {
        REGION85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    pub fn region86(&self) -> REGION86_R {
        REGION86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    pub fn region87(&self) -> REGION87_R {
        REGION87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    pub fn region88(&self) -> REGION88_R {
        REGION88_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    pub fn region89(&self) -> REGION89_R {
        REGION89_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    pub fn region90(&self) -> REGION90_R {
        REGION90_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    pub fn region91(&self) -> REGION91_R {
        REGION91_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    pub fn region92(&self) -> REGION92_R {
        REGION92_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    pub fn region93(&self) -> REGION93_R {
        REGION93_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    pub fn region94(&self) -> REGION94_R {
        REGION94_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    pub fn region95(&self) -> REGION95_R {
        REGION95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region64(&mut self) -> REGION64_W<0> {
        REGION64_W::new(self)
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region65(&mut self) -> REGION65_W<1> {
        REGION65_W::new(self)
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region66(&mut self) -> REGION66_W<2> {
        REGION66_W::new(self)
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region67(&mut self) -> REGION67_W<3> {
        REGION67_W::new(self)
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region68(&mut self) -> REGION68_W<4> {
        REGION68_W::new(self)
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region69(&mut self) -> REGION69_W<5> {
        REGION69_W::new(self)
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region70(&mut self) -> REGION70_W<6> {
        REGION70_W::new(self)
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region71(&mut self) -> REGION71_W<7> {
        REGION71_W::new(self)
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region72(&mut self) -> REGION72_W<8> {
        REGION72_W::new(self)
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region73(&mut self) -> REGION73_W<9> {
        REGION73_W::new(self)
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region74(&mut self) -> REGION74_W<10> {
        REGION74_W::new(self)
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region75(&mut self) -> REGION75_W<11> {
        REGION75_W::new(self)
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region76(&mut self) -> REGION76_W<12> {
        REGION76_W::new(self)
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region77(&mut self) -> REGION77_W<13> {
        REGION77_W::new(self)
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region78(&mut self) -> REGION78_W<14> {
        REGION78_W::new(self)
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region79(&mut self) -> REGION79_W<15> {
        REGION79_W::new(self)
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region80(&mut self) -> REGION80_W<16> {
        REGION80_W::new(self)
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region81(&mut self) -> REGION81_W<17> {
        REGION81_W::new(self)
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region82(&mut self) -> REGION82_W<18> {
        REGION82_W::new(self)
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region83(&mut self) -> REGION83_W<19> {
        REGION83_W::new(self)
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region84(&mut self) -> REGION84_W<20> {
        REGION84_W::new(self)
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region85(&mut self) -> REGION85_W<21> {
        REGION85_W::new(self)
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region86(&mut self) -> REGION86_W<22> {
        REGION86_W::new(self)
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region87(&mut self) -> REGION87_W<23> {
        REGION87_W::new(self)
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region88(&mut self) -> REGION88_W<24> {
        REGION88_W::new(self)
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region89(&mut self) -> REGION89_W<25> {
        REGION89_W::new(self)
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region90(&mut self) -> REGION90_W<26> {
        REGION90_W::new(self)
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region91(&mut self) -> REGION91_W<27> {
        REGION91_W::new(self)
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region92(&mut self) -> REGION92_W<28> {
        REGION92_W::new(self)
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region93(&mut self) -> REGION93_W<29> {
        REGION93_W::new(self)
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region94(&mut self) -> REGION94_W<30> {
        REGION94_W::new(self)
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region95(&mut self) -> REGION95_W<31> {
        REGION95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block protect configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config2](index.html) module"]
pub struct CONFIG2_SPEC;
impl crate::RegisterSpec for CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config2::R](R) reader structure"]
impl crate::Readable for CONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config2::W](W) writer structure"]
impl crate::Writable for CONFIG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG2 to value 0"]
impl crate::Resettable for CONFIG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
