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
#[doc = "Field `BB_SUSPEND` reader - Shortcut between event BB and task SUSPEND"]
pub type BB_SUSPEND_R = crate::BitReader<BB_SUSPEND_A>;
#[doc = "Shortcut between event BB and task SUSPEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<BB_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: BB_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl BB_SUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_SUSPEND_A {
        match self.bits {
            false => BB_SUSPEND_A::DISABLED,
            true => BB_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BB_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BB_SUSPEND_A::ENABLED
    }
}
#[doc = "Field `BB_SUSPEND` writer - Shortcut between event BB and task SUSPEND"]
pub type BB_SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, BB_SUSPEND_A, O>;
impl<'a, const O: u8> BB_SUSPEND_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BB_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BB_SUSPEND_A::ENABLED)
    }
}
#[doc = "Field `BB_STOP` reader - Shortcut between event BB and task STOP"]
pub type BB_STOP_R = crate::BitReader<BB_STOP_A>;
#[doc = "Shortcut between event BB and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BB_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<BB_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: BB_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl BB_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_STOP_A {
        match self.bits {
            false => BB_STOP_A::DISABLED,
            true => BB_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BB_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BB_STOP_A::ENABLED
    }
}
#[doc = "Field `BB_STOP` writer - Shortcut between event BB and task STOP"]
pub type BB_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, BB_STOP_A, O>;
impl<'a, const O: u8> BB_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BB_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BB_STOP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event BB and task SUSPEND"]
    #[inline(always)]
    pub fn bb_suspend(&self) -> BB_SUSPEND_R {
        BB_SUSPEND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event BB and task STOP"]
    #[inline(always)]
    pub fn bb_stop(&self) -> BB_STOP_R {
        BB_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event BB and task SUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn bb_suspend(&mut self) -> BB_SUSPEND_W<0> {
        BB_SUSPEND_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event BB and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn bb_stop(&mut self) -> BB_STOP_W<1> {
        BB_STOP_W::new(self)
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
