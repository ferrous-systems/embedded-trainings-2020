#[doc = "Register `CHENCLR` reader"]
pub struct R(crate::R<CHENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHENCLR` writer"]
pub struct W(crate::W<CHENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENCLR_SPEC>;
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
impl From<crate::W<CHENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Channel 0 enable clear register. Writing '0' has no effect"]
pub type CH0_R = crate::BitReader<CH0_A>;
#[doc = "Channel 0 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
impl CH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::DISABLED,
            true => CH0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0_A::ENABLED
    }
}
#[doc = "Channel 0 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH0_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Channel 0 enable clear register. Writing '0' has no effect"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH0_AW, O>;
impl<'a, const O: u8> CH0_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH0_AW::CLEAR)
    }
}
#[doc = "Field `CH1` reader - Channel 1 enable clear register. Writing '0' has no effect"]
pub type CH1_R = crate::BitReader<CH1_A>;
#[doc = "Channel 1 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::DISABLED,
            true => CH1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1_A::ENABLED
    }
}
#[doc = "Channel 1 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH1_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Channel 1 enable clear register. Writing '0' has no effect"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH1_AW, O>;
impl<'a, const O: u8> CH1_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH1_AW::CLEAR)
    }
}
#[doc = "Field `CH2` reader - Channel 2 enable clear register. Writing '0' has no effect"]
pub type CH2_R = crate::BitReader<CH2_A>;
#[doc = "Channel 2 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
impl CH2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::DISABLED,
            true => CH2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2_A::ENABLED
    }
}
#[doc = "Channel 2 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH2_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Channel 2 enable clear register. Writing '0' has no effect"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH2_AW, O>;
impl<'a, const O: u8> CH2_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH2_AW::CLEAR)
    }
}
#[doc = "Field `CH3` reader - Channel 3 enable clear register. Writing '0' has no effect"]
pub type CH3_R = crate::BitReader<CH3_A>;
#[doc = "Channel 3 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
impl CH3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::DISABLED,
            true => CH3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3_A::ENABLED
    }
}
#[doc = "Channel 3 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH3_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Channel 3 enable clear register. Writing '0' has no effect"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH3_AW, O>;
impl<'a, const O: u8> CH3_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH3_AW::CLEAR)
    }
}
#[doc = "Field `CH4` reader - Channel 4 enable clear register. Writing '0' has no effect"]
pub type CH4_R = crate::BitReader<CH4_A>;
#[doc = "Channel 4 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH4_A> for bool {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as u8 != 0
    }
}
impl CH4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4_A {
        match self.bits {
            false => CH4_A::DISABLED,
            true => CH4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4_A::ENABLED
    }
}
#[doc = "Channel 4 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH4_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` writer - Channel 4 enable clear register. Writing '0' has no effect"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH4_AW, O>;
impl<'a, const O: u8> CH4_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH4_AW::CLEAR)
    }
}
#[doc = "Field `CH5` reader - Channel 5 enable clear register. Writing '0' has no effect"]
pub type CH5_R = crate::BitReader<CH5_A>;
#[doc = "Channel 5 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH5_A> for bool {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as u8 != 0
    }
}
impl CH5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5_A {
        match self.bits {
            false => CH5_A::DISABLED,
            true => CH5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5_A::ENABLED
    }
}
#[doc = "Channel 5 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH5_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` writer - Channel 5 enable clear register. Writing '0' has no effect"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH5_AW, O>;
impl<'a, const O: u8> CH5_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH5_AW::CLEAR)
    }
}
#[doc = "Field `CH6` reader - Channel 6 enable clear register. Writing '0' has no effect"]
pub type CH6_R = crate::BitReader<CH6_A>;
#[doc = "Channel 6 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH6_A> for bool {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as u8 != 0
    }
}
impl CH6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6_A {
        match self.bits {
            false => CH6_A::DISABLED,
            true => CH6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6_A::ENABLED
    }
}
#[doc = "Channel 6 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH6_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` writer - Channel 6 enable clear register. Writing '0' has no effect"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH6_AW, O>;
impl<'a, const O: u8> CH6_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH6_AW::CLEAR)
    }
}
#[doc = "Field `CH7` reader - Channel 7 enable clear register. Writing '0' has no effect"]
pub type CH7_R = crate::BitReader<CH7_A>;
#[doc = "Channel 7 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH7_A> for bool {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as u8 != 0
    }
}
impl CH7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7_A {
        match self.bits {
            false => CH7_A::DISABLED,
            true => CH7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7_A::ENABLED
    }
}
#[doc = "Channel 7 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH7_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` writer - Channel 7 enable clear register. Writing '0' has no effect"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH7_AW, O>;
impl<'a, const O: u8> CH7_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH7_AW::CLEAR)
    }
}
#[doc = "Field `CH8` reader - Channel 8 enable clear register. Writing '0' has no effect"]
pub type CH8_R = crate::BitReader<CH8_A>;
#[doc = "Channel 8 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH8_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH8_A> for bool {
    #[inline(always)]
    fn from(variant: CH8_A) -> Self {
        variant as u8 != 0
    }
}
impl CH8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH8_A {
        match self.bits {
            false => CH8_A::DISABLED,
            true => CH8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH8_A::ENABLED
    }
}
#[doc = "Channel 8 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH8_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH8_AW> for bool {
    #[inline(always)]
    fn from(variant: CH8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` writer - Channel 8 enable clear register. Writing '0' has no effect"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH8_AW, O>;
impl<'a, const O: u8> CH8_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH8_AW::CLEAR)
    }
}
#[doc = "Field `CH9` reader - Channel 9 enable clear register. Writing '0' has no effect"]
pub type CH9_R = crate::BitReader<CH9_A>;
#[doc = "Channel 9 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH9_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH9_A> for bool {
    #[inline(always)]
    fn from(variant: CH9_A) -> Self {
        variant as u8 != 0
    }
}
impl CH9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH9_A {
        match self.bits {
            false => CH9_A::DISABLED,
            true => CH9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH9_A::ENABLED
    }
}
#[doc = "Channel 9 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH9_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH9_AW> for bool {
    #[inline(always)]
    fn from(variant: CH9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` writer - Channel 9 enable clear register. Writing '0' has no effect"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH9_AW, O>;
impl<'a, const O: u8> CH9_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH9_AW::CLEAR)
    }
}
#[doc = "Field `CH10` reader - Channel 10 enable clear register. Writing '0' has no effect"]
pub type CH10_R = crate::BitReader<CH10_A>;
#[doc = "Channel 10 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH10_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH10_A> for bool {
    #[inline(always)]
    fn from(variant: CH10_A) -> Self {
        variant as u8 != 0
    }
}
impl CH10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH10_A {
        match self.bits {
            false => CH10_A::DISABLED,
            true => CH10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH10_A::ENABLED
    }
}
#[doc = "Channel 10 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH10_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH10_AW> for bool {
    #[inline(always)]
    fn from(variant: CH10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` writer - Channel 10 enable clear register. Writing '0' has no effect"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH10_AW, O>;
impl<'a, const O: u8> CH10_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH10_AW::CLEAR)
    }
}
#[doc = "Field `CH11` reader - Channel 11 enable clear register. Writing '0' has no effect"]
pub type CH11_R = crate::BitReader<CH11_A>;
#[doc = "Channel 11 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH11_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH11_A> for bool {
    #[inline(always)]
    fn from(variant: CH11_A) -> Self {
        variant as u8 != 0
    }
}
impl CH11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH11_A {
        match self.bits {
            false => CH11_A::DISABLED,
            true => CH11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH11_A::ENABLED
    }
}
#[doc = "Channel 11 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH11_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH11_AW> for bool {
    #[inline(always)]
    fn from(variant: CH11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` writer - Channel 11 enable clear register. Writing '0' has no effect"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH11_AW, O>;
impl<'a, const O: u8> CH11_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH11_AW::CLEAR)
    }
}
#[doc = "Field `CH12` reader - Channel 12 enable clear register. Writing '0' has no effect"]
pub type CH12_R = crate::BitReader<CH12_A>;
#[doc = "Channel 12 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH12_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH12_A> for bool {
    #[inline(always)]
    fn from(variant: CH12_A) -> Self {
        variant as u8 != 0
    }
}
impl CH12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH12_A {
        match self.bits {
            false => CH12_A::DISABLED,
            true => CH12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH12_A::ENABLED
    }
}
#[doc = "Channel 12 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH12_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH12_AW> for bool {
    #[inline(always)]
    fn from(variant: CH12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` writer - Channel 12 enable clear register. Writing '0' has no effect"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH12_AW, O>;
impl<'a, const O: u8> CH12_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH12_AW::CLEAR)
    }
}
#[doc = "Field `CH13` reader - Channel 13 enable clear register. Writing '0' has no effect"]
pub type CH13_R = crate::BitReader<CH13_A>;
#[doc = "Channel 13 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH13_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH13_A> for bool {
    #[inline(always)]
    fn from(variant: CH13_A) -> Self {
        variant as u8 != 0
    }
}
impl CH13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH13_A {
        match self.bits {
            false => CH13_A::DISABLED,
            true => CH13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH13_A::ENABLED
    }
}
#[doc = "Channel 13 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH13_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH13_AW> for bool {
    #[inline(always)]
    fn from(variant: CH13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` writer - Channel 13 enable clear register. Writing '0' has no effect"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH13_AW, O>;
impl<'a, const O: u8> CH13_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH13_AW::CLEAR)
    }
}
#[doc = "Field `CH14` reader - Channel 14 enable clear register. Writing '0' has no effect"]
pub type CH14_R = crate::BitReader<CH14_A>;
#[doc = "Channel 14 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH14_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH14_A> for bool {
    #[inline(always)]
    fn from(variant: CH14_A) -> Self {
        variant as u8 != 0
    }
}
impl CH14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH14_A {
        match self.bits {
            false => CH14_A::DISABLED,
            true => CH14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH14_A::ENABLED
    }
}
#[doc = "Channel 14 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH14_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH14_AW> for bool {
    #[inline(always)]
    fn from(variant: CH14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` writer - Channel 14 enable clear register. Writing '0' has no effect"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH14_AW, O>;
impl<'a, const O: u8> CH14_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH14_AW::CLEAR)
    }
}
#[doc = "Field `CH15` reader - Channel 15 enable clear register. Writing '0' has no effect"]
pub type CH15_R = crate::BitReader<CH15_A>;
#[doc = "Channel 15 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH15_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH15_A> for bool {
    #[inline(always)]
    fn from(variant: CH15_A) -> Self {
        variant as u8 != 0
    }
}
impl CH15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH15_A {
        match self.bits {
            false => CH15_A::DISABLED,
            true => CH15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH15_A::ENABLED
    }
}
#[doc = "Channel 15 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH15_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH15_AW> for bool {
    #[inline(always)]
    fn from(variant: CH15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` writer - Channel 15 enable clear register. Writing '0' has no effect"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH15_AW, O>;
impl<'a, const O: u8> CH15_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH15_AW::CLEAR)
    }
}
#[doc = "Field `CH16` reader - Channel 16 enable clear register. Writing '0' has no effect"]
pub type CH16_R = crate::BitReader<CH16_A>;
#[doc = "Channel 16 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH16_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH16_A> for bool {
    #[inline(always)]
    fn from(variant: CH16_A) -> Self {
        variant as u8 != 0
    }
}
impl CH16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH16_A {
        match self.bits {
            false => CH16_A::DISABLED,
            true => CH16_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH16_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH16_A::ENABLED
    }
}
#[doc = "Channel 16 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH16_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH16_AW> for bool {
    #[inline(always)]
    fn from(variant: CH16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH16` writer - Channel 16 enable clear register. Writing '0' has no effect"]
pub type CH16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH16_AW, O>;
impl<'a, const O: u8> CH16_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH16_AW::CLEAR)
    }
}
#[doc = "Field `CH17` reader - Channel 17 enable clear register. Writing '0' has no effect"]
pub type CH17_R = crate::BitReader<CH17_A>;
#[doc = "Channel 17 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH17_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH17_A> for bool {
    #[inline(always)]
    fn from(variant: CH17_A) -> Self {
        variant as u8 != 0
    }
}
impl CH17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH17_A {
        match self.bits {
            false => CH17_A::DISABLED,
            true => CH17_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH17_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH17_A::ENABLED
    }
}
#[doc = "Channel 17 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH17_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH17_AW> for bool {
    #[inline(always)]
    fn from(variant: CH17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH17` writer - Channel 17 enable clear register. Writing '0' has no effect"]
pub type CH17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH17_AW, O>;
impl<'a, const O: u8> CH17_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH17_AW::CLEAR)
    }
}
#[doc = "Field `CH18` reader - Channel 18 enable clear register. Writing '0' has no effect"]
pub type CH18_R = crate::BitReader<CH18_A>;
#[doc = "Channel 18 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH18_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH18_A> for bool {
    #[inline(always)]
    fn from(variant: CH18_A) -> Self {
        variant as u8 != 0
    }
}
impl CH18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH18_A {
        match self.bits {
            false => CH18_A::DISABLED,
            true => CH18_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH18_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH18_A::ENABLED
    }
}
#[doc = "Channel 18 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH18_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH18_AW> for bool {
    #[inline(always)]
    fn from(variant: CH18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH18` writer - Channel 18 enable clear register. Writing '0' has no effect"]
pub type CH18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH18_AW, O>;
impl<'a, const O: u8> CH18_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH18_AW::CLEAR)
    }
}
#[doc = "Field `CH19` reader - Channel 19 enable clear register. Writing '0' has no effect"]
pub type CH19_R = crate::BitReader<CH19_A>;
#[doc = "Channel 19 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH19_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH19_A> for bool {
    #[inline(always)]
    fn from(variant: CH19_A) -> Self {
        variant as u8 != 0
    }
}
impl CH19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH19_A {
        match self.bits {
            false => CH19_A::DISABLED,
            true => CH19_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH19_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH19_A::ENABLED
    }
}
#[doc = "Channel 19 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH19_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH19_AW> for bool {
    #[inline(always)]
    fn from(variant: CH19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH19` writer - Channel 19 enable clear register. Writing '0' has no effect"]
pub type CH19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH19_AW, O>;
impl<'a, const O: u8> CH19_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH19_AW::CLEAR)
    }
}
#[doc = "Field `CH20` reader - Channel 20 enable clear register. Writing '0' has no effect"]
pub type CH20_R = crate::BitReader<CH20_A>;
#[doc = "Channel 20 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH20_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH20_A> for bool {
    #[inline(always)]
    fn from(variant: CH20_A) -> Self {
        variant as u8 != 0
    }
}
impl CH20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH20_A {
        match self.bits {
            false => CH20_A::DISABLED,
            true => CH20_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH20_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH20_A::ENABLED
    }
}
#[doc = "Channel 20 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH20_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH20_AW> for bool {
    #[inline(always)]
    fn from(variant: CH20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` writer - Channel 20 enable clear register. Writing '0' has no effect"]
pub type CH20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH20_AW, O>;
impl<'a, const O: u8> CH20_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH20_AW::CLEAR)
    }
}
#[doc = "Field `CH21` reader - Channel 21 enable clear register. Writing '0' has no effect"]
pub type CH21_R = crate::BitReader<CH21_A>;
#[doc = "Channel 21 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH21_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH21_A> for bool {
    #[inline(always)]
    fn from(variant: CH21_A) -> Self {
        variant as u8 != 0
    }
}
impl CH21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH21_A {
        match self.bits {
            false => CH21_A::DISABLED,
            true => CH21_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH21_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH21_A::ENABLED
    }
}
#[doc = "Channel 21 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH21_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH21_AW> for bool {
    #[inline(always)]
    fn from(variant: CH21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` writer - Channel 21 enable clear register. Writing '0' has no effect"]
pub type CH21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH21_AW, O>;
impl<'a, const O: u8> CH21_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH21_AW::CLEAR)
    }
}
#[doc = "Field `CH22` reader - Channel 22 enable clear register. Writing '0' has no effect"]
pub type CH22_R = crate::BitReader<CH22_A>;
#[doc = "Channel 22 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH22_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH22_A> for bool {
    #[inline(always)]
    fn from(variant: CH22_A) -> Self {
        variant as u8 != 0
    }
}
impl CH22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH22_A {
        match self.bits {
            false => CH22_A::DISABLED,
            true => CH22_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH22_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH22_A::ENABLED
    }
}
#[doc = "Channel 22 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH22_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH22_AW> for bool {
    #[inline(always)]
    fn from(variant: CH22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` writer - Channel 22 enable clear register. Writing '0' has no effect"]
pub type CH22_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH22_AW, O>;
impl<'a, const O: u8> CH22_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH22_AW::CLEAR)
    }
}
#[doc = "Field `CH23` reader - Channel 23 enable clear register. Writing '0' has no effect"]
pub type CH23_R = crate::BitReader<CH23_A>;
#[doc = "Channel 23 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH23_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH23_A> for bool {
    #[inline(always)]
    fn from(variant: CH23_A) -> Self {
        variant as u8 != 0
    }
}
impl CH23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH23_A {
        match self.bits {
            false => CH23_A::DISABLED,
            true => CH23_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH23_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH23_A::ENABLED
    }
}
#[doc = "Channel 23 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH23_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH23_AW> for bool {
    #[inline(always)]
    fn from(variant: CH23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` writer - Channel 23 enable clear register. Writing '0' has no effect"]
pub type CH23_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH23_AW, O>;
impl<'a, const O: u8> CH23_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH23_AW::CLEAR)
    }
}
#[doc = "Field `CH24` reader - Channel 24 enable clear register. Writing '0' has no effect"]
pub type CH24_R = crate::BitReader<CH24_A>;
#[doc = "Channel 24 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH24_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH24_A> for bool {
    #[inline(always)]
    fn from(variant: CH24_A) -> Self {
        variant as u8 != 0
    }
}
impl CH24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH24_A {
        match self.bits {
            false => CH24_A::DISABLED,
            true => CH24_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH24_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH24_A::ENABLED
    }
}
#[doc = "Channel 24 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH24_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH24_AW> for bool {
    #[inline(always)]
    fn from(variant: CH24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH24` writer - Channel 24 enable clear register. Writing '0' has no effect"]
pub type CH24_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH24_AW, O>;
impl<'a, const O: u8> CH24_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH24_AW::CLEAR)
    }
}
#[doc = "Field `CH25` reader - Channel 25 enable clear register. Writing '0' has no effect"]
pub type CH25_R = crate::BitReader<CH25_A>;
#[doc = "Channel 25 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH25_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH25_A> for bool {
    #[inline(always)]
    fn from(variant: CH25_A) -> Self {
        variant as u8 != 0
    }
}
impl CH25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH25_A {
        match self.bits {
            false => CH25_A::DISABLED,
            true => CH25_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH25_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH25_A::ENABLED
    }
}
#[doc = "Channel 25 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH25_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH25_AW> for bool {
    #[inline(always)]
    fn from(variant: CH25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH25` writer - Channel 25 enable clear register. Writing '0' has no effect"]
pub type CH25_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH25_AW, O>;
impl<'a, const O: u8> CH25_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH25_AW::CLEAR)
    }
}
#[doc = "Field `CH26` reader - Channel 26 enable clear register. Writing '0' has no effect"]
pub type CH26_R = crate::BitReader<CH26_A>;
#[doc = "Channel 26 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH26_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH26_A> for bool {
    #[inline(always)]
    fn from(variant: CH26_A) -> Self {
        variant as u8 != 0
    }
}
impl CH26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH26_A {
        match self.bits {
            false => CH26_A::DISABLED,
            true => CH26_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH26_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH26_A::ENABLED
    }
}
#[doc = "Channel 26 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH26_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH26_AW> for bool {
    #[inline(always)]
    fn from(variant: CH26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH26` writer - Channel 26 enable clear register. Writing '0' has no effect"]
pub type CH26_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH26_AW, O>;
impl<'a, const O: u8> CH26_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH26_AW::CLEAR)
    }
}
#[doc = "Field `CH27` reader - Channel 27 enable clear register. Writing '0' has no effect"]
pub type CH27_R = crate::BitReader<CH27_A>;
#[doc = "Channel 27 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH27_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH27_A> for bool {
    #[inline(always)]
    fn from(variant: CH27_A) -> Self {
        variant as u8 != 0
    }
}
impl CH27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH27_A {
        match self.bits {
            false => CH27_A::DISABLED,
            true => CH27_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH27_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH27_A::ENABLED
    }
}
#[doc = "Channel 27 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH27_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH27_AW> for bool {
    #[inline(always)]
    fn from(variant: CH27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH27` writer - Channel 27 enable clear register. Writing '0' has no effect"]
pub type CH27_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH27_AW, O>;
impl<'a, const O: u8> CH27_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH27_AW::CLEAR)
    }
}
#[doc = "Field `CH28` reader - Channel 28 enable clear register. Writing '0' has no effect"]
pub type CH28_R = crate::BitReader<CH28_A>;
#[doc = "Channel 28 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH28_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH28_A> for bool {
    #[inline(always)]
    fn from(variant: CH28_A) -> Self {
        variant as u8 != 0
    }
}
impl CH28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH28_A {
        match self.bits {
            false => CH28_A::DISABLED,
            true => CH28_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH28_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH28_A::ENABLED
    }
}
#[doc = "Channel 28 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH28_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH28_AW> for bool {
    #[inline(always)]
    fn from(variant: CH28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH28` writer - Channel 28 enable clear register. Writing '0' has no effect"]
pub type CH28_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH28_AW, O>;
impl<'a, const O: u8> CH28_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH28_AW::CLEAR)
    }
}
#[doc = "Field `CH29` reader - Channel 29 enable clear register. Writing '0' has no effect"]
pub type CH29_R = crate::BitReader<CH29_A>;
#[doc = "Channel 29 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH29_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH29_A> for bool {
    #[inline(always)]
    fn from(variant: CH29_A) -> Self {
        variant as u8 != 0
    }
}
impl CH29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH29_A {
        match self.bits {
            false => CH29_A::DISABLED,
            true => CH29_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH29_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH29_A::ENABLED
    }
}
#[doc = "Channel 29 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH29_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH29_AW> for bool {
    #[inline(always)]
    fn from(variant: CH29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH29` writer - Channel 29 enable clear register. Writing '0' has no effect"]
pub type CH29_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH29_AW, O>;
impl<'a, const O: u8> CH29_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH29_AW::CLEAR)
    }
}
#[doc = "Field `CH30` reader - Channel 30 enable clear register. Writing '0' has no effect"]
pub type CH30_R = crate::BitReader<CH30_A>;
#[doc = "Channel 30 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH30_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH30_A> for bool {
    #[inline(always)]
    fn from(variant: CH30_A) -> Self {
        variant as u8 != 0
    }
}
impl CH30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH30_A {
        match self.bits {
            false => CH30_A::DISABLED,
            true => CH30_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH30_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH30_A::ENABLED
    }
}
#[doc = "Channel 30 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH30_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH30_AW> for bool {
    #[inline(always)]
    fn from(variant: CH30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH30` writer - Channel 30 enable clear register. Writing '0' has no effect"]
pub type CH30_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH30_AW, O>;
impl<'a, const O: u8> CH30_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH30_AW::CLEAR)
    }
}
#[doc = "Field `CH31` reader - Channel 31 enable clear register. Writing '0' has no effect"]
pub type CH31_R = crate::BitReader<CH31_A>;
#[doc = "Channel 31 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH31_A {
    #[doc = "0: Read: channel disabled"]
    DISABLED = 0,
    #[doc = "1: Read: channel enabled"]
    ENABLED = 1,
}
impl From<CH31_A> for bool {
    #[inline(always)]
    fn from(variant: CH31_A) -> Self {
        variant as u8 != 0
    }
}
impl CH31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH31_A {
        match self.bits {
            false => CH31_A::DISABLED,
            true => CH31_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH31_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH31_A::ENABLED
    }
}
#[doc = "Channel 31 enable clear register. Writing '0' has no effect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CH31_AW {
    #[doc = "1: Write: disable channel"]
    CLEAR = 1,
}
impl From<CH31_AW> for bool {
    #[inline(always)]
    fn from(variant: CH31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH31` writer - Channel 31 enable clear register. Writing '0' has no effect"]
pub type CH31_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CHENCLR_SPEC, CH31_AW, O>;
impl<'a, const O: u8> CH31_W<'a, O> {
    #[doc = "Write: disable channel"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CH31_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 8 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 9 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 10 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 11 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 12 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 13 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 14 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 15 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 16 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch16(&self) -> CH16_R {
        CH16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 17 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch17(&self) -> CH17_R {
        CH17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 18 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch18(&self) -> CH18_R {
        CH18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 19 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch19(&self) -> CH19_R {
        CH19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 20 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 21 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 22 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 23 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 24 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 25 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 26 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 27 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Channel 28 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel 29 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel 30 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel 31 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Channel 8 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Channel 9 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Channel 10 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Channel 11 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Channel 12 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Channel 13 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Channel 14 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Channel 15 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Bit 16 - Channel 16 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch16(&mut self) -> CH16_W<16> {
        CH16_W::new(self)
    }
    #[doc = "Bit 17 - Channel 17 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch17(&mut self) -> CH17_W<17> {
        CH17_W::new(self)
    }
    #[doc = "Bit 18 - Channel 18 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch18(&mut self) -> CH18_W<18> {
        CH18_W::new(self)
    }
    #[doc = "Bit 19 - Channel 19 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch19(&mut self) -> CH19_W<19> {
        CH19_W::new(self)
    }
    #[doc = "Bit 20 - Channel 20 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch20(&mut self) -> CH20_W<20> {
        CH20_W::new(self)
    }
    #[doc = "Bit 21 - Channel 21 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch21(&mut self) -> CH21_W<21> {
        CH21_W::new(self)
    }
    #[doc = "Bit 22 - Channel 22 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch22(&mut self) -> CH22_W<22> {
        CH22_W::new(self)
    }
    #[doc = "Bit 23 - Channel 23 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch23(&mut self) -> CH23_W<23> {
        CH23_W::new(self)
    }
    #[doc = "Bit 24 - Channel 24 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch24(&mut self) -> CH24_W<24> {
        CH24_W::new(self)
    }
    #[doc = "Bit 25 - Channel 25 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch25(&mut self) -> CH25_W<25> {
        CH25_W::new(self)
    }
    #[doc = "Bit 26 - Channel 26 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch26(&mut self) -> CH26_W<26> {
        CH26_W::new(self)
    }
    #[doc = "Bit 27 - Channel 27 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch27(&mut self) -> CH27_W<27> {
        CH27_W::new(self)
    }
    #[doc = "Bit 28 - Channel 28 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch28(&mut self) -> CH28_W<28> {
        CH28_W::new(self)
    }
    #[doc = "Bit 29 - Channel 29 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch29(&mut self) -> CH29_W<29> {
        CH29_W::new(self)
    }
    #[doc = "Bit 30 - Channel 30 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch30(&mut self) -> CH30_W<30> {
        CH30_W::new(self)
    }
    #[doc = "Bit 31 - Channel 31 enable clear register. Writing '0' has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn ch31(&mut self) -> CH31_W<31> {
        CH31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel enable clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenclr](index.html) module"]
pub struct CHENCLR_SPEC;
impl crate::RegisterSpec for CHENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chenclr::R](R) reader structure"]
impl crate::Readable for CHENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chenclr::W](W) writer structure"]
impl crate::Writable for CHENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets CHENCLR to value 0"]
impl crate::Resettable for CHENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
