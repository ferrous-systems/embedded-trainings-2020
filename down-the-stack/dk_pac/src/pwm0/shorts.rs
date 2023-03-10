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
#[doc = "Field `SEQEND0_STOP` reader - Shortcut between event SEQEND\\[0\\]
and task STOP"]
pub type SEQEND0_STOP_R = crate::BitReader<SEQEND0_STOP_A>;
#[doc = "Shortcut between event SEQEND\\[0\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQEND0_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SEQEND0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQEND0_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND0_STOP_A {
        match self.bits {
            false => SEQEND0_STOP_A::DISABLED,
            true => SEQEND0_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND0_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND0_STOP_A::ENABLED
    }
}
#[doc = "Field `SEQEND0_STOP` writer - Shortcut between event SEQEND\\[0\\]
and task STOP"]
pub type SEQEND0_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, SEQEND0_STOP_A, O>;
impl<'a, const O: u8> SEQEND0_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOP_A::ENABLED)
    }
}
#[doc = "Field `SEQEND1_STOP` reader - Shortcut between event SEQEND\\[1\\]
and task STOP"]
pub type SEQEND1_STOP_R = crate::BitReader<SEQEND1_STOP_A>;
#[doc = "Shortcut between event SEQEND\\[1\\]
and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEQEND1_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SEQEND1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl SEQEND1_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND1_STOP_A {
        match self.bits {
            false => SEQEND1_STOP_A::DISABLED,
            true => SEQEND1_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND1_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND1_STOP_A::ENABLED
    }
}
#[doc = "Field `SEQEND1_STOP` writer - Shortcut between event SEQEND\\[1\\]
and task STOP"]
pub type SEQEND1_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, SEQEND1_STOP_A, O>;
impl<'a, const O: u8> SEQEND1_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOP_A::ENABLED)
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART0` reader - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
pub type LOOPSDONE_SEQSTART0_R = crate::BitReader<LOOPSDONE_SEQSTART0_A>;
#[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPSDONE_SEQSTART0_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_SEQSTART0_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_SEQSTART0_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPSDONE_SEQSTART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_SEQSTART0_A {
        match self.bits {
            false => LOOPSDONE_SEQSTART0_A::DISABLED,
            true => LOOPSDONE_SEQSTART0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART0_A::ENABLED
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART0` writer - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
pub type LOOPSDONE_SEQSTART0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, LOOPSDONE_SEQSTART0_A, O>;
impl<'a, const O: u8> LOOPSDONE_SEQSTART0_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0_A::ENABLED)
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART1` reader - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
pub type LOOPSDONE_SEQSTART1_R = crate::BitReader<LOOPSDONE_SEQSTART1_A>;
#[doc = "Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPSDONE_SEQSTART1_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_SEQSTART1_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_SEQSTART1_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPSDONE_SEQSTART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_SEQSTART1_A {
        match self.bits {
            false => LOOPSDONE_SEQSTART1_A::DISABLED,
            true => LOOPSDONE_SEQSTART1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART1_A::ENABLED
    }
}
#[doc = "Field `LOOPSDONE_SEQSTART1` writer - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
pub type LOOPSDONE_SEQSTART1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, LOOPSDONE_SEQSTART1_A, O>;
impl<'a, const O: u8> LOOPSDONE_SEQSTART1_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1_A::ENABLED)
    }
}
#[doc = "Field `LOOPSDONE_STOP` reader - Shortcut between event LOOPSDONE and task STOP"]
pub type LOOPSDONE_STOP_R = crate::BitReader<LOOPSDONE_STOP_A>;
#[doc = "Shortcut between event LOOPSDONE and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPSDONE_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPSDONE_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_STOP_A {
        match self.bits {
            false => LOOPSDONE_STOP_A::DISABLED,
            true => LOOPSDONE_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_STOP_A::ENABLED
    }
}
#[doc = "Field `LOOPSDONE_STOP` writer - Shortcut between event LOOPSDONE and task STOP"]
pub type LOOPSDONE_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, LOOPSDONE_STOP_A, O>;
impl<'a, const O: u8> LOOPSDONE_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[0\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend0_stop(&self) -> SEQEND0_STOP_R {
        SEQEND0_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[1\\]
and task STOP"]
    #[inline(always)]
    pub fn seqend1_stop(&self) -> SEQEND1_STOP_R {
        SEQEND1_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart0(&self) -> LOOPSDONE_SEQSTART0_R {
        LOOPSDONE_SEQSTART0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
    #[inline(always)]
    pub fn loopsdone_seqstart1(&self) -> LOOPSDONE_SEQSTART1_R {
        LOOPSDONE_SEQSTART1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    pub fn loopsdone_stop(&self) -> LOOPSDONE_STOP_R {
        LOOPSDONE_STOP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event SEQEND\\[0\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn seqend0_stop(&mut self) -> SEQEND0_STOP_W<0> {
        SEQEND0_STOP_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event SEQEND\\[1\\]
and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn seqend1_stop(&mut self) -> SEQEND1_STOP_W<1> {
        SEQEND1_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Shortcut between event LOOPSDONE and task SEQSTART\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_seqstart0(&mut self) -> LOOPSDONE_SEQSTART0_W<2> {
        LOOPSDONE_SEQSTART0_W::new(self)
    }
    #[doc = "Bit 3 - Shortcut between event LOOPSDONE and task SEQSTART\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_seqstart1(&mut self) -> LOOPSDONE_SEQSTART1_W<3> {
        LOOPSDONE_SEQSTART1_W::new(self)
    }
    #[doc = "Bit 4 - Shortcut between event LOOPSDONE and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn loopsdone_stop(&mut self) -> LOOPSDONE_STOP_W<4> {
        LOOPSDONE_STOP_W::new(self)
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
