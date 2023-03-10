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
#[doc = "Field `WRITE_SUSPEND` reader - Shortcut between event WRITE and task SUSPEND"]
pub type WRITE_SUSPEND_R = crate::BitReader<WRITE_SUSPEND_A>;
#[doc = "Shortcut between event WRITE and task SUSPEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRITE_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<WRITE_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl WRITE_SUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_SUSPEND_A {
        match self.bits {
            false => WRITE_SUSPEND_A::DISABLED,
            true => WRITE_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WRITE_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRITE_SUSPEND_A::ENABLED
    }
}
#[doc = "Field `WRITE_SUSPEND` writer - Shortcut between event WRITE and task SUSPEND"]
pub type WRITE_SUSPEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, WRITE_SUSPEND_A, O>;
impl<'a, const O: u8> WRITE_SUSPEND_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITE_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITE_SUSPEND_A::ENABLED)
    }
}
#[doc = "Field `READ_SUSPEND` reader - Shortcut between event READ and task SUSPEND"]
pub type READ_SUSPEND_R = crate::BitReader<READ_SUSPEND_A>;
#[doc = "Shortcut between event READ and task SUSPEND\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READ_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READ_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: READ_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl READ_SUSPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_SUSPEND_A {
        match self.bits {
            false => READ_SUSPEND_A::DISABLED,
            true => READ_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READ_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READ_SUSPEND_A::ENABLED
    }
}
#[doc = "Field `READ_SUSPEND` writer - Shortcut between event READ and task SUSPEND"]
pub type READ_SUSPEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, READ_SUSPEND_A, O>;
impl<'a, const O: u8> READ_SUSPEND_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READ_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READ_SUSPEND_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 13 - Shortcut between event WRITE and task SUSPEND"]
    #[inline(always)]
    pub fn write_suspend(&self) -> WRITE_SUSPEND_R {
        WRITE_SUSPEND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Shortcut between event READ and task SUSPEND"]
    #[inline(always)]
    pub fn read_suspend(&self) -> READ_SUSPEND_R {
        READ_SUSPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Shortcut between event WRITE and task SUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn write_suspend(&mut self) -> WRITE_SUSPEND_W<13> {
        WRITE_SUSPEND_W::new(self)
    }
    #[doc = "Bit 14 - Shortcut between event READ and task SUSPEND"]
    #[inline(always)]
    #[must_use]
    pub fn read_suspend(&mut self) -> READ_SUSPEND_W<14> {
        READ_SUSPEND_W::new(self)
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
