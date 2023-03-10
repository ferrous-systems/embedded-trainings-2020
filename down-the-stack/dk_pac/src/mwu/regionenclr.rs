#[doc = "Register `REGIONENCLR` reader"]
pub struct R(crate::R<REGIONENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGIONENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGIONENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGIONENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGIONENCLR` writer"]
pub struct W(crate::W<REGIONENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGIONENCLR_SPEC>;
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
impl From<crate::W<REGIONENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGIONENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RGN0WA` reader - Disable write access watch in region\\[0\\]"]
pub type RGN0WA_R = crate::BitReader<RGN0WA_A>;
#[doc = "Disable write access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN0WA_A {
    #[doc = "0: Write access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN0WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN0WA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN0WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN0WA_A {
        match self.bits {
            false => RGN0WA_A::DISABLED,
            true => RGN0WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN0WA_A::ENABLED
    }
}
#[doc = "Disable write access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN0WA_AW {
    #[doc = "1: Disable write access watch in this region"]
    CLEAR = 1,
}
impl From<RGN0WA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN0WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0WA` writer - Disable write access watch in region\\[0\\]"]
pub type RGN0WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN0WA_AW, O>;
impl<'a, const O: u8> RGN0WA_W<'a, O> {
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN0WA_AW::CLEAR)
    }
}
#[doc = "Field `RGN0RA` reader - Disable read access watch in region\\[0\\]"]
pub type RGN0RA_R = crate::BitReader<RGN0RA_A>;
#[doc = "Disable read access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN0RA_A {
    #[doc = "0: Read access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN0RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN0RA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN0RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN0RA_A {
        match self.bits {
            false => RGN0RA_A::DISABLED,
            true => RGN0RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN0RA_A::ENABLED
    }
}
#[doc = "Disable read access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN0RA_AW {
    #[doc = "1: Disable read access watch in this region"]
    CLEAR = 1,
}
impl From<RGN0RA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN0RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0RA` writer - Disable read access watch in region\\[0\\]"]
pub type RGN0RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN0RA_AW, O>;
impl<'a, const O: u8> RGN0RA_W<'a, O> {
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN0RA_AW::CLEAR)
    }
}
#[doc = "Field `RGN1WA` reader - Disable write access watch in region\\[1\\]"]
pub type RGN1WA_R = crate::BitReader<RGN1WA_A>;
#[doc = "Disable write access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN1WA_A {
    #[doc = "0: Write access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN1WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN1WA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN1WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN1WA_A {
        match self.bits {
            false => RGN1WA_A::DISABLED,
            true => RGN1WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN1WA_A::ENABLED
    }
}
#[doc = "Disable write access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN1WA_AW {
    #[doc = "1: Disable write access watch in this region"]
    CLEAR = 1,
}
impl From<RGN1WA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN1WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1WA` writer - Disable write access watch in region\\[1\\]"]
pub type RGN1WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN1WA_AW, O>;
impl<'a, const O: u8> RGN1WA_W<'a, O> {
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN1WA_AW::CLEAR)
    }
}
#[doc = "Field `RGN1RA` reader - Disable read access watch in region\\[1\\]"]
pub type RGN1RA_R = crate::BitReader<RGN1RA_A>;
#[doc = "Disable read access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN1RA_A {
    #[doc = "0: Read access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN1RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN1RA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN1RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN1RA_A {
        match self.bits {
            false => RGN1RA_A::DISABLED,
            true => RGN1RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN1RA_A::ENABLED
    }
}
#[doc = "Disable read access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN1RA_AW {
    #[doc = "1: Disable read access watch in this region"]
    CLEAR = 1,
}
impl From<RGN1RA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN1RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1RA` writer - Disable read access watch in region\\[1\\]"]
pub type RGN1RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN1RA_AW, O>;
impl<'a, const O: u8> RGN1RA_W<'a, O> {
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN1RA_AW::CLEAR)
    }
}
#[doc = "Field `RGN2WA` reader - Disable write access watch in region\\[2\\]"]
pub type RGN2WA_R = crate::BitReader<RGN2WA_A>;
#[doc = "Disable write access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN2WA_A {
    #[doc = "0: Write access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN2WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN2WA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN2WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN2WA_A {
        match self.bits {
            false => RGN2WA_A::DISABLED,
            true => RGN2WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN2WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN2WA_A::ENABLED
    }
}
#[doc = "Disable write access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN2WA_AW {
    #[doc = "1: Disable write access watch in this region"]
    CLEAR = 1,
}
impl From<RGN2WA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN2WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2WA` writer - Disable write access watch in region\\[2\\]"]
pub type RGN2WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN2WA_AW, O>;
impl<'a, const O: u8> RGN2WA_W<'a, O> {
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN2WA_AW::CLEAR)
    }
}
#[doc = "Field `RGN2RA` reader - Disable read access watch in region\\[2\\]"]
pub type RGN2RA_R = crate::BitReader<RGN2RA_A>;
#[doc = "Disable read access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN2RA_A {
    #[doc = "0: Read access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN2RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN2RA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN2RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN2RA_A {
        match self.bits {
            false => RGN2RA_A::DISABLED,
            true => RGN2RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN2RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN2RA_A::ENABLED
    }
}
#[doc = "Disable read access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN2RA_AW {
    #[doc = "1: Disable read access watch in this region"]
    CLEAR = 1,
}
impl From<RGN2RA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN2RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2RA` writer - Disable read access watch in region\\[2\\]"]
pub type RGN2RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN2RA_AW, O>;
impl<'a, const O: u8> RGN2RA_W<'a, O> {
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN2RA_AW::CLEAR)
    }
}
#[doc = "Field `RGN3WA` reader - Disable write access watch in region\\[3\\]"]
pub type RGN3WA_R = crate::BitReader<RGN3WA_A>;
#[doc = "Disable write access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN3WA_A {
    #[doc = "0: Write access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Write access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN3WA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN3WA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN3WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN3WA_A {
        match self.bits {
            false => RGN3WA_A::DISABLED,
            true => RGN3WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN3WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN3WA_A::ENABLED
    }
}
#[doc = "Disable write access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN3WA_AW {
    #[doc = "1: Disable write access watch in this region"]
    CLEAR = 1,
}
impl From<RGN3WA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN3WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3WA` writer - Disable write access watch in region\\[3\\]"]
pub type RGN3WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN3WA_AW, O>;
impl<'a, const O: u8> RGN3WA_W<'a, O> {
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN3WA_AW::CLEAR)
    }
}
#[doc = "Field `RGN3RA` reader - Disable read access watch in region\\[3\\]"]
pub type RGN3RA_R = crate::BitReader<RGN3RA_A>;
#[doc = "Disable read access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN3RA_A {
    #[doc = "0: Read access watch in this region is disabled"]
    DISABLED = 0,
    #[doc = "1: Read access watch in this region is enabled"]
    ENABLED = 1,
}
impl From<RGN3RA_A> for bool {
    #[inline(always)]
    fn from(variant: RGN3RA_A) -> Self {
        variant as u8 != 0
    }
}
impl RGN3RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGN3RA_A {
        match self.bits {
            false => RGN3RA_A::DISABLED,
            true => RGN3RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RGN3RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RGN3RA_A::ENABLED
    }
}
#[doc = "Disable read access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RGN3RA_AW {
    #[doc = "1: Disable read access watch in this region"]
    CLEAR = 1,
}
impl From<RGN3RA_AW> for bool {
    #[inline(always)]
    fn from(variant: RGN3RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3RA` writer - Disable read access watch in region\\[3\\]"]
