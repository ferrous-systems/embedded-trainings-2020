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
#[doc = "Field `ENDRX_STARTRX` reader - Shortcut between event ENDRX and task STARTRX"]
pub type ENDRX_STARTRX_R = crate::BitReader<ENDRX_STARTRX_A>;
#[doc = "Shortcut between event ENDRX and task STARTRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDRX_STARTRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDRX_STARTRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_STARTRX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDRX_STARTRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_STARTRX_A {
        match self.bits {
            false => ENDRX_STARTRX_A::DISABLED,
            true => ENDRX_STARTRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_STARTRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_STARTRX_A::ENABLED
    }
}
#[doc = "Field `ENDRX_STARTRX` writer - Shortcut between event ENDRX and task STARTRX"]
pub type ENDRX_STARTRX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, ENDRX_STARTRX_A, O>;
impl<'a, const O: u8> ENDRX_STARTRX_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDRX_STARTRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDRX_STARTRX_A::ENABLED)
    }
}
#[doc = "Field `ENDRX_STOPRX` reader - Shortcut between event ENDRX and task STOPRX"]
pub type ENDRX_STOPRX_R = crate::BitReader<ENDRX_STOPRX_A>;
#[doc = "Shortcut between event ENDRX and task STOPRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDRX_STOPRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDRX_STOPRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_STOPRX_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDRX_STOPRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_STOPRX_A {
        match self.bits {
            false => ENDRX_STOPRX_A::DISABLED,
            true => ENDRX_STOPRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_STOPRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_STOPRX_A::ENABLED
    }
}
#[doc = "Field `ENDRX_STOPRX` writer - Shortcut between event ENDRX and task STOPRX"]
pub type ENDRX_STOPRX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, ENDRX_STOPRX_A, O>;
impl<'a, const O: u8> ENDRX_STOPRX_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDRX_STOPRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDRX_STOPRX_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 5 - Shortcut between event ENDRX and task STARTRX"]
    #[inline(always)]
    pub fn endrx_startrx(&self) -> ENDRX_STARTRX_R {
        ENDRX_STARTRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event ENDRX and task STOPRX"]
    #[inline(always)]
    pub fn endrx_stoprx(&self) -> ENDRX_STOPRX_R {
        ENDRX_STOPRX_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Shortcut between event ENDRX and task STARTRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx_startrx(&mut self) -> ENDRX_STARTRX_W<5> {
        ENDRX_STARTRX_W::new(self)
    }
    #[doc = "Bit 6 - Shortcut between event ENDRX and task STOPRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx_stoprx(&mut self) -> ENDRX_STOPRX_W<6> {
        ENDRX_STOPRX_W::new(self)
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
