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
#[doc = "Field `STOPPED` reader - Write '1' to disable interrupt for event STOPPED"]
pub type STOPPED_R = crate::BitReader<STOPPED_A>;
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
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
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<STOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to disable interrupt for event STOPPED"]
pub type STOPPED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, STOPPED_AW, O>;
impl<'a, const O: u8> STOPPED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPPED_AW::CLEAR)
    }
}
#[doc = "Field `RXDREADY` reader - Write '1' to disable interrupt for event RXDREADY"]
pub type RXDREADY_R = crate::BitReader<RXDREADY_A>;
#[doc = "Write '1' to disable interrupt for event RXDREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDREADY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXDREADY_A> for bool {
    #[inline(always)]
    fn from(variant: RXDREADY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDREADY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDREADY_A {
        match self.bits {
            false => RXDREADY_A::DISABLED,
            true => RXDREADY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDREADY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDREADY_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event RXDREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDREADY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXDREADY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDREADY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDREADY` writer - Write '1' to disable interrupt for event RXDREADY"]
pub type RXDREADY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, RXDREADY_AW, O>;
impl<'a, const O: u8> RXDREADY_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXDREADY_AW::CLEAR)
    }
}
#[doc = "Field `TXDSENT` reader - Write '1' to disable interrupt for event TXDSENT"]
pub type TXDSENT_R = crate::BitReader<TXDSENT_A>;
#[doc = "Write '1' to disable interrupt for event TXDSENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDSENT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXDSENT_A> for bool {
    #[inline(always)]
    fn from(variant: TXDSENT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDSENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDSENT_A {
        match self.bits {
            false => TXDSENT_A::DISABLED,
            true => TXDSENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDSENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDSENT_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event TXDSENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDSENT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXDSENT_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDSENT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSENT` writer - Write '1' to disable interrupt for event TXDSENT"]
pub type TXDSENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, TXDSENT_AW, O>;
impl<'a, const O: u8> TXDSENT_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXDSENT_AW::CLEAR)
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
#[doc = "Field `BB` reader - Write '1' to disable interrupt for event BB"]
pub type BB_R = crate::BitReader<BB_A>;
#[doc = "Write '1' to disable interrupt for event BB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<BB_A> for bool {
    #[inline(always)]
    fn from(variant: BB_A) -> Self {
        variant as u8 != 0
    }
}
impl BB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_A {
        match self.bits {
            false => BB_A::DISABLED,
            true => BB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BB_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event BB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<BB_AW> for bool {
    #[inline(always)]
    fn from(variant: BB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` writer - Write '1' to disable interrupt for event BB"]
pub type BB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, BB_AW, O>;
impl<'a, const O: u8> BB_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BB_AW::CLEAR)
    }
}
#[doc = "Field `SUSPENDED` reader - Write '1' to disable interrupt for event SUSPENDED"]
pub type SUSPENDED_R = crate::BitReader<SUSPENDED_A>;
#[doc = "Write '1' to disable interrupt for event SUSPENDED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPENDED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SUSPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SUSPENDED_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSPENDED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSPENDED_A {
        match self.bits {
            false => SUSPENDED_A::DISABLED,
            true => SUSPENDED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SUSPENDED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SUSPENDED_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event SUSPENDED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSPENDED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SUSPENDED_AW> for bool {
    #[inline(always)]
    fn from(variant: SUSPENDED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDED` writer - Write '1' to disable interrupt for event SUSPENDED"]
pub type SUSPENDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SUSPENDED_AW, O>;
impl<'a, const O: u8> SUSPENDED_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SUSPENDED_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event RXDREADY"]
    #[inline(always)]
    pub fn rxdready(&self) -> RXDREADY_R {
        RXDREADY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event TXDSENT"]
    #[inline(always)]
    pub fn txdsent(&self) -> TXDSENT_R {
        TXDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event BB"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event SUSPENDED"]
    #[inline(always)]
    pub fn suspended(&self) -> SUSPENDED_R {
        SUSPENDED_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> STOPPED_W<1> {
        STOPPED_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event RXDREADY"]
    #[inline(always)]
    #[must_use]
    pub fn rxdready(&mut self) -> RXDREADY_W<2> {
        RXDREADY_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event TXDSENT"]
    #[inline(always)]
    #[must_use]
    pub fn txdsent(&mut self) -> TXDSENT_W<7> {
        TXDSENT_W::new(self)
    }
    #[doc = "Bit 9 - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<9> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 14 - Write '1' to disable interrupt for event BB"]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BB_W<14> {
        BB_W::new(self)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event SUSPENDED"]
    #[inline(always)]
    #[must_use]
    pub fn suspended(&mut self) -> SUSPENDED_W<18> {
        SUSPENDED_W::new(self)
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