pub type RGN3RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, RGN3RA_AW, O>;
impl<'a, const O: u8> RGN3RA_W<'a, O> {
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RGN3RA_AW::CLEAR)
    }
}
#[doc = "Field `PRGN0WA` reader - Disable write access watch in PREGION\\[0\\]"]
pub type PRGN0WA_R = crate::BitReader<PRGN0WA_A>;
#[doc = "Disable write access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN0WA_A {
    #[doc = "0: Write access watch in this PREGION is disabled"]
    DISABLED = 0,
    #[doc = "1: Write access watch in this PREGION is enabled"]
    ENABLED = 1,
}
impl From<PRGN0WA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN0WA_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGN0WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN0WA_A {
        match self.bits {
            false => PRGN0WA_A::DISABLED,
            true => PRGN0WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN0WA_A::ENABLED
    }
}
#[doc = "Disable write access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN0WA_AW {
    #[doc = "1: Disable write access watch in this PREGION"]
    CLEAR = 1,
}
impl From<PRGN0WA_AW> for bool {
    #[inline(always)]
    fn from(variant: PRGN0WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0WA` writer - Disable write access watch in PREGION\\[0\\]"]
pub type PRGN0WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, PRGN0WA_AW, O>;
impl<'a, const O: u8> PRGN0WA_W<'a, O> {
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN0WA_AW::CLEAR)
    }
}
#[doc = "Field `PRGN0RA` reader - Disable read access watch in PREGION\\[0\\]"]
pub type PRGN0RA_R = crate::BitReader<PRGN0RA_A>;
#[doc = "Disable read access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN0RA_A {
    #[doc = "0: Read access watch in this PREGION is disabled"]
    DISABLED = 0,
    #[doc = "1: Read access watch in this PREGION is enabled"]
    ENABLED = 1,
}
impl From<PRGN0RA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN0RA_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGN0RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN0RA_A {
        match self.bits {
            false => PRGN0RA_A::DISABLED,
            true => PRGN0RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN0RA_A::ENABLED
    }
}
#[doc = "Disable read access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN0RA_AW {
    #[doc = "1: Disable read access watch in this PREGION"]
    CLEAR = 1,
}
impl From<PRGN0RA_AW> for bool {
    #[inline(always)]
    fn from(variant: PRGN0RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0RA` writer - Disable read access watch in PREGION\\[0\\]"]
pub type PRGN0RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, PRGN0RA_AW, O>;
impl<'a, const O: u8> PRGN0RA_W<'a, O> {
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN0RA_AW::CLEAR)
    }
}
#[doc = "Field `PRGN1WA` reader - Disable write access watch in PREGION\\[1\\]"]
pub type PRGN1WA_R = crate::BitReader<PRGN1WA_A>;
#[doc = "Disable write access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN1WA_A {
    #[doc = "0: Write access watch in this PREGION is disabled"]
    DISABLED = 0,
    #[doc = "1: Write access watch in this PREGION is enabled"]
    ENABLED = 1,
}
impl From<PRGN1WA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN1WA_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGN1WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN1WA_A {
        match self.bits {
            false => PRGN1WA_A::DISABLED,
            true => PRGN1WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN1WA_A::ENABLED
    }
}
#[doc = "Disable write access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN1WA_AW {
    #[doc = "1: Disable write access watch in this PREGION"]
    CLEAR = 1,
}
impl From<PRGN1WA_AW> for bool {
    #[inline(always)]
    fn from(variant: PRGN1WA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1WA` writer - Disable write access watch in PREGION\\[1\\]"]
pub type PRGN1WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, PRGN1WA_AW, O>;
impl<'a, const O: u8> PRGN1WA_W<'a, O> {
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN1WA_AW::CLEAR)
    }
}
#[doc = "Field `PRGN1RA` reader - Disable read access watch in PREGION\\[1\\]"]
pub type PRGN1RA_R = crate::BitReader<PRGN1RA_A>;
#[doc = "Disable read access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN1RA_A {
    #[doc = "0: Read access watch in this PREGION is disabled"]
    DISABLED = 0,
    #[doc = "1: Read access watch in this PREGION is enabled"]
    ENABLED = 1,
}
impl From<PRGN1RA_A> for bool {
    #[inline(always)]
    fn from(variant: PRGN1RA_A) -> Self {
        variant as u8 != 0
    }
}
impl PRGN1RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRGN1RA_A {
        match self.bits {
            false => PRGN1RA_A::DISABLED,
            true => PRGN1RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRGN1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRGN1RA_A::ENABLED
    }
}
#[doc = "Disable read access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRGN1RA_AW {
    #[doc = "1: Disable read access watch in this PREGION"]
    CLEAR = 1,
}
impl From<PRGN1RA_AW> for bool {
    #[inline(always)]
    fn from(variant: PRGN1RA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1RA` writer - Disable read access watch in PREGION\\[1\\]"]
pub type PRGN1RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, REGIONENCLR_SPEC, PRGN1RA_AW, O>;
impl<'a, const O: u8> PRGN1RA_W<'a, O> {
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PRGN1RA_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Disable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0wa(&self) -> RGN0WA_R {
        RGN0WA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0ra(&self) -> RGN0RA_R {
        RGN0RA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable write access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1wa(&self) -> RGN1WA_R {
        RGN1WA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable read access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1ra(&self) -> RGN1RA_R {
        RGN1RA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable write access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2wa(&self) -> RGN2WA_R {
        RGN2WA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable read access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2ra(&self) -> RGN2RA_R {
        RGN2RA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable write access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3wa(&self) -> RGN3WA_R {
        RGN3WA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable read access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3ra(&self) -> RGN3RA_R {
        RGN3RA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0wa(&self) -> PRGN0WA_R {
        PRGN0WA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0ra(&self) -> PRGN0RA_R {
        PRGN0RA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1wa(&self) -> PRGN1WA_R {
        PRGN1WA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Disable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1ra(&self) -> PRGN1RA_R {
        PRGN1RA_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable write access watch in region\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn0wa(&mut self) -> RGN0WA_W<0> {
        RGN0WA_W::new(self)
    }
    #[doc = "Bit 1 - Disable read access watch in region\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn0ra(&mut self) -> RGN0RA_W<1> {
        RGN0RA_W::new(self)
    }
    #[doc = "Bit 2 - Disable write access watch in region\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn1wa(&mut self) -> RGN1WA_W<2> {
        RGN1WA_W::new(self)
    }
    #[doc = "Bit 3 - Disable read access watch in region\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn1ra(&mut self) -> RGN1RA_W<3> {
        RGN1RA_W::new(self)
    }
    #[doc = "Bit 4 - Disable write access watch in region\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn2wa(&mut self) -> RGN2WA_W<4> {
        RGN2WA_W::new(self)
    }
    #[doc = "Bit 5 - Disable read access watch in region\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn2ra(&mut self) -> RGN2RA_W<5> {
        RGN2RA_W::new(self)
    }
    #[doc = "Bit 6 - Disable write access watch in region\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn3wa(&mut self) -> RGN3WA_W<6> {
        RGN3WA_W::new(self)
    }
    #[doc = "Bit 7 - Disable read access watch in region\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rgn3ra(&mut self) -> RGN3RA_W<7> {
        RGN3RA_W::new(self)
    }
    #[doc = "Bit 24 - Disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn prgn0wa(&mut self) -> PRGN0WA_W<24> {
        PRGN0WA_W::new(self)
    }
    #[doc = "Bit 25 - Disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn prgn0ra(&mut self) -> PRGN0RA_W<25> {
        PRGN0RA_W::new(self)
    }
    #[doc = "Bit 26 - Disable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn prgn1wa(&mut self) -> PRGN1WA_W<26> {
        PRGN1WA_W::new(self)
    }
    #[doc = "Bit 27 - Disable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn prgn1ra(&mut self) -> PRGN1RA_W<27> {
        PRGN1RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable regions watch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [regionenclr](index.html) module"]
pub struct REGIONENCLR_SPEC;
impl crate::RegisterSpec for REGIONENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [regionenclr::R](R) reader structure"]
impl crate::Readable for REGIONENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [regionenclr::W](W) writer structure"]
impl crate::Writable for REGIONENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGIONENCLR to value 0"]
impl crate::Resettable for REGIONENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
