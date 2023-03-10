#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY` reader - Write '1' to disable interrupt for event READY"]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::DISABLED,
            true => READY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event READY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - Write '1' to disable interrupt for event READY"]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, READY_AW, O>;
impl<'a, const O: u8> READY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(READY_AW::CLEAR)
    }
}
#[doc = "Field `FIELDDETECTED` reader - Write '1' to disable interrupt for event FIELDDETECTED"]
pub type FIELDDETECTED_R = crate::BitReader<FIELDDETECTED_A>;
#[doc = "Write '1' to disable interrupt for event FIELDDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELDDETECTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<FIELDDETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELDDETECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDDETECTED_A {
        match self.bits {
            false => FIELDDETECTED_A::DISABLED,
            true => FIELDDETECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDDETECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDDETECTED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event FIELDDETECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELDDETECTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<FIELDDETECTED_AW> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDDETECTED` writer - Write '1' to disable interrupt for event FIELDDETECTED"]
pub type FIELDDETECTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, FIELDDETECTED_AW, O>;
impl<'a, const O: u8> FIELDDETECTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FIELDDETECTED_AW::CLEAR)
    }
}
#[doc = "Field `FIELDLOST` reader - Write '1' to disable interrupt for event FIELDLOST"]
pub type FIELDLOST_R = crate::BitReader<FIELDLOST_A>;
#[doc = "Write '1' to disable interrupt for event FIELDLOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELDLOST_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<FIELDLOST_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELDLOST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDLOST_A {
        match self.bits {
            false => FIELDLOST_A::DISABLED,
            true => FIELDLOST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDLOST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDLOST_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event FIELDLOST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELDLOST_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<FIELDLOST_AW> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELDLOST` writer - Write '1' to disable interrupt for event FIELDLOST"]
pub type FIELDLOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, FIELDLOST_AW, O>;
impl<'a, const O: u8> FIELDLOST_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FIELDLOST_AW::CLEAR)
    }
}
#[doc = "Field `TXFRAMESTART` reader - Write '1' to disable interrupt for event TXFRAMESTART"]
pub type TXFRAMESTART_R = crate::BitReader<TXFRAMESTART_A>;
#[doc = "Write '1' to disable interrupt for event TXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFRAMESTART_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXFRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFRAMESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMESTART_A {
        match self.bits {
            false => TXFRAMESTART_A::DISABLED,
            true => TXFRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMESTART_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event TXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFRAMESTART_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXFRAMESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMESTART` writer - Write '1' to disable interrupt for event TXFRAMESTART"]
pub type TXFRAMESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, TXFRAMESTART_AW, O>;
impl<'a, const O: u8> TXFRAMESTART_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFRAMESTART_AW::CLEAR)
    }
}
#[doc = "Field `TXFRAMEEND` reader - Write '1' to disable interrupt for event TXFRAMEEND"]
pub type TXFRAMEEND_R = crate::BitReader<TXFRAMEEND_A>;
#[doc = "Write '1' to disable interrupt for event TXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFRAMEEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXFRAMEEND_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMEEND_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFRAMEEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMEEND_A {
        match self.bits {
            false => TXFRAMEEND_A::DISABLED,
            true => TXFRAMEEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMEEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMEEND_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event TXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXFRAMEEND_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXFRAMEEND_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMEEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFRAMEEND` writer - Write '1' to disable interrupt for event TXFRAMEEND"]
pub type TXFRAMEEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, TXFRAMEEND_AW, O>;
impl<'a, const O: u8> TXFRAMEEND_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFRAMEEND_AW::CLEAR)
    }
}
#[doc = "Field `RXFRAMESTART` reader - Write '1' to disable interrupt for event RXFRAMESTART"]
pub type RXFRAMESTART_R = crate::BitReader<RXFRAMESTART_A>;
#[doc = "Write '1' to disable interrupt for event RXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRAMESTART_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXFRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFRAMESTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFRAMESTART_A {
        match self.bits {
            false => RXFRAMESTART_A::DISABLED,
            true => RXFRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFRAMESTART_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RXFRAMESTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRAMESTART_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXFRAMESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMESTART` writer - Write '1' to disable interrupt for event RXFRAMESTART"]
pub type RXFRAMESTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, RXFRAMESTART_AW, O>;
impl<'a, const O: u8> RXFRAMESTART_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFRAMESTART_AW::CLEAR)
    }
}
#[doc = "Field `RXFRAMEEND` reader - Write '1' to disable interrupt for event RXFRAMEEND"]
pub type RXFRAMEEND_R = crate::BitReader<RXFRAMEEND_A>;
#[doc = "Write '1' to disable interrupt for event RXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRAMEEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXFRAMEEND_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMEEND_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFRAMEEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFRAMEEND_A {
        match self.bits {
            false => RXFRAMEEND_A::DISABLED,
            true => RXFRAMEEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFRAMEEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFRAMEEND_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RXFRAMEEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFRAMEEND_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXFRAMEEND_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMEEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFRAMEEND` writer - Write '1' to disable interrupt for event RXFRAMEEND"]
pub type RXFRAMEEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RXFRAMEEND_AW, O>;
impl<'a, const O: u8> RXFRAMEEND_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFRAMEEND_AW::CLEAR)
    }
}
#[doc = "Field `ERROR` reader - Write '1' to disable interrupt for event ERROR"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERROR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to disable interrupt for event ERROR"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, ERROR_AW, O>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERROR_AW::CLEAR)
    }
}
#[doc = "Field `RXERROR` reader - Write '1' to disable interrupt for event RXERROR"]
pub type RXERROR_R = crate::BitReader<RXERROR_A>;
#[doc = "Write '1' to disable interrupt for event RXERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RXERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl RXERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXERROR_A {
        match self.bits {
            false => RXERROR_A::DISABLED,
            true => RXERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXERROR_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RXERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXERROR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: RXERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXERROR` writer - Write '1' to disable interrupt for event RXERROR"]
pub type RXERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RXERROR_AW, O>;
impl<'a, const O: u8> RXERROR_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXERROR_AW::CLEAR)
    }
}
#[doc = "Field `ENDRX` reader - Write '1' to disable interrupt for event ENDRX"]
pub type ENDRX_R = crate::BitReader<ENDRX_A>;
#[doc = "Write '1' to disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDRX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::DISABLED,
            true => ENDRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDRX_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDRX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` writer - Write '1' to disable interrupt for event ENDRX"]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, ENDRX_AW, O>;
impl<'a, const O: u8> ENDRX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDRX_AW::CLEAR)
    }
}
#[doc = "Field `ENDTX` reader - Write '1' to disable interrupt for event ENDTX"]
pub type ENDTX_R = crate::BitReader<ENDTX_A>;
#[doc = "Write '1' to disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDTX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDTX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTX_A {
        match self.bits {
            false => ENDTX_A::DISABLED,
            true => ENDTX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDTX_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDTX_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDTX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` writer - Write '1' to disable interrupt for event ENDTX"]
pub type ENDTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, ENDTX_AW, O>;
impl<'a, const O: u8> ENDTX_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDTX_AW::CLEAR)
    }
}
#[doc = "Field `AUTOCOLRESSTARTED` reader - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
pub type AUTOCOLRESSTARTED_R = crate::BitReader<AUTOCOLRESSTARTED_A>;
#[doc = "Write '1' to disable interrupt for event AUTOCOLRESSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOCOLRESSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<AUTOCOLRESSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOCOLRESSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOCOLRESSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOCOLRESSTARTED_A {
        match self.bits {
            false => AUTOCOLRESSTARTED_A::DISABLED,
            true => AUTOCOLRESSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOCOLRESSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOCOLRESSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event AUTOCOLRESSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOCOLRESSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<AUTOCOLRESSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: AUTOCOLRESSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTOCOLRESSTARTED` writer - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
pub type AUTOCOLRESSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, AUTOCOLRESSTARTED_AW, O>;
impl<'a, const O: u8> AUTOCOLRESSTARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AUTOCOLRESSTARTED_AW::CLEAR)
    }
}
#[doc = "Field `COLLISION` reader - Write '1' to disable interrupt for event COLLISION"]
pub type COLLISION_R = crate::BitReader<COLLISION_A>;
#[doc = "Write '1' to disable interrupt for event COLLISION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLLISION_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COLLISION_A> for bool {
    #[inline(always)]
    fn from(variant: COLLISION_A) -> Self {
        variant as u8 != 0
    }
}
impl COLLISION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLLISION_A {
        match self.bits {
            false => COLLISION_A::DISABLED,
            true => COLLISION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COLLISION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COLLISION_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event COLLISION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COLLISION_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COLLISION_AW> for bool {
    #[inline(always)]
    fn from(variant: COLLISION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COLLISION` writer - Write '1' to disable interrupt for event COLLISION"]
pub type COLLISION_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, COLLISION_AW, O>;
impl<'a, const O: u8> COLLISION_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COLLISION_AW::CLEAR)
    }
}
#[doc = "Field `SELECTED` reader - Write '1' to disable interrupt for event SELECTED"]
pub type SELECTED_R = crate::BitReader<SELECTED_A>;
#[doc = "Write '1' to disable interrupt for event SELECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELECTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SELECTED_A> for bool {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        variant as u8 != 0
    }
}
impl SELECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELECTED_A {
        match self.bits {
            false => SELECTED_A::DISABLED,
            true => SELECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SELECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SELECTED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event SELECTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELECTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SELECTED_AW> for bool {
    #[inline(always)]
    fn from(variant: SELECTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECTED` writer - Write '1' to disable interrupt for event SELECTED"]
pub type SELECTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SELECTED_AW, O>;
impl<'a, const O: u8> SELECTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SELECTED_AW::CLEAR)
    }
}
#[doc = "Field `STARTED` reader - Write '1' to disable interrupt for event STARTED"]
pub type STARTED_R = crate::BitReader<STARTED_A>;
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
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
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<STARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: STARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` writer - Write '1' to disable interrupt for event STARTED"]
pub type STARTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, STARTED_AW, O>;
impl<'a, const O: u8> STARTED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STARTED_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    pub fn fielddetected(&self) -> FIELDDETECTED_R {
        FIELDDETECTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event FIELDLOST"]
    #[inline(always)]
    pub fn fieldlost(&self) -> FIELDLOST_R {
        FIELDLOST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    pub fn txframestart(&self) -> TXFRAMESTART_R {
        TXFRAMESTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    pub fn txframeend(&self) -> TXFRAMEEND_R {
        TXFRAMEEND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    pub fn rxframestart(&self) -> RXFRAMESTART_R {
        RXFRAMESTART_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    pub fn rxframeend(&self) -> RXFRAMEEND_R {
        RXFRAMEEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event RXERROR"]
    #[inline(always)]
    pub fn rxerror(&self) -> RXERROR_R {
        RXERROR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    pub fn autocolresstarted(&self) -> AUTOCOLRESSTARTED_R {
        AUTOCOLRESSTARTED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COLLISION"]
    #[inline(always)]
    pub fn collision(&self) -> COLLISION_R {
        COLLISION_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event SELECTED"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event READY"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<0> {
        READY_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event FIELDDETECTED"]
    #[inline(always)]
    #[must_use]
    pub fn fielddetected(&mut self) -> FIELDDETECTED_W<1> {
        FIELDDETECTED_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event FIELDLOST"]
    #[inline(always)]
    #[must_use]
    pub fn fieldlost(&mut self) -> FIELDLOST_W<2> {
        FIELDLOST_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event TXFRAMESTART"]
    #[inline(always)]
    #[must_use]
    pub fn txframestart(&mut self) -> TXFRAMESTART_W<3> {
        TXFRAMESTART_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event TXFRAMEEND"]
    #[inline(always)]
    #[must_use]
    pub fn txframeend(&mut self) -> TXFRAMEEND_W<4> {
        TXFRAMEEND_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event RXFRAMESTART"]
    #[inline(always)]
    #[must_use]
    pub fn rxframestart(&mut self) -> RXFRAMESTART_W<5> {
        RXFRAMESTART_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event RXFRAMEEND"]
    #[inline(always)]
    #[must_use]
    pub fn rxframeend(&mut self) -> RXFRAMEEND_W<6> {
        RXFRAMEEND_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<7> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event RXERROR"]
    #[inline(always)]
    #[must_use]
    pub fn rxerror(&mut self) -> RXERROR_W<10> {
        RXERROR_W::new(self)
    }
    #[doc = "Bit 11 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<11> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 12 - Write '1' to disable interrupt for event ENDTX"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<12> {
        ENDTX_W::new(self)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event AUTOCOLRESSTARTED"]
    #[inline(always)]
    #[must_use]
    pub fn autocolresstarted(&mut self) -> AUTOCOLRESSTARTED_W<14> {
        AUTOCOLRESSTARTED_W::new(self)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COLLISION"]
    #[inline(always)]
    #[must_use]
    pub fn collision(&mut self) -> COLLISION_W<18> {
        COLLISION_W::new(self)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event SELECTED"]
    #[inline(always)]
    #[must_use]
    pub fn selected(&mut self) -> SELECTED_W<19> {
        SELECTED_W::new(self)
    }
    #[doc = "Bit 20 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    #[must_use]
    pub fn started(&mut self) -> STARTED_W<20> {
        STARTED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
