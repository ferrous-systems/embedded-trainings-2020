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
#[doc = "Field `TICK` reader - Write '1' to disable interrupt for event TICK"]
pub type TICK_R = crate::BitReader<TICK_A>;
#[doc = "Write '1' to disable interrupt for event TICK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICK_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TICK_A> for bool {
    #[inline(always)]
    fn from(variant: TICK_A) -> Self {
        variant as u8 != 0
    }
}
impl TICK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_A {
        match self.bits {
            false => TICK_A::DISABLED,
            true => TICK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TICK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TICK_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event TICK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICK_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TICK_AW> for bool {
    #[inline(always)]
    fn from(variant: TICK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICK` writer - Write '1' to disable interrupt for event TICK"]
pub type TICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, TICK_AW, O>;
impl<'a, const O: u8> TICK_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TICK_AW::CLEAR)
    }
}
#[doc = "Field `OVRFLW` reader - Write '1' to disable interrupt for event OVRFLW"]
pub type OVRFLW_R = crate::BitReader<OVRFLW_A>;
#[doc = "Write '1' to disable interrupt for event OVRFLW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRFLW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<OVRFLW_A> for bool {
    #[inline(always)]
    fn from(variant: OVRFLW_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRFLW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRFLW_A {
        match self.bits {
            false => OVRFLW_A::DISABLED,
            true => OVRFLW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRFLW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRFLW_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event OVRFLW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRFLW_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<OVRFLW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRFLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRFLW` writer - Write '1' to disable interrupt for event OVRFLW"]
pub type OVRFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, OVRFLW_AW, O>;
impl<'a, const O: u8> OVRFLW_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRFLW_AW::CLEAR)
    }
}
#[doc = "Field `COMPARE0` reader - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
pub type COMPARE0_R = crate::BitReader<COMPARE0_A>;
#[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE0_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_A {
        match self.bits {
            false => COMPARE0_A::DISABLED,
            true => COMPARE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE0_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` writer - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
pub type COMPARE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, COMPARE0_AW, O>;
impl<'a, const O: u8> COMPARE0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE0_AW::CLEAR)
    }
}
#[doc = "Field `COMPARE1` reader - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
pub type COMPARE1_R = crate::BitReader<COMPARE1_A>;
#[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE1_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_A {
        match self.bits {
            false => COMPARE1_A::DISABLED,
            true => COMPARE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE1_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` writer - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
pub type COMPARE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, COMPARE1_AW, O>;
impl<'a, const O: u8> COMPARE1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE1_AW::CLEAR)
    }
}
#[doc = "Field `COMPARE2` reader - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
pub type COMPARE2_R = crate::BitReader<COMPARE2_A>;
#[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE2_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_A {
        match self.bits {
            false => COMPARE2_A::DISABLED,
            true => COMPARE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE2_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE2_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` writer - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
pub type COMPARE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, COMPARE2_AW, O>;
impl<'a, const O: u8> COMPARE2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE2_AW::CLEAR)
    }
}
#[doc = "Field `COMPARE3` reader - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
pub type COMPARE3_R = crate::BitReader<COMPARE3_A>;
#[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE3_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_A {
        match self.bits {
            false => COMPARE3_A::DISABLED,
            true => COMPARE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE3_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<COMPARE3_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` writer - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
pub type COMPARE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, COMPARE3_AW, O>;
impl<'a, const O: u8> COMPARE3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COMPARE3_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TICK"]
    #[inline(always)]
    pub fn tick(&self) -> TICK_R {
        TICK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event OVRFLW"]
    #[inline(always)]
    pub fn ovrflw(&self) -> OVRFLW_R {
        OVRFLW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> COMPARE0_R {
        COMPARE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> COMPARE1_R {
        COMPARE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> COMPARE2_R {
        COMPARE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> COMPARE3_R {
        COMPARE3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TICK"]
    #[inline(always)]
    #[must_use]
    pub fn tick(&mut self) -> TICK_W<0> {
        TICK_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event OVRFLW"]
    #[inline(always)]
    #[must_use]
    pub fn ovrflw(&mut self) -> OVRFLW_W<1> {
        OVRFLW_W::new(self)
    }
    #[doc = "Bit 16 - Write '1' to disable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare0(&mut self) -> COMPARE0_W<16> {
        COMPARE0_W::new(self)
    }
    #[doc = "Bit 17 - Write '1' to disable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare1(&mut self) -> COMPARE1_W<17> {
        COMPARE1_W::new(self)
    }
    #[doc = "Bit 18 - Write '1' to disable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare2(&mut self) -> COMPARE2_W<18> {
        COMPARE2_W::new(self)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare3(&mut self) -> COMPARE3_W<19> {
        COMPARE3_W::new(self)
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
