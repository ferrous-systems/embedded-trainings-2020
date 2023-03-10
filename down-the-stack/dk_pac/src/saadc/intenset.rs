#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTED` reader - Write '1' to enable interrupt for event STARTED"]
pub type STARTED_R = crate::BitReader<STARTED_A>;
#[doc = "Write '1' to enable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: STARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl STARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTED_A {
        match self.bits {
            false => STARTED_A::DISABLED,
            true => STARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<STARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: STARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` writer - Write '1' to enable interrupt for event STARTED"]
pub type STARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, STARTED_AW, O>;
impl<'a, const O: u8> STARTED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STARTED_AW::SET)
    }
}
#[doc = "Field `END` reader - Write '1' to enable interrupt for event END"]
pub type END_R = crate::BitReader<END_A>;
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum END_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
impl END_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum END_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to enable interrupt for event END"]
pub type END_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, END_AW, O>;
impl<'a, const O: u8> END_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(END_AW::SET)
    }
}
#[doc = "Field `DONE` reader - Write '1' to enable interrupt for event DONE"]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "Write '1' to enable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::DISABLED,
            true => DONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DONE_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DONE_AW> for bool {
    #[inline(always)]
    fn from(variant: DONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - Write '1' to enable interrupt for event DONE"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, DONE_AW, O>;
impl<'a, const O: u8> DONE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DONE_AW::SET)
    }
}
#[doc = "Field `RESULTDONE` reader - Write '1' to enable interrupt for event RESULTDONE"]
pub type RESULTDONE_R = crate::BitReader<RESULTDONE_A>;
#[doc = "Write '1' to enable interrupt for event RESULTDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESULTDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RESULTDONE_A> for bool {
    #[inline(always)]
    fn from(variant: RESULTDONE_A) -> Self {
        variant as u8 != 0
    }
}
impl RESULTDONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESULTDONE_A {
        match self.bits {
            false => RESULTDONE_A::DISABLED,
            true => RESULTDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESULTDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESULTDONE_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event RESULTDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESULTDONE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RESULTDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: RESULTDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESULTDONE` writer - Write '1' to enable interrupt for event RESULTDONE"]
pub type RESULTDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, RESULTDONE_AW, O>;
impl<'a, const O: u8> RESULTDONE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RESULTDONE_AW::SET)
    }
}
#[doc = "Field `CALIBRATEDONE` reader - Write '1' to enable interrupt for event CALIBRATEDONE"]
pub type CALIBRATEDONE_R = crate::BitReader<CALIBRATEDONE_A>;
#[doc = "Write '1' to enable interrupt for event CALIBRATEDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALIBRATEDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CALIBRATEDONE_A> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATEDONE_A) -> Self {
        variant as u8 != 0
    }
}
impl CALIBRATEDONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIBRATEDONE_A {
        match self.bits {
            false => CALIBRATEDONE_A::DISABLED,
            true => CALIBRATEDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALIBRATEDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALIBRATEDONE_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CALIBRATEDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALIBRATEDONE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CALIBRATEDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATEDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALIBRATEDONE` writer - Write '1' to enable interrupt for event CALIBRATEDONE"]
pub type CALIBRATEDONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, CALIBRATEDONE_AW, O>;
impl<'a, const O: u8> CALIBRATEDONE_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CALIBRATEDONE_AW::SET)
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to enable interrupt for event STOPPED"]
pub type STOPPED_R = crate::BitReader<STOPPED_A>;
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPPED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPPED_A {
        match self.bits {
            false => STOPPED_A::DISABLED,
            true => STOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPPED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<STOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to enable interrupt for event STOPPED"]
pub type STOPPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, STOPPED_AW, O>;
impl<'a, const O: u8> STOPPED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STOPPED_AW::SET)
    }
}
#[doc = "Field `CH0LIMITH` reader - Write '1' to enable interrupt for event CH0LIMITH"]
pub type CH0LIMITH_R = crate::BitReader<CH0LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH0LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH0LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0LIMITH_A {
        match self.bits {
            false => CH0LIMITH_A::DISABLED,
            true => CH0LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH0LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH0LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITH` writer - Write '1' to enable interrupt for event CH0LIMITH"]
pub type CH0LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH0LIMITH_AW, O>;
impl<'a, const O: u8> CH0LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH0LIMITH_AW::SET)
    }
}
#[doc = "Field `CH0LIMITL` reader - Write '1' to enable interrupt for event CH0LIMITL"]
pub type CH0LIMITL_R = crate::BitReader<CH0LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH0LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH0LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0LIMITL_A {
        match self.bits {
            false => CH0LIMITL_A::DISABLED,
            true => CH0LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH0LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH0LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0LIMITL` writer - Write '1' to enable interrupt for event CH0LIMITL"]
pub type CH0LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH0LIMITL_AW, O>;
impl<'a, const O: u8> CH0LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH0LIMITL_AW::SET)
    }
}
#[doc = "Field `CH1LIMITH` reader - Write '1' to enable interrupt for event CH1LIMITH"]
pub type CH1LIMITH_R = crate::BitReader<CH1LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH1LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH1LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1LIMITH_A {
        match self.bits {
            false => CH1LIMITH_A::DISABLED,
            true => CH1LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH1LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH1LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITH` writer - Write '1' to enable interrupt for event CH1LIMITH"]
pub type CH1LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH1LIMITH_AW, O>;
impl<'a, const O: u8> CH1LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1LIMITH_AW::SET)
    }
}
#[doc = "Field `CH1LIMITL` reader - Write '1' to enable interrupt for event CH1LIMITL"]
pub type CH1LIMITL_R = crate::BitReader<CH1LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH1LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH1LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1LIMITL_A {
        match self.bits {
            false => CH1LIMITL_A::DISABLED,
            true => CH1LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH1LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH1LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1LIMITL` writer - Write '1' to enable interrupt for event CH1LIMITL"]
pub type CH1LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH1LIMITL_AW, O>;
impl<'a, const O: u8> CH1LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1LIMITL_AW::SET)
    }
}
#[doc = "Field `CH2LIMITH` reader - Write '1' to enable interrupt for event CH2LIMITH"]
pub type CH2LIMITH_R = crate::BitReader<CH2LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH2LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH2LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2LIMITH_A {
        match self.bits {
            false => CH2LIMITH_A::DISABLED,
            true => CH2LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH2LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH2LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITH` writer - Write '1' to enable interrupt for event CH2LIMITH"]
pub type CH2LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH2LIMITH_AW, O>;
impl<'a, const O: u8> CH2LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2LIMITH_AW::SET)
    }
}
#[doc = "Field `CH2LIMITL` reader - Write '1' to enable interrupt for event CH2LIMITL"]
pub type CH2LIMITL_R = crate::BitReader<CH2LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH2LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH2LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2LIMITL_A {
        match self.bits {
            false => CH2LIMITL_A::DISABLED,
            true => CH2LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH2LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH2LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2LIMITL` writer - Write '1' to enable interrupt for event CH2LIMITL"]
pub type CH2LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH2LIMITL_AW, O>;
impl<'a, const O: u8> CH2LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2LIMITL_AW::SET)
    }
}
#[doc = "Field `CH3LIMITH` reader - Write '1' to enable interrupt for event CH3LIMITH"]
pub type CH3LIMITH_R = crate::BitReader<CH3LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH3LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH3LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3LIMITH_A {
        match self.bits {
            false => CH3LIMITH_A::DISABLED,
            true => CH3LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH3LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH3LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITH` writer - Write '1' to enable interrupt for event CH3LIMITH"]
pub type CH3LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH3LIMITH_AW, O>;
impl<'a, const O: u8> CH3LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3LIMITH_AW::SET)
    }
}
#[doc = "Field `CH3LIMITL` reader - Write '1' to enable interrupt for event CH3LIMITL"]
pub type CH3LIMITL_R = crate::BitReader<CH3LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH3LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH3LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3LIMITL_A {
        match self.bits {
            false => CH3LIMITL_A::DISABLED,
            true => CH3LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH3LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH3LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3LIMITL` writer - Write '1' to enable interrupt for event CH3LIMITL"]
pub type CH3LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH3LIMITL_AW, O>;
impl<'a, const O: u8> CH3LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3LIMITL_AW::SET)
    }
}
#[doc = "Field `CH4LIMITH` reader - Write '1' to enable interrupt for event CH4LIMITH"]
pub type CH4LIMITH_R = crate::BitReader<CH4LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH4LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH4LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4LIMITH_A {
        match self.bits {
            false => CH4LIMITH_A::DISABLED,
            true => CH4LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH4LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH4LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITH` writer - Write '1' to enable interrupt for event CH4LIMITH"]
pub type CH4LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH4LIMITH_AW, O>;
impl<'a, const O: u8> CH4LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4LIMITH_AW::SET)
    }
}
#[doc = "Field `CH4LIMITL` reader - Write '1' to enable interrupt for event CH4LIMITL"]
pub type CH4LIMITL_R = crate::BitReader<CH4LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH4LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH4LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4LIMITL_A {
        match self.bits {
            false => CH4LIMITL_A::DISABLED,
            true => CH4LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH4LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH4LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4LIMITL` writer - Write '1' to enable interrupt for event CH4LIMITL"]
pub type CH4LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH4LIMITL_AW, O>;
impl<'a, const O: u8> CH4LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4LIMITL_AW::SET)
    }
}
#[doc = "Field `CH5LIMITH` reader - Write '1' to enable interrupt for event CH5LIMITH"]
pub type CH5LIMITH_R = crate::BitReader<CH5LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH5LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH5LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5LIMITH_A {
        match self.bits {
            false => CH5LIMITH_A::DISABLED,
            true => CH5LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH5LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH5LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITH` writer - Write '1' to enable interrupt for event CH5LIMITH"]
pub type CH5LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH5LIMITH_AW, O>;
impl<'a, const O: u8> CH5LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5LIMITH_AW::SET)
    }
}
#[doc = "Field `CH5LIMITL` reader - Write '1' to enable interrupt for event CH5LIMITL"]
pub type CH5LIMITL_R = crate::BitReader<CH5LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH5LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH5LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5LIMITL_A {
        match self.bits {
            false => CH5LIMITL_A::DISABLED,
            true => CH5LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH5LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH5LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5LIMITL` writer - Write '1' to enable interrupt for event CH5LIMITL"]
pub type CH5LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH5LIMITL_AW, O>;
impl<'a, const O: u8> CH5LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5LIMITL_AW::SET)
    }
}
#[doc = "Field `CH6LIMITH` reader - Write '1' to enable interrupt for event CH6LIMITH"]
pub type CH6LIMITH_R = crate::BitReader<CH6LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH6LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH6LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6LIMITH_A {
        match self.bits {
            false => CH6LIMITH_A::DISABLED,
            true => CH6LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH6LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH6LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITH` writer - Write '1' to enable interrupt for event CH6LIMITH"]
pub type CH6LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH6LIMITH_AW, O>;
impl<'a, const O: u8> CH6LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6LIMITH_AW::SET)
    }
}
#[doc = "Field `CH6LIMITL` reader - Write '1' to enable interrupt for event CH6LIMITL"]
pub type CH6LIMITL_R = crate::BitReader<CH6LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH6LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH6LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6LIMITL_A {
        match self.bits {
            false => CH6LIMITL_A::DISABLED,
            true => CH6LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH6LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH6LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6LIMITL` writer - Write '1' to enable interrupt for event CH6LIMITL"]
pub type CH6LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH6LIMITL_AW, O>;
impl<'a, const O: u8> CH6LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6LIMITL_AW::SET)
    }
}
#[doc = "Field `CH7LIMITH` reader - Write '1' to enable interrupt for event CH7LIMITH"]
pub type CH7LIMITH_R = crate::BitReader<CH7LIMITH_A>;
#[doc = "Write '1' to enable interrupt for event CH7LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH7LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7LIMITH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7LIMITH_A {
        match self.bits {
            false => CH7LIMITH_A::DISABLED,
            true => CH7LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH7LIMITH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH7LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITH` writer - Write '1' to enable interrupt for event CH7LIMITH"]
pub type CH7LIMITH_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH7LIMITH_AW, O>;
impl<'a, const O: u8> CH7LIMITH_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH7LIMITH_AW::SET)
    }
}
#[doc = "Field `CH7LIMITL` reader - Write '1' to enable interrupt for event CH7LIMITL"]
pub type CH7LIMITL_R = crate::BitReader<CH7LIMITL_A>;
#[doc = "Write '1' to enable interrupt for event CH7LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH7LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7LIMITL_A {
        match self.bits {
            false => CH7LIMITL_A::DISABLED,
            true => CH7LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CH7LIMITL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH7LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7LIMITL` writer - Write '1' to enable interrupt for event CH7LIMITL"]
pub type CH7LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CH7LIMITL_AW, O>;
impl<'a, const O: u8> CH7LIMITL_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH7LIMITL_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RESULTDONE"]
    #[inline(always)]
    pub fn resultdone(&self) -> RESULTDONE_R {
        RESULTDONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    pub fn calibratedone(&self) -> CALIBRATEDONE_R {
        CALIBRATEDONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event CH0LIMITH"]
    #[inline(always)]
    pub fn ch0limith(&self) -> CH0LIMITH_R {
        CH0LIMITH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event CH0LIMITL"]
    #[inline(always)]
    pub fn ch0limitl(&self) -> CH0LIMITL_R {
        CH0LIMITL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event CH1LIMITH"]
    #[inline(always)]
    pub fn ch1limith(&self) -> CH1LIMITH_R {
        CH1LIMITH_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event CH1LIMITL"]
    #[inline(always)]
    pub fn ch1limitl(&self) -> CH1LIMITL_R {
        CH1LIMITL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event CH2LIMITH"]
    #[inline(always)]
    pub fn ch2limith(&self) -> CH2LIMITH_R {
        CH2LIMITH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event CH2LIMITL"]
    #[inline(always)]
    pub fn ch2limitl(&self) -> CH2LIMITL_R {
        CH2LIMITL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CH3LIMITH"]
    #[inline(always)]
    pub fn ch3limith(&self) -> CH3LIMITH_R {
        CH3LIMITH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CH3LIMITL"]
    #[inline(always)]
    pub fn ch3limitl(&self) -> CH3LIMITL_R {
        CH3LIMITL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event CH4LIMITH"]
    #[inline(always)]
    pub fn ch4limith(&self) -> CH4LIMITH_R {
        CH4LIMITH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event CH4LIMITL"]
    #[inline(always)]
    pub fn ch4limitl(&self) -> CH4LIMITL_R {
        CH4LIMITL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event CH5LIMITH"]
    #[inline(always)]
    pub fn ch5limith(&self) -> CH5LIMITH_R {
        CH5LIMITH_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event CH5LIMITL"]
    #[inline(always)]
    pub fn ch5limitl(&self) -> CH5LIMITL_R {
        CH5LIMITL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event CH6LIMITH"]
    #[inline(always)]
    pub fn ch6limith(&self) -> CH6LIMITH_R {
        CH6LIMITH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event CH6LIMITL"]
    #[inline(always)]
    pub fn ch6limitl(&self) -> CH6LIMITL_R {
        CH6LIMITL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event CH7LIMITH"]
    #[inline(always)]
    pub fn ch7limith(&self) -> CH7LIMITH_R {
        CH7LIMITH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event CH7LIMITL"]
    #[inline(always)]
    pub fn ch7limitl(&self) -> CH7LIMITL_R {
        CH7LIMITL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event STARTED"]
    #[inline(always)]
    #[must_use]
    pub fn started(&mut self) -> STARTED_W<0> {
        STARTED_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event END"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> END_W<1> {
        END_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<2> {
        DONE_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event RESULTDONE"]
    #[inline(always)]
    #[must_use]
    pub fn resultdone(&mut self) -> RESULTDONE_W<3> {
        RESULTDONE_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CALIBRATEDONE"]
    #[inline(always)]
    #[must_use]
    pub fn calibratedone(&mut self) -> CALIBRATEDONE_W<4> {
        CALIBRATEDONE_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> STOPPED_W<5> {
        STOPPED_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event CH0LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch0limith(&mut self) -> CH0LIMITH_W<6> {
        CH0LIMITH_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event CH0LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch0limitl(&mut self) -> CH0LIMITL_W<7> {
        CH0LIMITL_W::new(self)
    }
    #[doc = "Bit 8 - Write '1' to enable interrupt for event CH1LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch1limith(&mut self) -> CH1LIMITH_W<8> {
        CH1LIMITH_W::new(self)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event CH1LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch1limitl(&mut self) -> CH1LIMITL_W<9> {
        CH1LIMITL_W::new(self)
    }
    #[doc = "Bit 10 - Write '1' to enable interrupt for event CH2LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch2limith(&mut self) -> CH2LIMITH_W<10> {
        CH2LIMITH_W::new(self)
    }
    #[doc = "Bit 11 - Write '1' to enable interrupt for event CH2LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch2limitl(&mut self) -> CH2LIMITL_W<11> {
        CH2LIMITL_W::new(self)
    }
    #[doc = "Bit 12 - Write '1' to enable interrupt for event CH3LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch3limith(&mut self) -> CH3LIMITH_W<12> {
        CH3LIMITH_W::new(self)
    }
    #[doc = "Bit 13 - Write '1' to enable interrupt for event CH3LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch3limitl(&mut self) -> CH3LIMITL_W<13> {
        CH3LIMITL_W::new(self)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event CH4LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch4limith(&mut self) -> CH4LIMITH_W<14> {
        CH4LIMITH_W::new(self)
    }
    #[doc = "Bit 15 - Write '1' to enable interrupt for event CH4LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch4limitl(&mut self) -> CH4LIMITL_W<15> {
        CH4LIMITL_W::new(self)
    }
    #[doc = "Bit 16 - Write '1' to enable interrupt for event CH5LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch5limith(&mut self) -> CH5LIMITH_W<16> {
        CH5LIMITH_W::new(self)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event CH5LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch5limitl(&mut self) -> CH5LIMITL_W<17> {
        CH5LIMITL_W::new(self)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event CH6LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch6limith(&mut self) -> CH6LIMITH_W<18> {
        CH6LIMITH_W::new(self)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event CH6LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch6limitl(&mut self) -> CH6LIMITL_W<19> {
        CH6LIMITL_W::new(self)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event CH7LIMITH"]
    #[inline(always)]
    #[must_use]
    pub fn ch7limith(&mut self) -> CH7LIMITH_W<20> {
        CH7LIMITH_W::new(self)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event CH7LIMITL"]
    #[inline(always)]
    #[must_use]
    pub fn ch7limitl(&mut self) -> CH7LIMITL_W<21> {
        CH7LIMITL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
