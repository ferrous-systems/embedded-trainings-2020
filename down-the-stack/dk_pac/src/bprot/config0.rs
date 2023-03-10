#[doc = "Register `CONFIG0` reader"]
pub struct R(crate::R<CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG0` writer"]
pub struct W(crate::W<CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG0_SPEC>;
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
impl From<crate::W<CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION0` reader - Enable protection for region 0. Write '0' has no effect."]
pub type REGION0_R = crate::BitReader<REGION0_A>;
#[doc = "Enable protection for region 0. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION0_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION0_A> for bool {
    #[inline(always)]
    fn from(variant: REGION0_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION0_A {
        match self.bits {
            false => REGION0_A::DISABLED,
            true => REGION0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION0_A::ENABLED
    }
}
#[doc = "Field `REGION0` writer - Enable protection for region 0. Write '0' has no effect."]
pub type REGION0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION0_A, O>;
impl<'a, const O: u8> REGION0_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION0_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION0_A::ENABLED)
    }
}
#[doc = "Field `REGION1` reader - Enable protection for region 1. Write '0' has no effect."]
pub type REGION1_R = crate::BitReader<REGION1_A>;
#[doc = "Enable protection for region 1. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION1_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION1_A> for bool {
    #[inline(always)]
    fn from(variant: REGION1_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION1_A {
        match self.bits {
            false => REGION1_A::DISABLED,
            true => REGION1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION1_A::ENABLED
    }
}
#[doc = "Field `REGION1` writer - Enable protection for region 1. Write '0' has no effect."]
pub type REGION1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION1_A, O>;
impl<'a, const O: u8> REGION1_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION1_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION1_A::ENABLED)
    }
}
#[doc = "Field `REGION2` reader - Enable protection for region 2. Write '0' has no effect."]
pub type REGION2_R = crate::BitReader<REGION2_A>;
#[doc = "Enable protection for region 2. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION2_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION2_A> for bool {
    #[inline(always)]
    fn from(variant: REGION2_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION2_A {
        match self.bits {
            false => REGION2_A::DISABLED,
            true => REGION2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION2_A::ENABLED
    }
}
#[doc = "Field `REGION2` writer - Enable protection for region 2. Write '0' has no effect."]
pub type REGION2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION2_A, O>;
impl<'a, const O: u8> REGION2_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION2_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION2_A::ENABLED)
    }
}
#[doc = "Field `REGION3` reader - Enable protection for region 3. Write '0' has no effect."]
pub type REGION3_R = crate::BitReader<REGION3_A>;
#[doc = "Enable protection for region 3. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION3_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION3_A> for bool {
    #[inline(always)]
    fn from(variant: REGION3_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION3_A {
        match self.bits {
            false => REGION3_A::DISABLED,
            true => REGION3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION3_A::ENABLED
    }
}
#[doc = "Field `REGION3` writer - Enable protection for region 3. Write '0' has no effect."]
pub type REGION3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION3_A, O>;
impl<'a, const O: u8> REGION3_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION3_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION3_A::ENABLED)
    }
}
#[doc = "Field `REGION4` reader - Enable protection for region 4. Write '0' has no effect."]
pub type REGION4_R = crate::BitReader<REGION4_A>;
#[doc = "Enable protection for region 4. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION4_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION4_A> for bool {
    #[inline(always)]
    fn from(variant: REGION4_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION4_A {
        match self.bits {
            false => REGION4_A::DISABLED,
            true => REGION4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION4_A::ENABLED
    }
}
#[doc = "Field `REGION4` writer - Enable protection for region 4. Write '0' has no effect."]
pub type REGION4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION4_A, O>;
impl<'a, const O: u8> REGION4_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION4_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION4_A::ENABLED)
    }
}
#[doc = "Field `REGION5` reader - Enable protection for region 5. Write '0' has no effect."]
pub type REGION5_R = crate::BitReader<REGION5_A>;
#[doc = "Enable protection for region 5. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION5_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION5_A> for bool {
    #[inline(always)]
    fn from(variant: REGION5_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION5_A {
        match self.bits {
            false => REGION5_A::DISABLED,
            true => REGION5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION5_A::ENABLED
    }
}
#[doc = "Field `REGION5` writer - Enable protection for region 5. Write '0' has no effect."]
pub type REGION5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION5_A, O>;
impl<'a, const O: u8> REGION5_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION5_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION5_A::ENABLED)
    }
}
#[doc = "Field `REGION6` reader - Enable protection for region 6. Write '0' has no effect."]
pub type REGION6_R = crate::BitReader<REGION6_A>;
#[doc = "Enable protection for region 6. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION6_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION6_A> for bool {
    #[inline(always)]
    fn from(variant: REGION6_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION6_A {
        match self.bits {
            false => REGION6_A::DISABLED,
            true => REGION6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION6_A::ENABLED
    }
}
#[doc = "Field `REGION6` writer - Enable protection for region 6. Write '0' has no effect."]
pub type REGION6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION6_A, O>;
impl<'a, const O: u8> REGION6_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION6_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION6_A::ENABLED)
    }
}
#[doc = "Field `REGION7` reader - Enable protection for region 7. Write '0' has no effect."]
pub type REGION7_R = crate::BitReader<REGION7_A>;
#[doc = "Enable protection for region 7. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION7_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION7_A> for bool {
    #[inline(always)]
    fn from(variant: REGION7_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION7_A {
        match self.bits {
            false => REGION7_A::DISABLED,
            true => REGION7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION7_A::ENABLED
    }
}
#[doc = "Field `REGION7` writer - Enable protection for region 7. Write '0' has no effect."]
pub type REGION7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION7_A, O>;
impl<'a, const O: u8> REGION7_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION7_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION7_A::ENABLED)
    }
}
#[doc = "Field `REGION8` reader - Enable protection for region 8. Write '0' has no effect."]
pub type REGION8_R = crate::BitReader<REGION8_A>;
#[doc = "Enable protection for region 8. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION8_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION8_A> for bool {
    #[inline(always)]
    fn from(variant: REGION8_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION8_A {
        match self.bits {
            false => REGION8_A::DISABLED,
            true => REGION8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION8_A::ENABLED
    }
}
#[doc = "Field `REGION8` writer - Enable protection for region 8. Write '0' has no effect."]
pub type REGION8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION8_A, O>;
impl<'a, const O: u8> REGION8_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION8_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION8_A::ENABLED)
    }
}
#[doc = "Field `REGION9` reader - Enable protection for region 9. Write '0' has no effect."]
pub type REGION9_R = crate::BitReader<REGION9_A>;
#[doc = "Enable protection for region 9. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION9_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION9_A> for bool {
    #[inline(always)]
    fn from(variant: REGION9_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION9_A {
        match self.bits {
            false => REGION9_A::DISABLED,
            true => REGION9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION9_A::ENABLED
    }
}
#[doc = "Field `REGION9` writer - Enable protection for region 9. Write '0' has no effect."]
pub type REGION9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION9_A, O>;
impl<'a, const O: u8> REGION9_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION9_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION9_A::ENABLED)
    }
}
#[doc = "Field `REGION10` reader - Enable protection for region 10. Write '0' has no effect."]
pub type REGION10_R = crate::BitReader<REGION10_A>;
#[doc = "Enable protection for region 10. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION10_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION10_A> for bool {
    #[inline(always)]
    fn from(variant: REGION10_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION10_A {
        match self.bits {
            false => REGION10_A::DISABLED,
            true => REGION10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION10_A::ENABLED
    }
}
#[doc = "Field `REGION10` writer - Enable protection for region 10. Write '0' has no effect."]
pub type REGION10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION10_A, O>;
impl<'a, const O: u8> REGION10_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION10_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION10_A::ENABLED)
    }
}
#[doc = "Field `REGION11` reader - Enable protection for region 11. Write '0' has no effect."]
pub type REGION11_R = crate::BitReader<REGION11_A>;
#[doc = "Enable protection for region 11. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION11_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION11_A> for bool {
    #[inline(always)]
    fn from(variant: REGION11_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION11_A {
        match self.bits {
            false => REGION11_A::DISABLED,
            true => REGION11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION11_A::ENABLED
    }
}
#[doc = "Field `REGION11` writer - Enable protection for region 11. Write '0' has no effect."]
pub type REGION11_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION11_A, O>;
impl<'a, const O: u8> REGION11_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION11_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION11_A::ENABLED)
    }
}
#[doc = "Field `REGION12` reader - Enable protection for region 12. Write '0' has no effect."]
pub type REGION12_R = crate::BitReader<REGION12_A>;
#[doc = "Enable protection for region 12. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION12_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION12_A> for bool {
    #[inline(always)]
    fn from(variant: REGION12_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION12_A {
        match self.bits {
            false => REGION12_A::DISABLED,
            true => REGION12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION12_A::ENABLED
    }
}
#[doc = "Field `REGION12` writer - Enable protection for region 12. Write '0' has no effect."]
pub type REGION12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION12_A, O>;
impl<'a, const O: u8> REGION12_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION12_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION12_A::ENABLED)
    }
}
#[doc = "Field `REGION13` reader - Enable protection for region 13. Write '0' has no effect."]
pub type REGION13_R = crate::BitReader<REGION13_A>;
#[doc = "Enable protection for region 13. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION13_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION13_A> for bool {
    #[inline(always)]
    fn from(variant: REGION13_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION13_A {
        match self.bits {
            false => REGION13_A::DISABLED,
            true => REGION13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION13_A::ENABLED
    }
}
#[doc = "Field `REGION13` writer - Enable protection for region 13. Write '0' has no effect."]
pub type REGION13_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION13_A, O>;
impl<'a, const O: u8> REGION13_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION13_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION13_A::ENABLED)
    }
}
#[doc = "Field `REGION14` reader - Enable protection for region 14. Write '0' has no effect."]
pub type REGION14_R = crate::BitReader<REGION14_A>;
#[doc = "Enable protection for region 14. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION14_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION14_A> for bool {
    #[inline(always)]
    fn from(variant: REGION14_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION14_A {
        match self.bits {
            false => REGION14_A::DISABLED,
            true => REGION14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION14_A::ENABLED
    }
}
#[doc = "Field `REGION14` writer - Enable protection for region 14. Write '0' has no effect."]
pub type REGION14_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION14_A, O>;
impl<'a, const O: u8> REGION14_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION14_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION14_A::ENABLED)
    }
}
#[doc = "Field `REGION15` reader - Enable protection for region 15. Write '0' has no effect."]
pub type REGION15_R = crate::BitReader<REGION15_A>;
#[doc = "Enable protection for region 15. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION15_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION15_A> for bool {
    #[inline(always)]
    fn from(variant: REGION15_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION15_A {
        match self.bits {
            false => REGION15_A::DISABLED,
            true => REGION15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION15_A::ENABLED
    }
}
#[doc = "Field `REGION15` writer - Enable protection for region 15. Write '0' has no effect."]
pub type REGION15_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION15_A, O>;
impl<'a, const O: u8> REGION15_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION15_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION15_A::ENABLED)
    }
}
#[doc = "Field `REGION16` reader - Enable protection for region 16. Write '0' has no effect."]
pub type REGION16_R = crate::BitReader<REGION16_A>;
#[doc = "Enable protection for region 16. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION16_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION16_A> for bool {
    #[inline(always)]
    fn from(variant: REGION16_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION16_A {
        match self.bits {
            false => REGION16_A::DISABLED,
            true => REGION16_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION16_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION16_A::ENABLED
    }
}
#[doc = "Field `REGION16` writer - Enable protection for region 16. Write '0' has no effect."]
pub type REGION16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION16_A, O>;
impl<'a, const O: u8> REGION16_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION16_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION16_A::ENABLED)
    }
}
#[doc = "Field `REGION17` reader - Enable protection for region 17. Write '0' has no effect."]
pub type REGION17_R = crate::BitReader<REGION17_A>;
#[doc = "Enable protection for region 17. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION17_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION17_A> for bool {
    #[inline(always)]
    fn from(variant: REGION17_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION17_A {
        match self.bits {
            false => REGION17_A::DISABLED,
            true => REGION17_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION17_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION17_A::ENABLED
    }
}
#[doc = "Field `REGION17` writer - Enable protection for region 17. Write '0' has no effect."]
pub type REGION17_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION17_A, O>;
impl<'a, const O: u8> REGION17_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION17_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION17_A::ENABLED)
    }
}
#[doc = "Field `REGION18` reader - Enable protection for region 18. Write '0' has no effect."]
pub type REGION18_R = crate::BitReader<REGION18_A>;
#[doc = "Enable protection for region 18. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION18_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION18_A> for bool {
    #[inline(always)]
    fn from(variant: REGION18_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION18_A {
        match self.bits {
            false => REGION18_A::DISABLED,
            true => REGION18_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION18_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION18_A::ENABLED
    }
}
#[doc = "Field `REGION18` writer - Enable protection for region 18. Write '0' has no effect."]
pub type REGION18_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION18_A, O>;
impl<'a, const O: u8> REGION18_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION18_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION18_A::ENABLED)
    }
}
#[doc = "Field `REGION19` reader - Enable protection for region 19. Write '0' has no effect."]
pub type REGION19_R = crate::BitReader<REGION19_A>;
#[doc = "Enable protection for region 19. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION19_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION19_A> for bool {
    #[inline(always)]
    fn from(variant: REGION19_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION19_A {
        match self.bits {
            false => REGION19_A::DISABLED,
            true => REGION19_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION19_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION19_A::ENABLED
    }
}
#[doc = "Field `REGION19` writer - Enable protection for region 19. Write '0' has no effect."]
pub type REGION19_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION19_A, O>;
impl<'a, const O: u8> REGION19_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION19_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION19_A::ENABLED)
    }
}
#[doc = "Field `REGION20` reader - Enable protection for region 20. Write '0' has no effect."]
pub type REGION20_R = crate::BitReader<REGION20_A>;
#[doc = "Enable protection for region 20. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION20_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION20_A> for bool {
    #[inline(always)]
    fn from(variant: REGION20_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION20_A {
        match self.bits {
            false => REGION20_A::DISABLED,
            true => REGION20_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION20_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION20_A::ENABLED
    }
}
#[doc = "Field `REGION20` writer - Enable protection for region 20. Write '0' has no effect."]
pub type REGION20_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION20_A, O>;
impl<'a, const O: u8> REGION20_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION20_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION20_A::ENABLED)
    }
}
#[doc = "Field `REGION21` reader - Enable protection for region 21. Write '0' has no effect."]
pub type REGION21_R = crate::BitReader<REGION21_A>;
#[doc = "Enable protection for region 21. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION21_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION21_A> for bool {
    #[inline(always)]
    fn from(variant: REGION21_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION21_A {
        match self.bits {
            false => REGION21_A::DISABLED,
            true => REGION21_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION21_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION21_A::ENABLED
    }
}
#[doc = "Field `REGION21` writer - Enable protection for region 21. Write '0' has no effect."]
pub type REGION21_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION21_A, O>;
impl<'a, const O: u8> REGION21_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION21_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION21_A::ENABLED)
    }
}
#[doc = "Field `REGION22` reader - Enable protection for region 22. Write '0' has no effect."]
pub type REGION22_R = crate::BitReader<REGION22_A>;
#[doc = "Enable protection for region 22. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION22_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION22_A> for bool {
    #[inline(always)]
    fn from(variant: REGION22_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION22_A {
        match self.bits {
            false => REGION22_A::DISABLED,
            true => REGION22_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION22_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION22_A::ENABLED
    }
}
#[doc = "Field `REGION22` writer - Enable protection for region 22. Write '0' has no effect."]
pub type REGION22_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION22_A, O>;
impl<'a, const O: u8> REGION22_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION22_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION22_A::ENABLED)
    }
}
#[doc = "Field `REGION23` reader - Enable protection for region 23. Write '0' has no effect."]
pub type REGION23_R = crate::BitReader<REGION23_A>;
#[doc = "Enable protection for region 23. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION23_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION23_A> for bool {
    #[inline(always)]
    fn from(variant: REGION23_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION23_A {
        match self.bits {
            false => REGION23_A::DISABLED,
            true => REGION23_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION23_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION23_A::ENABLED
    }
}
#[doc = "Field `REGION23` writer - Enable protection for region 23. Write '0' has no effect."]
pub type REGION23_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION23_A, O>;
impl<'a, const O: u8> REGION23_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION23_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION23_A::ENABLED)
    }
}
#[doc = "Field `REGION24` reader - Enable protection for region 24. Write '0' has no effect."]
pub type REGION24_R = crate::BitReader<REGION24_A>;
#[doc = "Enable protection for region 24. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION24_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION24_A> for bool {
    #[inline(always)]
    fn from(variant: REGION24_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION24_A {
        match self.bits {
            false => REGION24_A::DISABLED,
            true => REGION24_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION24_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION24_A::ENABLED
    }
}
#[doc = "Field `REGION24` writer - Enable protection for region 24. Write '0' has no effect."]
pub type REGION24_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION24_A, O>;
impl<'a, const O: u8> REGION24_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION24_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION24_A::ENABLED)
    }
}
#[doc = "Field `REGION25` reader - Enable protection for region 25. Write '0' has no effect."]
pub type REGION25_R = crate::BitReader<REGION25_A>;
#[doc = "Enable protection for region 25. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION25_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION25_A> for bool {
    #[inline(always)]
    fn from(variant: REGION25_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION25_A {
        match self.bits {
            false => REGION25_A::DISABLED,
            true => REGION25_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION25_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION25_A::ENABLED
    }
}
#[doc = "Field `REGION25` writer - Enable protection for region 25. Write '0' has no effect."]
pub type REGION25_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION25_A, O>;
impl<'a, const O: u8> REGION25_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION25_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION25_A::ENABLED)
    }
}
#[doc = "Field `REGION26` reader - Enable protection for region 26. Write '0' has no effect."]
pub type REGION26_R = crate::BitReader<REGION26_A>;
#[doc = "Enable protection for region 26. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION26_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION26_A> for bool {
    #[inline(always)]
    fn from(variant: REGION26_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION26_A {
        match self.bits {
            false => REGION26_A::DISABLED,
            true => REGION26_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION26_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION26_A::ENABLED
    }
}
#[doc = "Field `REGION26` writer - Enable protection for region 26. Write '0' has no effect."]
pub type REGION26_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION26_A, O>;
impl<'a, const O: u8> REGION26_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION26_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION26_A::ENABLED)
    }
}
#[doc = "Field `REGION27` reader - Enable protection for region 27. Write '0' has no effect."]
pub type REGION27_R = crate::BitReader<REGION27_A>;
#[doc = "Enable protection for region 27. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION27_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION27_A> for bool {
    #[inline(always)]
    fn from(variant: REGION27_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION27_A {
        match self.bits {
            false => REGION27_A::DISABLED,
            true => REGION27_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION27_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION27_A::ENABLED
    }
}
#[doc = "Field `REGION27` writer - Enable protection for region 27. Write '0' has no effect."]
pub type REGION27_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION27_A, O>;
impl<'a, const O: u8> REGION27_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION27_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION27_A::ENABLED)
    }
}
#[doc = "Field `REGION28` reader - Enable protection for region 28. Write '0' has no effect."]
pub type REGION28_R = crate::BitReader<REGION28_A>;
#[doc = "Enable protection for region 28. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION28_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION28_A> for bool {
    #[inline(always)]
    fn from(variant: REGION28_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION28_A {
        match self.bits {
            false => REGION28_A::DISABLED,
            true => REGION28_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION28_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION28_A::ENABLED
    }
}
#[doc = "Field `REGION28` writer - Enable protection for region 28. Write '0' has no effect."]
pub type REGION28_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION28_A, O>;
impl<'a, const O: u8> REGION28_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION28_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION28_A::ENABLED)
    }
}
#[doc = "Field `REGION29` reader - Enable protection for region 29. Write '0' has no effect."]
pub type REGION29_R = crate::BitReader<REGION29_A>;
#[doc = "Enable protection for region 29. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION29_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION29_A> for bool {
    #[inline(always)]
    fn from(variant: REGION29_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION29_A {
        match self.bits {
            false => REGION29_A::DISABLED,
            true => REGION29_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION29_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION29_A::ENABLED
    }
}
#[doc = "Field `REGION29` writer - Enable protection for region 29. Write '0' has no effect."]
pub type REGION29_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION29_A, O>;
impl<'a, const O: u8> REGION29_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION29_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION29_A::ENABLED)
    }
}
#[doc = "Field `REGION30` reader - Enable protection for region 30. Write '0' has no effect."]
pub type REGION30_R = crate::BitReader<REGION30_A>;
#[doc = "Enable protection for region 30. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION30_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION30_A> for bool {
    #[inline(always)]
    fn from(variant: REGION30_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION30_A {
        match self.bits {
            false => REGION30_A::DISABLED,
            true => REGION30_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION30_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION30_A::ENABLED
    }
}
#[doc = "Field `REGION30` writer - Enable protection for region 30. Write '0' has no effect."]
pub type REGION30_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION30_A, O>;
impl<'a, const O: u8> REGION30_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION30_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION30_A::ENABLED)
    }
}
#[doc = "Field `REGION31` reader - Enable protection for region 31. Write '0' has no effect."]
pub type REGION31_R = crate::BitReader<REGION31_A>;
#[doc = "Enable protection for region 31. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGION31_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION31_A> for bool {
    #[inline(always)]
    fn from(variant: REGION31_A) -> Self {
        variant as u8 != 0
    }
}
impl REGION31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION31_A {
        match self.bits {
            false => REGION31_A::DISABLED,
            true => REGION31_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION31_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION31_A::ENABLED
    }
}
#[doc = "Field `REGION31` writer - Enable protection for region 31. Write '0' has no effect."]
pub type REGION31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, REGION31_A, O>;
impl<'a, const O: u8> REGION31_W<'a, O> {
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION31_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION31_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    pub fn region0(&self) -> REGION0_R {
        REGION0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    pub fn region1(&self) -> REGION1_R {
        REGION1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    pub fn region2(&self) -> REGION2_R {
        REGION2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    pub fn region3(&self) -> REGION3_R {
        REGION3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    pub fn region4(&self) -> REGION4_R {
        REGION4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    pub fn region5(&self) -> REGION5_R {
        REGION5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    pub fn region6(&self) -> REGION6_R {
        REGION6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    pub fn region7(&self) -> REGION7_R {
        REGION7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    pub fn region8(&self) -> REGION8_R {
        REGION8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    pub fn region9(&self) -> REGION9_R {
        REGION9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    pub fn region10(&self) -> REGION10_R {
        REGION10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    pub fn region11(&self) -> REGION11_R {
        REGION11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    pub fn region12(&self) -> REGION12_R {
        REGION12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    pub fn region13(&self) -> REGION13_R {
        REGION13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    pub fn region14(&self) -> REGION14_R {
        REGION14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    pub fn region15(&self) -> REGION15_R {
        REGION15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    pub fn region16(&self) -> REGION16_R {
        REGION16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    pub fn region17(&self) -> REGION17_R {
        REGION17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    pub fn region18(&self) -> REGION18_R {
        REGION18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    pub fn region19(&self) -> REGION19_R {
        REGION19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    pub fn region20(&self) -> REGION20_R {
        REGION20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    pub fn region21(&self) -> REGION21_R {
        REGION21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    pub fn region22(&self) -> REGION22_R {
        REGION22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    pub fn region23(&self) -> REGION23_R {
        REGION23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    pub fn region24(&self) -> REGION24_R {
        REGION24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    pub fn region25(&self) -> REGION25_R {
        REGION25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    pub fn region26(&self) -> REGION26_R {
        REGION26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    pub fn region27(&self) -> REGION27_R {
        REGION27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    pub fn region28(&self) -> REGION28_R {
        REGION28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    pub fn region29(&self) -> REGION29_R {
        REGION29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    pub fn region30(&self) -> REGION30_R {
        REGION30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    pub fn region31(&self) -> REGION31_R {
        REGION31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region0(&mut self) -> REGION0_W<0> {
        REGION0_W::new(self)
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region1(&mut self) -> REGION1_W<1> {
        REGION1_W::new(self)
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region2(&mut self) -> REGION2_W<2> {
        REGION2_W::new(self)
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region3(&mut self) -> REGION3_W<3> {
        REGION3_W::new(self)
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region4(&mut self) -> REGION4_W<4> {
        REGION4_W::new(self)
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region5(&mut self) -> REGION5_W<5> {
        REGION5_W::new(self)
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region6(&mut self) -> REGION6_W<6> {
        REGION6_W::new(self)
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region7(&mut self) -> REGION7_W<7> {
        REGION7_W::new(self)
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region8(&mut self) -> REGION8_W<8> {
        REGION8_W::new(self)
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region9(&mut self) -> REGION9_W<9> {
        REGION9_W::new(self)
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region10(&mut self) -> REGION10_W<10> {
        REGION10_W::new(self)
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region11(&mut self) -> REGION11_W<11> {
        REGION11_W::new(self)
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region12(&mut self) -> REGION12_W<12> {
        REGION12_W::new(self)
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region13(&mut self) -> REGION13_W<13> {
        REGION13_W::new(self)
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region14(&mut self) -> REGION14_W<14> {
        REGION14_W::new(self)
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region15(&mut self) -> REGION15_W<15> {
        REGION15_W::new(self)
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region16(&mut self) -> REGION16_W<16> {
        REGION16_W::new(self)
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region17(&mut self) -> REGION17_W<17> {
        REGION17_W::new(self)
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region18(&mut self) -> REGION18_W<18> {
        REGION18_W::new(self)
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region19(&mut self) -> REGION19_W<19> {
        REGION19_W::new(self)
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region20(&mut self) -> REGION20_W<20> {
        REGION20_W::new(self)
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region21(&mut self) -> REGION21_W<21> {
        REGION21_W::new(self)
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region22(&mut self) -> REGION22_W<22> {
        REGION22_W::new(self)
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region23(&mut self) -> REGION23_W<23> {
        REGION23_W::new(self)
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region24(&mut self) -> REGION24_W<24> {
        REGION24_W::new(self)
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region25(&mut self) -> REGION25_W<25> {
        REGION25_W::new(self)
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region26(&mut self) -> REGION26_W<26> {
        REGION26_W::new(self)
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region27(&mut self) -> REGION27_W<27> {
        REGION27_W::new(self)
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region28(&mut self) -> REGION28_W<28> {
        REGION28_W::new(self)
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region29(&mut self) -> REGION29_W<29> {
        REGION29_W::new(self)
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region30(&mut self) -> REGION30_W<30> {
        REGION30_W::new(self)
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn region31(&mut self) -> REGION31_W<31> {
        REGION31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block protect configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config0](index.html) module"]
pub struct CONFIG0_SPEC;
impl crate::RegisterSpec for CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config0::R](R) reader structure"]
impl crate::Readable for CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config0::W](W) writer structure"]
impl crate::Writable for CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG0 to value 0"]
impl crate::Resettable for CONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
