#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARE0_CLEAR` reader - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
pub type COMPARE0_CLEAR_R = crate::BitReader<COMPARE0_CLEAR_A>;
#[doc = "Shortcut between event COMPARE\\[0\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE0_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE0_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE0_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_CLEAR_A {
        match self.bits {
            false => COMPARE0_CLEAR_A::DISABLED,
            true => COMPARE0_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE0_CLEAR` writer - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
pub type COMPARE0_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE0_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE0_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE1_CLEAR` reader - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
pub type COMPARE1_CLEAR_R = crate::BitReader<COMPARE1_CLEAR_A>;
#[doc = "Shortcut between event COMPARE\\[1\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE1_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE1_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE1_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_CLEAR_A {
        match self.bits {
            false => COMPARE1_CLEAR_A::DISABLED,
            true => COMPARE1_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE1_CLEAR` writer - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
pub type COMPARE1_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE1_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE1_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE2_CLEAR` reader - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
pub type COMPARE2_CLEAR_R = crate::BitReader<COMPARE2_CLEAR_A>;
#[doc = "Shortcut between event COMPARE\\[2\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE2_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE2_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE2_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_CLEAR_A {
        match self.bits {
            false => COMPARE2_CLEAR_A::DISABLED,
            true => COMPARE2_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE2_CLEAR` writer - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
pub type COMPARE2_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE2_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE2_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE3_CLEAR` reader - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
pub type COMPARE3_CLEAR_R = crate::BitReader<COMPARE3_CLEAR_A>;
#[doc = "Shortcut between event COMPARE\\[3\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE3_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE3_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE3_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_CLEAR_A {
        match self.bits {
            false => COMPARE3_CLEAR_A::DISABLED,
            true => COMPARE3_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE3_CLEAR` writer - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
pub type COMPARE3_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE3_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE3_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE4_CLEAR` reader - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
pub type COMPARE4_CLEAR_R = crate::BitReader<COMPARE4_CLEAR_A>;
#[doc = "Shortcut between event COMPARE\\[4\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE4_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE4_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE4_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE4_CLEAR_A {
        match self.bits {
            false => COMPARE4_CLEAR_A::DISABLED,
            true => COMPARE4_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE4_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE4_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE4_CLEAR` writer - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
pub type COMPARE4_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE4_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE4_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE4_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE4_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE5_CLEAR` reader - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
pub type COMPARE5_CLEAR_R = crate::BitReader<COMPARE5_CLEAR_A>;
#[doc = "Shortcut between event COMPARE\\[5\\]
and task CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE5_CLEAR_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE5_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE5_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE5_CLEAR_A {
        match self.bits {
            false => COMPARE5_CLEAR_A::DISABLED,
            true => COMPARE5_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE5_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE5_CLEAR_A::ENABLED
    }
}
#[doc = "Field `COMPARE5_CLEAR` writer - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
pub type COMPARE5_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE5_CLEAR_A, O>;
impl<'a, const O: u8> COMPARE5_CLEAR_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE5_CLEAR_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE5_CLEAR_A::ENABLED)
    }
}
#[doc = "Field `COMPARE0_STOP` reader - Shortcut between event COMPARE\\[0\\]
and task STOP"]
pub type COMPARE0_STOP_R = crate::BitReader<COMPARE0_STOP_A>;
#[doc = "Shortcut between event COMPARE\\[0\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE0_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE0_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_STOP_A {
        match self.bits {
            false => COMPARE0_STOP_A::DISABLED,
            true => COMPARE0_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE0_STOP` writer - Shortcut between event COMPARE\\[0\\]
and task STOP"]
pub type COMPARE0_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE0_STOP_A, O>;
impl<'a, const O: u8> COMPARE0_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE1_STOP` reader - Shortcut between event COMPARE\\[1\\]
and task STOP"]
pub type COMPARE1_STOP_R = crate::BitReader<COMPARE1_STOP_A>;
#[doc = "Shortcut between event COMPARE\\[1\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE1_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_STOP_A {
        match self.bits {
            false => COMPARE1_STOP_A::DISABLED,
            true => COMPARE1_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE1_STOP` writer - Shortcut between event COMPARE\\[1\\]
and task STOP"]
pub type COMPARE1_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE1_STOP_A, O>;
impl<'a, const O: u8> COMPARE1_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE2_STOP` reader - Shortcut between event COMPARE\\[2\\]
and task STOP"]
pub type COMPARE2_STOP_R = crate::BitReader<COMPARE2_STOP_A>;
#[doc = "Shortcut between event COMPARE\\[2\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE2_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE2_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_STOP_A {
        match self.bits {
            false => COMPARE2_STOP_A::DISABLED,
            true => COMPARE2_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE2_STOP` writer - Shortcut between event COMPARE\\[2\\]
and task STOP"]
pub type COMPARE2_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE2_STOP_A, O>;
impl<'a, const O: u8> COMPARE2_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE3_STOP` reader - Shortcut between event COMPARE\\[3\\]
and task STOP"]
pub type COMPARE3_STOP_R = crate::BitReader<COMPARE3_STOP_A>;
#[doc = "Shortcut between event COMPARE\\[3\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE3_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE3_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_STOP_A {
        match self.bits {
            false => COMPARE3_STOP_A::DISABLED,
            true => COMPARE3_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE3_STOP` writer - Shortcut between event COMPARE\\[3\\]
and task STOP"]
pub type COMPARE3_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE3_STOP_A, O>;
impl<'a, const O: u8> COMPARE3_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE4_STOP` reader - Shortcut between event COMPARE\\[4\\]
and task STOP"]
pub type COMPARE4_STOP_R = crate::BitReader<COMPARE4_STOP_A>;
#[doc = "Shortcut between event COMPARE\\[4\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE4_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE4_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE4_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE4_STOP_A {
        match self.bits {
            false => COMPARE4_STOP_A::DISABLED,
            true => COMPARE4_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE4_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE4_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE4_STOP` writer - Shortcut between event COMPARE\\[4\\]
and task STOP"]
pub type COMPARE4_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE4_STOP_A, O>;
impl<'a, const O: u8> COMPARE4_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE4_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE4_STOP_A::ENABLED)
    }
}
#[doc = "Field `COMPARE5_STOP` reader - Shortcut between event COMPARE\\[5\\]
and task STOP"]
pub type COMPARE5_STOP_R = crate::BitReader<COMPARE5_STOP_A>;
#[doc = "Shortcut between event COMPARE\\[5\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE5_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<COMPARE5_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE5_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE5_STOP_A {
        match self.bits {
            false => COMPARE5_STOP_A::DISABLED,
            true => COMPARE5_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE5_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE5_STOP_A::ENABLED
    }
}
#[doc = "Field `COMPARE5_STOP` writer - Shortcut between event COMPARE\\[5\\]
and task STOP"]
pub type COMPARE5_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, COMPARE5_STOP_A, O>;
impl<'a, const O: u8> COMPARE5_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE5_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE5_STOP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare0_clear(&self) -> COMPARE0_CLEAR_R {
        COMPARE0_CLEAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare1_clear(&self) -> COMPARE1_CLEAR_R {
        COMPARE1_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare2_clear(&self) -> COMPARE2_CLEAR_R {
        COMPARE2_CLEAR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare3_clear(&self) -> COMPARE3_CLEAR_R {
        COMPARE3_CLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare4_clear(&self) -> COMPARE4_CLEAR_R {
        COMPARE4_CLEAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
    #[inline(always)]
    pub fn compare5_clear(&self) -> COMPARE5_CLEAR_R {
        COMPARE5_CLEAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event COMPARE\\[0\\]
and task STOP"]
    #[inline(always)]
    pub fn compare0_stop(&self) -> COMPARE0_STOP_R {
        COMPARE0_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Shortcut between event COMPARE\\[1\\]
and task STOP"]
    #[inline(always)]
    pub fn compare1_stop(&self) -> COMPARE1_STOP_R {
        COMPARE1_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Shortcut between event COMPARE\\[2\\]
and task STOP"]
    #[inline(always)]
    pub fn compare2_stop(&self) -> COMPARE2_STOP_R {
        COMPARE2_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Shortcut between event COMPARE\\[3\\]
and task STOP"]
    #[inline(always)]
    pub fn compare3_stop(&self) -> COMPARE3_STOP_R {
        COMPARE3_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Shortcut between event COMPARE\\[4\\]
and task STOP"]
    #[inline(always)]
    pub fn compare4_stop(&self) -> COMPARE4_STOP_R {
        COMPARE4_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Shortcut between event COMPARE\\[5\\]
and task STOP"]
    #[inline(always)]
    pub fn compare5_stop(&self) -> COMPARE5_STOP_R {
        COMPARE5_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event COMPARE\\[0\\]
and task CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn compare0_clear(&mut self) -> COMPARE0_CLEAR_W<0> {
        COMPARE0_CLEAR_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event COMPARE\\[1\\]
and task CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn compare1_clear(&mut self) -> COMPARE1_CLEAR_W<1> {
        COMPARE1_CLEAR_W::new(self)
    }
    #[doc = "Bit 2 - Shortcut between event COMPARE\\[2\\]
and task CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn compare2_clear(&mut self) -> COMPARE2_CLEAR_W<2> {
        COMPARE2_CLEAR_W::new(self)
    }
    #[doc = "Bit 3 - Shortcut between event COMPARE\\[3\\]
and task CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn compare3_clear(&mut self) -> COMPARE3_CLEAR_W<3> {
        COMPARE3_CLEAR_W::new(self)
    }
    #[doc = "Bit 4 - Shortcut between event COMPARE\\[4\\]
and task CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn compare4_clear(&mut self) -> COMPARE4_CLEAR_W<4> {
        COMPARE4_CLEAR_W::new(self)
    }
    #[doc = "Bit 5 - Shortcut between event COMPARE\\[5\\]
and task CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn compare5_clear(&mut self) -> COMPARE5_CLEAR_W<5> {
        COMPARE5_CLEAR_W::new(self)
    }
    #[doc = "Bit 8 - Shortcut between event COMPARE\\[0\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn compare0_stop(&mut self) -> COMPARE0_STOP_W<8> {
        COMPARE0_STOP_W::new(self)
    }
    #[doc = "Bit 9 - Shortcut between event COMPARE\\[1\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn compare1_stop(&mut self) -> COMPARE1_STOP_W<9> {
        COMPARE1_STOP_W::new(self)
    }
    #[doc = "Bit 10 - Shortcut between event COMPARE\\[2\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn compare2_stop(&mut self) -> COMPARE2_STOP_W<10> {
        COMPARE2_STOP_W::new(self)
    }
    #[doc = "Bit 11 - Shortcut between event COMPARE\\[3\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn compare3_stop(&mut self) -> COMPARE3_STOP_W<11> {
        COMPARE3_STOP_W::new(self)
    }
    #[doc = "Bit 12 - Shortcut between event COMPARE\\[4\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn compare4_stop(&mut self) -> COMPARE4_STOP_W<12> {
        COMPARE4_STOP_W::new(self)
    }
    #[doc = "Bit 13 - Shortcut between event COMPARE\\[5\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn compare5_stop(&mut self) -> COMPARE5_STOP_W<13> {
        COMPARE5_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
