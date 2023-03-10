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
#[doc = "Field `HFCLKSTARTED` reader - Write '1' to enable interrupt for event HFCLKSTARTED"]
pub type HFCLKSTARTED_R = crate::BitReader<HFCLKSTARTED_A>;
#[doc = "Write '1' to enable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<HFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl HFCLKSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFCLKSTARTED_A {
        match self.bits {
            false => HFCLKSTARTED_A::DISABLED,
            true => HFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HFCLKSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<HFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: HFCLKSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` writer - Write '1' to enable interrupt for event HFCLKSTARTED"]
pub type HFCLKSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, HFCLKSTARTED_AW, O>;
impl<'a, const O: u8> HFCLKSTARTED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(HFCLKSTARTED_AW::SET)
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Write '1' to enable interrupt for event LFCLKSTARTED"]
pub type LFCLKSTARTED_R = crate::BitReader<LFCLKSTARTED_A>;
#[doc = "Write '1' to enable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFCLKSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<LFCLKSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
impl LFCLKSTARTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCLKSTARTED_A {
        match self.bits {
            false => LFCLKSTARTED_A::DISABLED,
            true => LFCLKSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFCLKSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFCLKSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFCLKSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<LFCLKSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: LFCLKSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` writer - Write '1' to enable interrupt for event LFCLKSTARTED"]
pub type LFCLKSTARTED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTENSET_SPEC, LFCLKSTARTED_AW, O>;
impl<'a, const O: u8> LFCLKSTARTED_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(LFCLKSTARTED_AW::SET)
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
#[doc = "Field `CTTO` reader - Write '1' to enable interrupt for event CTTO"]
pub type CTTO_R = crate::BitReader<CTTO_A>;
#[doc = "Write '1' to enable interrupt for event CTTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTTO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTTO_A> for bool {
    #[inline(always)]
    fn from(variant: CTTO_A) -> Self {
        variant as u8 != 0
    }
}
impl CTTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTTO_A {
        match self.bits {
            false => CTTO_A::DISABLED,
            true => CTTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTTO_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event CTTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTTO_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CTTO_AW> for bool {
    #[inline(always)]
    fn from(variant: CTTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTO` writer - Write '1' to enable interrupt for event CTTO"]
pub type CTTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, CTTO_AW, O>;
impl<'a, const O: u8> CTTO_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CTTO_AW::SET)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HFCLKSTARTED_R {
        HFCLKSTARTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LFCLKSTARTED_R {
        LFCLKSTARTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CTTO"]
    #[inline(always)]
    pub fn ctto(&self) -> CTTO_R {
        CTTO_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    #[must_use]
    pub fn hfclkstarted(&mut self) -> HFCLKSTARTED_W<0> {
        HFCLKSTARTED_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    #[must_use]
    pub fn lfclkstarted(&mut self) -> LFCLKSTARTED_W<1> {
        LFCLKSTARTED_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<3> {
        DONE_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CTTO"]
    #[inline(always)]
    #[must_use]
    pub fn ctto(&mut self) -> CTTO_W<4> {
        CTTO_W::new(self)
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
