#[doc = "Register `EVTEN` reader"]
pub struct R(crate::R<EVTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTEN` writer"]
pub struct W(crate::W<EVTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTEN_SPEC>;
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
impl From<crate::W<EVTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TICK` reader - Enable or disable event routing for event TICK"]
pub type TICK_R = crate::BitReader<TICK_A>;
#[doc = "Enable or disable event routing for event TICK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TICK_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `TICK` writer - Enable or disable event routing for event TICK"]
pub type TICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTEN_SPEC, TICK_A, O>;
impl<'a, const O: u8> TICK_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TICK_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TICK_A::ENABLED)
    }
}
#[doc = "Field `OVRFLW` reader - Enable or disable event routing for event OVRFLW"]
pub type OVRFLW_R = crate::BitReader<OVRFLW_A>;
#[doc = "Enable or disable event routing for event OVRFLW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRFLW_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `OVRFLW` writer - Enable or disable event routing for event OVRFLW"]
pub type OVRFLW_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTEN_SPEC, OVRFLW_A, O>;
impl<'a, const O: u8> OVRFLW_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRFLW_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRFLW_A::ENABLED)
    }
}
#[doc = "Field `COMPARE0` reader - Enable or disable event routing for event COMPARE\\[0\\]"]
pub type COMPARE0_R = crate::BitReader<COMPARE0_A>;
#[doc = "Enable or disable event routing for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `COMPARE0` writer - Enable or disable event routing for event COMPARE\\[0\\]"]
pub type COMPARE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTEN_SPEC, COMPARE0_A, O>;
impl<'a, const O: u8> COMPARE0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_A::ENABLED)
    }
}
#[doc = "Field `COMPARE1` reader - Enable or disable event routing for event COMPARE\\[1\\]"]
pub type COMPARE1_R = crate::BitReader<COMPARE1_A>;
#[doc = "Enable or disable event routing for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `COMPARE1` writer - Enable or disable event routing for event COMPARE\\[1\\]"]
pub type COMPARE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTEN_SPEC, COMPARE1_A, O>;
impl<'a, const O: u8> COMPARE1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_A::ENABLED)
    }
}
#[doc = "Field `COMPARE2` reader - Enable or disable event routing for event COMPARE\\[2\\]"]
pub type COMPARE2_R = crate::BitReader<COMPARE2_A>;
#[doc = "Enable or disable event routing for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `COMPARE2` writer - Enable or disable event routing for event COMPARE\\[2\\]"]
pub type COMPARE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTEN_SPEC, COMPARE2_A, O>;
impl<'a, const O: u8> COMPARE2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_A::ENABLED)
    }
}
#[doc = "Field `COMPARE3` reader - Enable or disable event routing for event COMPARE\\[3\\]"]
pub type COMPARE3_R = crate::BitReader<COMPARE3_A>;
#[doc = "Enable or disable event routing for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
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
#[doc = "Field `COMPARE3` writer - Enable or disable event routing for event COMPARE\\[3\\]"]
pub type COMPARE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTEN_SPEC, COMPARE3_A, O>;
impl<'a, const O: u8> COMPARE3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable event routing for event TICK"]
    #[inline(always)]
    pub fn tick(&self) -> TICK_R {
        TICK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable event routing for event OVRFLW"]
    #[inline(always)]
    pub fn ovrflw(&self) -> OVRFLW_R {
        OVRFLW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable or disable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> COMPARE0_R {
        COMPARE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> COMPARE1_R {
        COMPARE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable or disable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> COMPARE2_R {
        COMPARE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable or disable event routing for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> COMPARE3_R {
        COMPARE3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable event routing for event TICK"]
    #[inline(always)]
    #[must_use]
    pub fn tick(&mut self) -> TICK_W<0> {
        TICK_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable event routing for event OVRFLW"]
    #[inline(always)]
    #[must_use]
    pub fn ovrflw(&mut self) -> OVRFLW_W<1> {
        OVRFLW_W::new(self)
    }
    #[doc = "Bit 16 - Enable or disable event routing for event COMPARE\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare0(&mut self) -> COMPARE0_W<16> {
        COMPARE0_W::new(self)
    }
    #[doc = "Bit 17 - Enable or disable event routing for event COMPARE\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare1(&mut self) -> COMPARE1_W<17> {
        COMPARE1_W::new(self)
    }
    #[doc = "Bit 18 - Enable or disable event routing for event COMPARE\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare2(&mut self) -> COMPARE2_W<18> {
        COMPARE2_W::new(self)
    }
    #[doc = "Bit 19 - Enable or disable event routing for event COMPARE\\[3\\]"]
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
#[doc = "Enable or disable event routing\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evten](index.html) module"]
pub struct EVTEN_SPEC;
impl crate::RegisterSpec for EVTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evten::R](R) reader structure"]
impl crate::Readable for EVTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evten::W](W) writer structure"]
impl crate::Writable for EVTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EVTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
