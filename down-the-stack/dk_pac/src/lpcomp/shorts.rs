#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY_SAMPLE` reader - Shortcut between event READY and task SAMPLE"]
pub type READY_SAMPLE_R = crate::BitReader<READY_SAMPLE_A>;
#[doc = "Shortcut between event READY and task SAMPLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_SAMPLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: READY_SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_SAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_SAMPLE_A {
        match self.bits {
            false => READY_SAMPLE_A::DISABLED,
            true => READY_SAMPLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_SAMPLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_SAMPLE_A::ENABLED
    }
}
#[doc = "Field `READY_SAMPLE` writer - Shortcut between event READY and task SAMPLE"]
pub type READY_SAMPLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, READY_SAMPLE_A, O>;
impl<'a, const O: u8> READY_SAMPLE_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_SAMPLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_SAMPLE_A::ENABLED)
    }
}
#[doc = "Field `READY_STOP` reader - Shortcut between event READY and task STOP"]
pub type READY_STOP_R = crate::BitReader<READY_STOP_A>;
#[doc = "Shortcut between event READY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: READY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_STOP_A {
        match self.bits {
            false => READY_STOP_A::DISABLED,
            true => READY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_STOP_A::ENABLED
    }
}
#[doc = "Field `READY_STOP` writer - Shortcut between event READY and task STOP"]
pub type READY_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, READY_STOP_A, O>;
impl<'a, const O: u8> READY_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_STOP_A::ENABLED)
    }
}
#[doc = "Field `DOWN_STOP` reader - Shortcut between event DOWN and task STOP"]
pub type DOWN_STOP_R = crate::BitReader<DOWN_STOP_A>;
#[doc = "Shortcut between event DOWN and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWN_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DOWN_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DOWN_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOWN_STOP_A {
        match self.bits {
            false => DOWN_STOP_A::DISABLED,
            true => DOWN_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWN_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWN_STOP_A::ENABLED
    }
}
#[doc = "Field `DOWN_STOP` writer - Shortcut between event DOWN and task STOP"]
pub type DOWN_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, DOWN_STOP_A, O>;
impl<'a, const O: u8> DOWN_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOWN_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOWN_STOP_A::ENABLED)
    }
}
#[doc = "Field `UP_STOP` reader - Shortcut between event UP and task STOP"]
pub type UP_STOP_R = crate::BitReader<UP_STOP_A>;
#[doc = "Shortcut between event UP and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UP_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<UP_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl UP_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_STOP_A {
        match self.bits {
            false => UP_STOP_A::DISABLED,
            true => UP_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UP_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UP_STOP_A::ENABLED
    }
}
#[doc = "Field `UP_STOP` writer - Shortcut between event UP and task STOP"]
pub type UP_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, UP_STOP_A, O>;
impl<'a, const O: u8> UP_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UP_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UP_STOP_A::ENABLED)
    }
}
#[doc = "Field `CROSS_STOP` reader - Shortcut between event CROSS and task STOP"]
pub type CROSS_STOP_R = crate::BitReader<CROSS_STOP_A>;
#[doc = "Shortcut between event CROSS and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CROSS_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<CROSS_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: CROSS_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl CROSS_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CROSS_STOP_A {
        match self.bits {
            false => CROSS_STOP_A::DISABLED,
            true => CROSS_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CROSS_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CROSS_STOP_A::ENABLED
    }
}
#[doc = "Field `CROSS_STOP` writer - Shortcut between event CROSS and task STOP"]
pub type CROSS_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, CROSS_STOP_A, O>;
impl<'a, const O: u8> CROSS_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CROSS_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CROSS_STOP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event READY and task SAMPLE"]
    #[inline(always)]
    pub fn ready_sample(&self) -> READY_SAMPLE_R {
        READY_SAMPLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event READY and task STOP"]
    #[inline(always)]
    pub fn ready_stop(&self) -> READY_STOP_R {
        READY_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event DOWN and task STOP"]
    #[inline(always)]
    pub fn down_stop(&self) -> DOWN_STOP_R {
        DOWN_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event UP and task STOP"]
    #[inline(always)]
    pub fn up_stop(&self) -> UP_STOP_R {
        UP_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event CROSS and task STOP"]
    #[inline(always)]
    pub fn cross_stop(&self) -> CROSS_STOP_R {
        CROSS_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event READY and task SAMPLE"]
    #[inline(always)]
    #[must_use]
    pub fn ready_sample(&mut self) -> READY_SAMPLE_W<0> {
        READY_SAMPLE_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event READY and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn ready_stop(&mut self) -> READY_STOP_W<1> {
        READY_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Shortcut between event DOWN and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn down_stop(&mut self) -> DOWN_STOP_W<2> {
        DOWN_STOP_W::new(self)
    }
    #[doc = "Bit 3 - Shortcut between event UP and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn up_stop(&mut self) -> UP_STOP_W<3> {
        UP_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Shortcut between event CROSS and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn cross_stop(&mut self) -> CROSS_STOP_W<4> {
        CROSS_STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
