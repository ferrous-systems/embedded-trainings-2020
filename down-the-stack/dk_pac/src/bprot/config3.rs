#[doc = "Register `CONFIG3` reader"]
pub struct R(crate::R<CONFIG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG3` writer"]
pub struct W(crate::W<CONFIG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG3_SPEC>;
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
impl From<crate::W<CONFIG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION96` reader - Enable protection for region 96. Write '0' has no effect."]
pub type REGION96_R = crate::BitReader<REGION96_A>;
#[doc = "Enable protection for region 96. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION96_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION96_A> for bool {
    #[inline(always)]
    fn from(variant: REGION96_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION96_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION96_A {
        match self.bits {
            false => REGION96_A::DISABLED,
            true => REGION96_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION96_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION96_A::ENABLED
    }
}
#[doc = "Field `REGION96` writer - Enable protection for region 96. Write '0' has no effect."]
pub type REGION96_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION96_A, O>;
impl<'a, const O: u8> REGION96_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION96_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION96_A::ENABLED)
    }
}
#[doc = "Field `REGION97` reader - Enable protection for region 97. Write '0' has no effect."]
pub type REGION97_R = crate::BitReader<REGION97_A>;
#[doc = "Enable protection for region 97. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION97_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION97_A> for bool {
    #[inline(always)]
    fn from(variant: REGION97_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION97_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION97_A {
        match self.bits {
            false => REGION97_A::DISABLED,
            true => REGION97_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION97_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION97_A::ENABLED
    }
}
#[doc = "Field `REGION97` writer - Enable protection for region 97. Write '0' has no effect."]
pub type REGION97_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION97_A, O>;
impl<'a, const O: u8> REGION97_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION97_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION97_A::ENABLED)
    }
}
#[doc = "Field `REGION98` reader - Enable protection for region 98. Write '0' has no effect."]
pub type REGION98_R = crate::BitReader<REGION98_A>;
#[doc = "Enable protection for region 98. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION98_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION98_A> for bool {
    #[inline(always)]
    fn from(variant: REGION98_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION98_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION98_A {
        match self.bits {
            false => REGION98_A::DISABLED,
            true => REGION98_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION98_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION98_A::ENABLED
    }
}
#[doc = "Field `REGION98` writer - Enable protection for region 98. Write '0' has no effect."]
pub type REGION98_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION98_A, O>;
impl<'a, const O: u8> REGION98_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION98_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION98_A::ENABLED)
    }
}
#[doc = "Field `REGION99` reader - Enable protection for region 99. Write '0' has no effect."]
pub type REGION99_R = crate::BitReader<REGION99_A>;
#[doc = "Enable protection for region 99. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION99_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION99_A> for bool {
    #[inline(always)]
    fn from(variant: REGION99_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION99_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION99_A {
        match self.bits {
            false => REGION99_A::DISABLED,
            true => REGION99_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION99_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION99_A::ENABLED
    }
}
#[doc = "Field `REGION99` writer - Enable protection for region 99. Write '0' has no effect."]
pub type REGION99_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION99_A, O>;
impl<'a, const O: u8> REGION99_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION99_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION99_A::ENABLED)
    }
}
#[doc = "Field `REGION100` reader - Enable protection for region 100. Write '0' has no effect."]
pub type REGION100_R = crate::BitReader<REGION100_A>;
#[doc = "Enable protection for region 100. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION100_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION100_A> for bool {
    #[inline(always)]
    fn from(variant: REGION100_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION100_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION100_A {
        match self.bits {
            false => REGION100_A::DISABLED,
            true => REGION100_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION100_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION100_A::ENABLED
    }
}
#[doc = "Field `REGION100` writer - Enable protection for region 100. Write '0' has no effect."]
pub type REGION100_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION100_A, O>;
impl<'a, const O: u8> REGION100_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION100_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION100_A::ENABLED)
    }
}
#[doc = "Field `REGION101` reader - Enable protection for region 101. Write '0' has no effect."]
pub type REGION101_R = crate::BitReader<REGION101_A>;
#[doc = "Enable protection for region 101. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION101_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION101_A> for bool {
    #[inline(always)]
    fn from(variant: REGION101_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION101_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION101_A {
        match self.bits {
            false => REGION101_A::DISABLED,
            true => REGION101_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION101_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION101_A::ENABLED
    }
}
#[doc = "Field `REGION101` writer - Enable protection for region 101. Write '0' has no effect."]
pub type REGION101_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION101_A, O>;
impl<'a, const O: u8> REGION101_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION101_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION101_A::ENABLED)
    }
}
#[doc = "Field `REGION102` reader - Enable protection for region 102. Write '0' has no effect."]
pub type REGION102_R = crate::BitReader<REGION102_A>;
#[doc = "Enable protection for region 102. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION102_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION102_A> for bool {
    #[inline(always)]
    fn from(variant: REGION102_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION102_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION102_A {
        match self.bits {
            false => REGION102_A::DISABLED,
            true => REGION102_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION102_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION102_A::ENABLED
    }
}
#[doc = "Field `REGION102` writer - Enable protection for region 102. Write '0' has no effect."]
pub type REGION102_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION102_A, O>;
impl<'a, const O: u8> REGION102_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION102_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION102_A::ENABLED)
    }
}
#[doc = "Field `REGION103` reader - Enable protection for region 103. Write '0' has no effect."]
pub type REGION103_R = crate::BitReader<REGION103_A>;
#[doc = "Enable protection for region 103. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION103_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION103_A> for bool {
    #[inline(always)]
    fn from(variant: REGION103_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION103_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION103_A {
        match self.bits {
            false => REGION103_A::DISABLED,
            true => REGION103_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION103_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION103_A::ENABLED
    }
}
#[doc = "Field `REGION103` writer - Enable protection for region 103. Write '0' has no effect."]
pub type REGION103_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION103_A, O>;
impl<'a, const O: u8> REGION103_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION103_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION103_A::ENABLED)
    }
}
#[doc = "Field `REGION104` reader - Enable protection for region 104. Write '0' has no effect."]
pub type REGION104_R = crate::BitReader<REGION104_A>;
#[doc = "Enable protection for region 104. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION104_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION104_A> for bool {
    #[inline(always)]
    fn from(variant: REGION104_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION104_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION104_A {
        match self.bits {
            false => REGION104_A::DISABLED,
            true => REGION104_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION104_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION104_A::ENABLED
    }
}
#[doc = "Field `REGION104` writer - Enable protection for region 104. Write '0' has no effect."]
pub type REGION104_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION104_A, O>;
impl<'a, const O: u8> REGION104_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION104_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION104_A::ENABLED)
    }
}
#[doc = "Field `REGION105` reader - Enable protection for region 105. Write '0' has no effect."]
pub type REGION105_R = crate::BitReader<REGION105_A>;
#[doc = "Enable protection for region 105. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION105_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION105_A> for bool {
    #[inline(always)]
    fn from(variant: REGION105_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION105_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION105_A {
        match self.bits {
            false => REGION105_A::DISABLED,
            true => REGION105_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION105_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION105_A::ENABLED
    }
}
#[doc = "Field `REGION105` writer - Enable protection for region 105. Write '0' has no effect."]
pub type REGION105_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION105_A, O>;
impl<'a, const O: u8> REGION105_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION105_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION105_A::ENABLED)
    }
}
#[doc = "Field `REGION106` reader - Enable protection for region 106. Write '0' has no effect."]
pub type REGION106_R = crate::BitReader<REGION106_A>;
#[doc = "Enable protection for region 106. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION106_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION106_A> for bool {
    #[inline(always)]
    fn from(variant: REGION106_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION106_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION106_A {
        match self.bits {
            false => REGION106_A::DISABLED,
            true => REGION106_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION106_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION106_A::ENABLED
    }
}
#[doc = "Field `REGION106` writer - Enable protection for region 106. Write '0' has no effect."]
pub type REGION106_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION106_A, O>;
impl<'a, const O: u8> REGION106_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION106_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION106_A::ENABLED)
    }
}
#[doc = "Field `REGION107` reader - Enable protection for region 107. Write '0' has no effect."]
pub type REGION107_R = crate::BitReader<REGION107_A>;
#[doc = "Enable protection for region 107. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION107_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION107_A> for bool {
    #[inline(always)]
    fn from(variant: REGION107_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION107_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION107_A {
        match self.bits {
            false => REGION107_A::DISABLED,
            true => REGION107_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION107_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION107_A::ENABLED
    }
}
#[doc = "Field `REGION107` writer - Enable protection for region 107. Write '0' has no effect."]
pub type REGION107_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION107_A, O>;
impl<'a, const O: u8> REGION107_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION107_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION107_A::ENABLED)
    }
}
#[doc = "Field `REGION108` reader - Enable protection for region 108. Write '0' has no effect."]
pub type REGION108_R = crate::BitReader<REGION108_A>;
#[doc = "Enable protection for region 108. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION108_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION108_A> for bool {
    #[inline(always)]
    fn from(variant: REGION108_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION108_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION108_A {
        match self.bits {
            false => REGION108_A::DISABLED,
            true => REGION108_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION108_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION108_A::ENABLED
    }
}
#[doc = "Field `REGION108` writer - Enable protection for region 108. Write '0' has no effect."]
pub type REGION108_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION108_A, O>;
impl<'a, const O: u8> REGION108_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION108_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION108_A::ENABLED)
    }
}
#[doc = "Field `REGION109` reader - Enable protection for region 109. Write '0' has no effect."]
pub type REGION109_R = crate::BitReader<REGION109_A>;
#[doc = "Enable protection for region 109. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION109_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION109_A> for bool {
    #[inline(always)]
    fn from(variant: REGION109_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION109_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION109_A {
        match self.bits {
            false => REGION109_A::DISABLED,
            true => REGION109_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION109_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION109_A::ENABLED
    }
}
#[doc = "Field `REGION109` writer - Enable protection for region 109. Write '0' has no effect."]
pub type REGION109_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION109_A, O>;
impl<'a, const O: u8> REGION109_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION109_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION109_A::ENABLED)
    }
}
#[doc = "Field `REGION110` reader - Enable protection for region 110. Write '0' has no effect."]
pub type REGION110_R = crate::BitReader<REGION110_A>;
#[doc = "Enable protection for region 110. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION110_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION110_A> for bool {
    #[inline(always)]
    fn from(variant: REGION110_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION110_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION110_A {
        match self.bits {
            false => REGION110_A::DISABLED,
            true => REGION110_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION110_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION110_A::ENABLED
    }
}
#[doc = "Field `REGION110` writer - Enable protection for region 110. Write '0' has no effect."]
pub type REGION110_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION110_A, O>;
impl<'a, const O: u8> REGION110_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION110_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION110_A::ENABLED)
    }
}
#[doc = "Field `REGION111` reader - Enable protection for region 111. Write '0' has no effect."]
pub type REGION111_R = crate::BitReader<REGION111_A>;
#[doc = "Enable protection for region 111. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION111_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION111_A> for bool {
    #[inline(always)]
    fn from(variant: REGION111_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION111_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION111_A {
        match self.bits {
            false => REGION111_A::DISABLED,
            true => REGION111_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION111_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION111_A::ENABLED
    }
}
#[doc = "Field `REGION111` writer - Enable protection for region 111. Write '0' has no effect."]
pub type REGION111_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION111_A, O>;
impl<'a, const O: u8> REGION111_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION111_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION111_A::ENABLED)
    }
}
#[doc = "Field `REGION112` reader - Enable protection for region 112. Write '0' has no effect."]
pub type REGION112_R = crate::BitReader<REGION112_A>;
#[doc = "Enable protection for region 112. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION112_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION112_A> for bool {
    #[inline(always)]
    fn from(variant: REGION112_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION112_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION112_A {
        match self.bits {
            false => REGION112_A::DISABLED,
            true => REGION112_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION112_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION112_A::ENABLED
    }
}
#[doc = "Field `REGION112` writer - Enable protection for region 112. Write '0' has no effect."]
pub type REGION112_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION112_A, O>;
impl<'a, const O: u8> REGION112_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION112_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION112_A::ENABLED)
    }
}
#[doc = "Field `REGION113` reader - Enable protection for region 113. Write '0' has no effect."]
pub type REGION113_R = crate::BitReader<REGION113_A>;
#[doc = "Enable protection for region 113. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION113_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION113_A> for bool {
    #[inline(always)]
    fn from(variant: REGION113_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION113_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION113_A {
        match self.bits {
            false => REGION113_A::DISABLED,
            true => REGION113_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION113_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION113_A::ENABLED
    }
}
#[doc = "Field `REGION113` writer - Enable protection for region 113. Write '0' has no effect."]
pub type REGION113_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION113_A, O>;
impl<'a, const O: u8> REGION113_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION113_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION113_A::ENABLED)
    }
}
#[doc = "Field `REGION114` reader - Enable protection for region 114. Write '0' has no effect."]
pub type REGION114_R = crate::BitReader<REGION114_A>;
#[doc = "Enable protection for region 114. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION114_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION114_A> for bool {
    #[inline(always)]
    fn from(variant: REGION114_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION114_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION114_A {
        match self.bits {
            false => REGION114_A::DISABLED,
            true => REGION114_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION114_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION114_A::ENABLED
    }
}
#[doc = "Field `REGION114` writer - Enable protection for region 114. Write '0' has no effect."]
pub type REGION114_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION114_A, O>;
impl<'a, const O: u8> REGION114_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION114_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION114_A::ENABLED)
    }
}
#[doc = "Field `REGION115` reader - Enable protection for region 115. Write '0' has no effect."]
pub type REGION115_R = crate::BitReader<REGION115_A>;
#[doc = "Enable protection for region 115. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION115_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION115_A> for bool {
    #[inline(always)]
    fn from(variant: REGION115_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION115_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION115_A {
        match self.bits {
            false => REGION115_A::DISABLED,
            true => REGION115_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION115_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION115_A::ENABLED
    }
}
#[doc = "Field `REGION115` writer - Enable protection for region 115. Write '0' has no effect."]
pub type REGION115_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION115_A, O>;
impl<'a, const O: u8> REGION115_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION115_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION115_A::ENABLED)
    }
}
#[doc = "Field `REGION116` reader - Enable protection for region 116. Write '0' has no effect."]
pub type REGION116_R = crate::BitReader<REGION116_A>;
#[doc = "Enable protection for region 116. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION116_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION116_A> for bool {
    #[inline(always)]
    fn from(variant: REGION116_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION116_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION116_A {
        match self.bits {
            false => REGION116_A::DISABLED,
            true => REGION116_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION116_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION116_A::ENABLED
    }
}
#[doc = "Field `REGION116` writer - Enable protection for region 116. Write '0' has no effect."]
pub type REGION116_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION116_A, O>;
impl<'a, const O: u8> REGION116_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION116_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION116_A::ENABLED)
    }
}
#[doc = "Field `REGION117` reader - Enable protection for region 117. Write '0' has no effect."]
pub type REGION117_R = crate::BitReader<REGION117_A>;
#[doc = "Enable protection for region 117. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION117_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION117_A> for bool {
    #[inline(always)]
    fn from(variant: REGION117_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION117_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION117_A {
        match self.bits {
            false => REGION117_A::DISABLED,
            true => REGION117_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION117_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION117_A::ENABLED
    }
}
#[doc = "Field `REGION117` writer - Enable protection for region 117. Write '0' has no effect."]
pub type REGION117_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION117_A, O>;
impl<'a, const O: u8> REGION117_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION117_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION117_A::ENABLED)
    }
}
#[doc = "Field `REGION118` reader - Enable protection for region 118. Write '0' has no effect."]
pub type REGION118_R = crate::BitReader<REGION118_A>;
#[doc = "Enable protection for region 118. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION118_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION118_A> for bool {
    #[inline(always)]
    fn from(variant: REGION118_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION118_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION118_A {
        match self.bits {
            false => REGION118_A::DISABLED,
            true => REGION118_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION118_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION118_A::ENABLED
    }
}
#[doc = "Field `REGION118` writer - Enable protection for region 118. Write '0' has no effect."]
pub type REGION118_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION118_A, O>;
impl<'a, const O: u8> REGION118_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION118_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION118_A::ENABLED)
    }
}
#[doc = "Field `REGION119` reader - Enable protection for region 119. Write '0' has no effect."]
pub type REGION119_R = crate::BitReader<REGION119_A>;
#[doc = "Enable protection for region 119. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION119_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION119_A> for bool {
    #[inline(always)]
    fn from(variant: REGION119_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION119_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION119_A {
        match self.bits {
            false => REGION119_A::DISABLED,
            true => REGION119_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION119_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION119_A::ENABLED
    }
}
#[doc = "Field `REGION119` writer - Enable protection for region 119. Write '0' has no effect."]
pub type REGION119_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION119_A, O>;
impl<'a, const O: u8> REGION119_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION119_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION119_A::ENABLED)
    }
}
#[doc = "Field `REGION120` reader - Enable protection for region 120. Write '0' has no effect."]
pub type REGION120_R = crate::BitReader<REGION120_A>;
#[doc = "Enable protection for region 120. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION120_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION120_A> for bool {
    #[inline(always)]
    fn from(variant: REGION120_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION120_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION120_A {
        match self.bits {
            false => REGION120_A::DISABLED,
            true => REGION120_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION120_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION120_A::ENABLED
    }
}
#[doc = "Field `REGION120` writer - Enable protection for region 120. Write '0' has no effect."]
pub type REGION120_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION120_A, O>;
impl<'a, const O: u8> REGION120_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION120_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION120_A::ENABLED)
    }
}
#[doc = "Field `REGION121` reader - Enable protection for region 121. Write '0' has no effect."]
pub type REGION121_R = crate::BitReader<REGION121_A>;
#[doc = "Enable protection for region 121. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION121_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION121_A> for bool {
    #[inline(always)]
    fn from(variant: REGION121_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION121_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION121_A {
        match self.bits {
            false => REGION121_A::DISABLED,
            true => REGION121_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION121_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION121_A::ENABLED
    }
}
#[doc = "Field `REGION121` writer - Enable protection for region 121. Write '0' has no effect."]
pub type REGION121_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION121_A, O>;
impl<'a, const O: u8> REGION121_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION121_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION121_A::ENABLED)
    }
}
#[doc = "Field `REGION122` reader - Enable protection for region 122. Write '0' has no effect."]
pub type REGION122_R = crate::BitReader<REGION122_A>;
#[doc = "Enable protection for region 122. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION122_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION122_A> for bool {
    #[inline(always)]
    fn from(variant: REGION122_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION122_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION122_A {
        match self.bits {
            false => REGION122_A::DISABLED,
            true => REGION122_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION122_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION122_A::ENABLED
    }
}
#[doc = "Field `REGION122` writer - Enable protection for region 122. Write '0' has no effect."]
pub type REGION122_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION122_A, O>;
impl<'a, const O: u8> REGION122_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION122_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION122_A::ENABLED)
    }
}
#[doc = "Field `REGION123` reader - Enable protection for region 123. Write '0' has no effect."]
pub type REGION123_R = crate::BitReader<REGION123_A>;
#[doc = "Enable protection for region 123. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION123_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION123_A> for bool {
    #[inline(always)]
    fn from(variant: REGION123_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION123_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION123_A {
        match self.bits {
            false => REGION123_A::DISABLED,
            true => REGION123_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION123_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION123_A::ENABLED
    }
}
#[doc = "Field `REGION123` writer - Enable protection for region 123. Write '0' has no effect."]
pub type REGION123_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION123_A, O>;
impl<'a, const O: u8> REGION123_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION123_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION123_A::ENABLED)
    }
}
#[doc = "Field `REGION124` reader - Enable protection for region 124. Write '0' has no effect."]
pub type REGION124_R = crate::BitReader<REGION124_A>;
#[doc = "Enable protection for region 124. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION124_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION124_A> for bool {
    #[inline(always)]
    fn from(variant: REGION124_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION124_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION124_A {
        match self.bits {
            false => REGION124_A::DISABLED,
            true => REGION124_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION124_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION124_A::ENABLED
    }
}
#[doc = "Field `REGION124` writer - Enable protection for region 124. Write '0' has no effect."]
pub type REGION124_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION124_A, O>;
impl<'a, const O: u8> REGION124_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION124_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION124_A::ENABLED)
    }
}
#[doc = "Field `REGION125` reader - Enable protection for region 125. Write '0' has no effect."]
pub type REGION125_R = crate::BitReader<REGION125_A>;
#[doc = "Enable protection for region 125. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION125_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION125_A> for bool {
    #[inline(always)]
    fn from(variant: REGION125_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION125_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION125_A {
        match self.bits {
            false => REGION125_A::DISABLED,
            true => REGION125_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION125_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION125_A::ENABLED
    }
}
#[doc = "Field `REGION125` writer - Enable protection for region 125. Write '0' has no effect."]
pub type REGION125_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION125_A, O>;
impl<'a, const O: u8> REGION125_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION125_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION125_A::ENABLED)
    }
}
#[doc = "Field `REGION126` reader - Enable protection for region 126. Write '0' has no effect."]
pub type REGION126_R = crate::BitReader<REGION126_A>;
#[doc = "Enable protection for region 126. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION126_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION126_A> for bool {
    #[inline(always)]
    fn from(variant: REGION126_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION126_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION126_A {
        match self.bits {
            false => REGION126_A::DISABLED,
            true => REGION126_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION126_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION126_A::ENABLED
    }
}
#[doc = "Field `REGION126` writer - Enable protection for region 126. Write '0' has no effect."]
pub type REGION126_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION126_A, O>;
impl<'a, const O: u8> REGION126_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION126_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION126_A::ENABLED)
    }
}
#[doc = "Field `REGION127` reader - Enable protection for region 127. Write '0' has no effect."]
pub type REGION127_R = crate::BitReader<REGION127_A>;
#[doc = "Enable protection for region 127. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION127_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION127_A> for bool {
    #[inline(always)]
    fn from(variant: REGION127_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION127_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION127_A {
        match self.bits {
            false => REGION127_A::DISABLED,
            true => REGION127_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION127_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION127_A::ENABLED
    }
}
#[doc = "Field `REGION127` writer - Enable protection for region 127. Write '0' has no effect."]
pub type REGION127_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, REGION127_A, O>;
impl<'a, const O: u8> REGION127_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION127_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION127_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    pub fn region96(&self) -> REGION96_R {
        REGION96_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    pub fn region97(&self) -> REGION97_R {
        REGION97_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    pub fn region98(&self) -> REGION98_R {
        REGION98_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    pub fn region99(&self) -> REGION99_R {
        REGION99_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    pub fn region100(&self) -> REGION100_R {
        REGION100_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    pub fn region101(&self) -> REGION101_R {
        REGION101_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    pub fn region102(&self) -> REGION102_R {
        REGION102_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    pub fn region103(&self) -> REGION103_R {
        REGION103_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    pub fn region104(&self) -> REGION104_R {
        REGION104_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    pub fn region105(&self) -> REGION105_R {
        REGION105_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    pub fn region106(&self) -> REGION106_R {
        REGION106_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    pub fn region107(&self) -> REGION107_R {
        REGION107_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    pub fn region108(&self) -> REGION108_R {
        REGION108_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    pub fn region109(&self) -> REGION109_R {
        REGION109_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    pub fn region110(&self) -> REGION110_R {
        REGION110_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    pub fn region111(&self) -> REGION111_R {
        REGION111_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    pub fn region112(&self) -> REGION112_R {
        REGION112_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    pub fn region113(&self) -> REGION113_R {
        REGION113_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    pub fn region114(&self) -> REGION114_R {
        REGION114_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    pub fn region115(&self) -> REGION115_R {
        REGION115_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    pub fn region116(&self) -> REGION116_R {
        REGION116_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    pub fn region117(&self) -> REGION117_R {
        REGION117_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    pub fn region118(&self) -> REGION118_R {
        REGION118_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    pub fn region119(&self) -> REGION119_R {
        REGION119_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    pub fn region120(&self) -> REGION120_R {
        REGION120_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    pub fn region121(&self) -> REGION121_R {
        REGION121_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    pub fn region122(&self) -> REGION122_R {
        REGION122_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    pub fn region123(&self) -> REGION123_R {
        REGION123_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    pub fn region124(&self) -> REGION124_R {
        REGION124_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    pub fn region125(&self) -> REGION125_R {
        REGION125_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    pub fn region126(&self) -> REGION126_R {
        REGION126_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    pub fn region127(&self) -> REGION127_R {
        REGION127_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region96(&mut self) -> REGION96_W<0> {
        REGION96_W::new(self)
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region97(&mut self) -> REGION97_W<1> {
        REGION97_W::new(self)
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region98(&mut self) -> REGION98_W<2> {
        REGION98_W::new(self)
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region99(&mut self) -> REGION99_W<3> {
        REGION99_W::new(self)
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region100(&mut self) -> REGION100_W<4> {
        REGION100_W::new(self)
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region101(&mut self) -> REGION101_W<5> {
        REGION101_W::new(self)
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region102(&mut self) -> REGION102_W<6> {
        REGION102_W::new(self)
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region103(&mut self) -> REGION103_W<7> {
        REGION103_W::new(self)
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region104(&mut self) -> REGION104_W<8> {
        REGION104_W::new(self)
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region105(&mut self) -> REGION105_W<9> {
        REGION105_W::new(self)
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region106(&mut self) -> REGION106_W<10> {
        REGION106_W::new(self)
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region107(&mut self) -> REGION107_W<11> {
        REGION107_W::new(self)
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region108(&mut self) -> REGION108_W<12> {
        REGION108_W::new(self)
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region109(&mut self) -> REGION109_W<13> {
        REGION109_W::new(self)
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region110(&mut self) -> REGION110_W<14> {
        REGION110_W::new(self)
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region111(&mut self) -> REGION111_W<15> {
        REGION111_W::new(self)
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region112(&mut self) -> REGION112_W<16> {
        REGION112_W::new(self)
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region113(&mut self) -> REGION113_W<17> {
        REGION113_W::new(self)
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region114(&mut self) -> REGION114_W<18> {
        REGION114_W::new(self)
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region115(&mut self) -> REGION115_W<19> {
        REGION115_W::new(self)
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region116(&mut self) -> REGION116_W<20> {
        REGION116_W::new(self)
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region117(&mut self) -> REGION117_W<21> {
        REGION117_W::new(self)
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region118(&mut self) -> REGION118_W<22> {
        REGION118_W::new(self)
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region119(&mut self) -> REGION119_W<23> {
        REGION119_W::new(self)
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region120(&mut self) -> REGION120_W<24> {
        REGION120_W::new(self)
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region121(&mut self) -> REGION121_W<25> {
        REGION121_W::new(self)
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region122(&mut self) -> REGION122_W<26> {
        REGION122_W::new(self)
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region123(&mut self) -> REGION123_W<27> {
        REGION123_W::new(self)
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region124(&mut self) -> REGION124_W<28> {
        REGION124_W::new(self)
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region125(&mut self) -> REGION125_W<29> {
        REGION125_W::new(self)
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region126(&mut self) -> REGION126_W<30> {
        REGION126_W::new(self)
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region127(&mut self) -> REGION127_W<31> {
        REGION127_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block protect configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config3](index.html) module"]
pub struct CONFIG3_SPEC;
impl crate::RegisterSpec for CONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config3::R](R) reader structure"]
impl crate::Readable for CONFIG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config3::W](W) writer structure"]
impl crate::Writable for CONFIG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG3 to value 0"]
impl crate::Resettable for CONFIG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
