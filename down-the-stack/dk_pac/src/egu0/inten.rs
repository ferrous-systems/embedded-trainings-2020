#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGERED0` reader - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
pub type TRIGGERED0_R = crate::BitReader<TRIGGERED0_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED0_A {
        match self.bits {
            false => TRIGGERED0_A::DISABLED,
            true => TRIGGERED0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED0_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED0` writer - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
pub type TRIGGERED0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED0_A, O>;
impl<'a, const O: u8> TRIGGERED0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED0_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED1` reader - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
pub type TRIGGERED1_R = crate::BitReader<TRIGGERED1_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED1_A {
        match self.bits {
            false => TRIGGERED1_A::DISABLED,
            true => TRIGGERED1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED1_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED1` writer - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
pub type TRIGGERED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED1_A, O>;
impl<'a, const O: u8> TRIGGERED1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED1_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED2` reader - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
pub type TRIGGERED2_R = crate::BitReader<TRIGGERED2_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED2_A {
        match self.bits {
            false => TRIGGERED2_A::DISABLED,
            true => TRIGGERED2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED2_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED2` writer - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
pub type TRIGGERED2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED2_A, O>;
impl<'a, const O: u8> TRIGGERED2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED2_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED3` reader - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
pub type TRIGGERED3_R = crate::BitReader<TRIGGERED3_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED3_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED3_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED3_A {
        match self.bits {
            false => TRIGGERED3_A::DISABLED,
            true => TRIGGERED3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED3_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED3` writer - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
pub type TRIGGERED3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED3_A, O>;
impl<'a, const O: u8> TRIGGERED3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED3_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED4` reader - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
pub type TRIGGERED4_R = crate::BitReader<TRIGGERED4_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED4_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED4_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED4_A {
        match self.bits {
            false => TRIGGERED4_A::DISABLED,
            true => TRIGGERED4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED4_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED4` writer - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
pub type TRIGGERED4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED4_A, O>;
impl<'a, const O: u8> TRIGGERED4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED4_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED5` reader - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
pub type TRIGGERED5_R = crate::BitReader<TRIGGERED5_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED5_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED5_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED5_A {
        match self.bits {
            false => TRIGGERED5_A::DISABLED,
            true => TRIGGERED5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED5_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED5` writer - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
pub type TRIGGERED5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED5_A, O>;
impl<'a, const O: u8> TRIGGERED5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED5_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED6` reader - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
pub type TRIGGERED6_R = crate::BitReader<TRIGGERED6_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED6_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED6_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED6_A {
        match self.bits {
            false => TRIGGERED6_A::DISABLED,
            true => TRIGGERED6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED6_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED6` writer - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
pub type TRIGGERED6_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED6_A, O>;
impl<'a, const O: u8> TRIGGERED6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED6_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED7` reader - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
pub type TRIGGERED7_R = crate::BitReader<TRIGGERED7_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED7_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED7_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED7_A {
        match self.bits {
            false => TRIGGERED7_A::DISABLED,
            true => TRIGGERED7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED7_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED7` writer - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
pub type TRIGGERED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED7_A, O>;
impl<'a, const O: u8> TRIGGERED7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED7_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED8` reader - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
pub type TRIGGERED8_R = crate::BitReader<TRIGGERED8_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED8_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED8_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED8_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED8_A {
        match self.bits {
            false => TRIGGERED8_A::DISABLED,
            true => TRIGGERED8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED8_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED8` writer - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
pub type TRIGGERED8_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED8_A, O>;
impl<'a, const O: u8> TRIGGERED8_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED8_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED8_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED9` reader - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
pub type TRIGGERED9_R = crate::BitReader<TRIGGERED9_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED9_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED9_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED9_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED9_A {
        match self.bits {
            false => TRIGGERED9_A::DISABLED,
            true => TRIGGERED9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED9_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED9` writer - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
pub type TRIGGERED9_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED9_A, O>;
impl<'a, const O: u8> TRIGGERED9_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED9_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED9_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED10` reader - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
pub type TRIGGERED10_R = crate::BitReader<TRIGGERED10_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED10_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED10_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED10_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED10_A {
        match self.bits {
            false => TRIGGERED10_A::DISABLED,
            true => TRIGGERED10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED10_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED10` writer - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
pub type TRIGGERED10_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED10_A, O>;
impl<'a, const O: u8> TRIGGERED10_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED10_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED10_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED11` reader - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
pub type TRIGGERED11_R = crate::BitReader<TRIGGERED11_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED11_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED11_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED11_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED11_A {
        match self.bits {
            false => TRIGGERED11_A::DISABLED,
            true => TRIGGERED11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED11_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED11` writer - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
pub type TRIGGERED11_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED11_A, O>;
impl<'a, const O: u8> TRIGGERED11_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED11_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED11_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED12` reader - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
pub type TRIGGERED12_R = crate::BitReader<TRIGGERED12_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED12_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED12_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED12_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED12_A {
        match self.bits {
            false => TRIGGERED12_A::DISABLED,
            true => TRIGGERED12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED12_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED12` writer - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
pub type TRIGGERED12_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED12_A, O>;
impl<'a, const O: u8> TRIGGERED12_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED12_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED12_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED13` reader - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
pub type TRIGGERED13_R = crate::BitReader<TRIGGERED13_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED13_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED13_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED13_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED13_A {
        match self.bits {
            false => TRIGGERED13_A::DISABLED,
            true => TRIGGERED13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED13_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED13` writer - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
pub type TRIGGERED13_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED13_A, O>;
impl<'a, const O: u8> TRIGGERED13_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED13_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED13_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED14` reader - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
pub type TRIGGERED14_R = crate::BitReader<TRIGGERED14_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED14_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED14_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED14_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED14_A {
        match self.bits {
            false => TRIGGERED14_A::DISABLED,
            true => TRIGGERED14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED14_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED14` writer - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
pub type TRIGGERED14_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED14_A, O>;
impl<'a, const O: u8> TRIGGERED14_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED14_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED14_A::ENABLED)
    }
}
#[doc = "Field `TRIGGERED15` reader - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
pub type TRIGGERED15_R = crate::BitReader<TRIGGERED15_A>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIGGERED15_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED15_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED15_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIGGERED15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED15_A {
        match self.bits {
            false => TRIGGERED15_A::DISABLED,
            true => TRIGGERED15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED15_A::ENABLED
    }
}
#[doc = "Field `TRIGGERED15` writer - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
pub type TRIGGERED15_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, TRIGGERED15_A, O>;
impl<'a, const O: u8> TRIGGERED15_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED15_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED15_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    pub fn triggered0(&self) -> TRIGGERED0_R {
        TRIGGERED0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    pub fn triggered1(&self) -> TRIGGERED1_R {
        TRIGGERED1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    pub fn triggered2(&self) -> TRIGGERED2_R {
        TRIGGERED2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    pub fn triggered3(&self) -> TRIGGERED3_R {
        TRIGGERED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    pub fn triggered4(&self) -> TRIGGERED4_R {
        TRIGGERED4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    pub fn triggered5(&self) -> TRIGGERED5_R {
        TRIGGERED5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    pub fn triggered6(&self) -> TRIGGERED6_R {
        TRIGGERED6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    pub fn triggered7(&self) -> TRIGGERED7_R {
        TRIGGERED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    pub fn triggered8(&self) -> TRIGGERED8_R {
        TRIGGERED8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    pub fn triggered9(&self) -> TRIGGERED9_R {
        TRIGGERED9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    pub fn triggered10(&self) -> TRIGGERED10_R {
        TRIGGERED10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    pub fn triggered11(&self) -> TRIGGERED11_R {
        TRIGGERED11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    pub fn triggered12(&self) -> TRIGGERED12_R {
        TRIGGERED12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    pub fn triggered13(&self) -> TRIGGERED13_R {
        TRIGGERED13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    pub fn triggered14(&self) -> TRIGGERED14_R {
        TRIGGERED14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    pub fn triggered15(&self) -> TRIGGERED15_R {
        TRIGGERED15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered0(&mut self) -> TRIGGERED0_W<0> {
        TRIGGERED0_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered1(&mut self) -> TRIGGERED1_W<1> {
        TRIGGERED1_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered2(&mut self) -> TRIGGERED2_W<2> {
        TRIGGERED2_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered3(&mut self) -> TRIGGERED3_W<3> {
        TRIGGERED3_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered4(&mut self) -> TRIGGERED4_W<4> {
        TRIGGERED4_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered5(&mut self) -> TRIGGERED5_W<5> {
        TRIGGERED5_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered6(&mut self) -> TRIGGERED6_W<6> {
        TRIGGERED6_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered7(&mut self) -> TRIGGERED7_W<7> {
        TRIGGERED7_W::new(self)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered8(&mut self) -> TRIGGERED8_W<8> {
        TRIGGERED8_W::new(self)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered9(&mut self) -> TRIGGERED9_W<9> {
        TRIGGERED9_W::new(self)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered10(&mut self) -> TRIGGERED10_W<10> {
        TRIGGERED10_W::new(self)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered11(&mut self) -> TRIGGERED11_W<11> {
        TRIGGERED11_W::new(self)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered12(&mut self) -> TRIGGERED12_W<12> {
        TRIGGERED12_W::new(self)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered13(&mut self) -> TRIGGERED13_W<13> {
        TRIGGERED13_W::new(self)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered14(&mut self) -> TRIGGERED14_W<14> {
        TRIGGERED14_W::new(self)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    #[must_use]
    pub fn triggered15(&mut self) -> TRIGGERED15_W<15> {
        TRIGGERED15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
