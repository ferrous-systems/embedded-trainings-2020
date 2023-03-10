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
#[doc = "Field `SEQSTARTED0` reader - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
pub type SEQSTARTED0_R = crate::BitReader<SEQSTARTED0_A>;
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQSTARTED0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQSTARTED0_A> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED0_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQSTARTED0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQSTARTED0_A {
        match self.bits {
            false => SEQSTARTED0_A::DISABLED,
            true => SEQSTARTED0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQSTARTED0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQSTARTED0_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQSTARTED0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQSTARTED0_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED0` writer - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
pub type SEQSTARTED0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, SEQSTARTED0_AW, O>;
impl<'a, const O: u8> SEQSTARTED0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQSTARTED0_AW::CLEAR)
    }
}
#[doc = "Field `SEQSTARTED1` reader - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
pub type SEQSTARTED1_R = crate::BitReader<SEQSTARTED1_A>;
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQSTARTED1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQSTARTED1_A> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED1_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQSTARTED1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQSTARTED1_A {
        match self.bits {
            false => SEQSTARTED1_A::DISABLED,
            true => SEQSTARTED1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQSTARTED1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQSTARTED1_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event SEQSTARTED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQSTARTED1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQSTARTED1_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQSTARTED1` writer - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
pub type SEQSTARTED1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, SEQSTARTED1_AW, O>;
impl<'a, const O: u8> SEQSTARTED1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQSTARTED1_AW::CLEAR)
    }
}
#[doc = "Field `SEQEND0` reader - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
pub type SEQEND0_R = crate::BitReader<SEQEND0_A>;
#[doc = "Write '1' to disable interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQEND0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQEND0_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQEND0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND0_A {
        match self.bits {
            false => SEQEND0_A::DISABLED,
            true => SEQEND0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND0_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event SEQEND\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQEND0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQEND0_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND0` writer - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
pub type SEQEND0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SEQEND0_AW, O>;
impl<'a, const O: u8> SEQEND0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQEND0_AW::CLEAR)
    }
}
#[doc = "Field `SEQEND1` reader - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
pub type SEQEND1_R = crate::BitReader<SEQEND1_A>;
#[doc = "Write '1' to disable interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQEND1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQEND1_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQEND1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND1_A {
        match self.bits {
            false => SEQEND1_A::DISABLED,
            true => SEQEND1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND1_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event SEQEND\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQEND1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQEND1_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQEND1` writer - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
pub type SEQEND1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, SEQEND1_AW, O>;
impl<'a, const O: u8> SEQEND1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQEND1_AW::CLEAR)
    }
}
#[doc = "Field `PWMPERIODEND` reader - Write '1' to disable interrupt for event PWMPERIODEND"]
pub type PWMPERIODEND_R = crate::BitReader<PWMPERIODEND_A>;
#[doc = "Write '1' to disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMPERIODEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PWMPERIODEND_A> for bool {
    #[inline(always)]
    fn from(variant: PWMPERIODEND_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMPERIODEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMPERIODEND_A {
        match self.bits {
            false => PWMPERIODEND_A::DISABLED,
            true => PWMPERIODEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMPERIODEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMPERIODEND_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event PWMPERIODEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMPERIODEND_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PWMPERIODEND_AW> for bool {
    #[inline(always)]
    fn from(variant: PWMPERIODEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMPERIODEND` writer - Write '1' to disable interrupt for event PWMPERIODEND"]
pub type PWMPERIODEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENCLR_SPEC, PWMPERIODEND_AW, O>;
impl<'a, const O: u8> PWMPERIODEND_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PWMPERIODEND_AW::CLEAR)
    }
}
#[doc = "Field `LOOPSDONE` reader - Write '1' to disable interrupt for event LOOPSDONE"]
pub type LOOPSDONE_R = crate::BitReader<LOOPSDONE_A>;
#[doc = "Write '1' to disable interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPSDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<LOOPSDONE_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPSDONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_A {
        match self.bits {
            false => LOOPSDONE_A::DISABLED,
            true => LOOPSDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event LOOPSDONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPSDONE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<LOOPSDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPSDONE` writer - Write '1' to disable interrupt for event LOOPSDONE"]
pub type LOOPSDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, LOOPSDONE_AW, O>;
impl<'a, const O: u8> LOOPSDONE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LOOPSDONE_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    pub fn seqstarted0(&self) -> SEQSTARTED0_R {
        SEQSTARTED0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    pub fn seqstarted1(&self) -> SEQSTARTED1_R {
        SEQSTARTED1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    pub fn seqend0(&self) -> SEQEND0_R {
        SEQEND0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    pub fn seqend1(&self) -> SEQEND1_R {
        SEQEND1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PWMPERIODEND_R {
        PWMPERIODEND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    pub fn loopsdone(&self) -> LOOPSDONE_R {
        LOOPSDONE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> STOPPED_W<1> {
        STOPPED_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event SEQSTARTED\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqstarted0(&mut self) -> SEQSTARTED0_W<2> {
        SEQSTARTED0_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event SEQSTARTED\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqstarted1(&mut self) -> SEQSTARTED1_W<3> {
        SEQSTARTED1_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event SEQEND\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqend0(&mut self) -> SEQEND0_W<4> {
        SEQEND0_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event SEQEND\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn seqend1(&mut self) -> SEQEND1_W<5> {
        SEQEND1_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event PWMPERIODEND"]
    #[inline(always)]
    #[must_use]
    pub fn pwmperiodend(&mut self) -> PWMPERIODEND_W<6> {
        PWMPERIODEND_W::new(self)
    }
    #[doc = "Bit 7 - Write '1' to disable interrupt for event LOOPSDONE"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone(&mut self) -> LOOPSDONE_W<7> {
        LOOPSDONE_W::new(self)
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
