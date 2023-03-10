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
#[doc = "Field `FIELDDETECTED_ACTIVATE` reader - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub type FIELDDETECTED_ACTIVATE_R = crate::BitReader<FIELDDETECTED_ACTIVATE_A>;
#[doc = "Shortcut between event FIELDDETECTED and task ACTIVATE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELDDETECTED_ACTIVATE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<FIELDDETECTED_ACTIVATE_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_ACTIVATE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELDDETECTED_ACTIVATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDDETECTED_ACTIVATE_A {
        match self.bits {
            false => FIELDDETECTED_ACTIVATE_A::DISABLED,
            true => FIELDDETECTED_ACTIVATE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDDETECTED_ACTIVATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDDETECTED_ACTIVATE_A::ENABLED
    }
}
#[doc = "Field `FIELDDETECTED_ACTIVATE` writer - Shortcut between event FIELDDETECTED and task ACTIVATE"]
pub type FIELDDETECTED_ACTIVATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, FIELDDETECTED_ACTIVATE_A, O>;
impl<'a, const O: u8> FIELDDETECTED_ACTIVATE_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDDETECTED_ACTIVATE_A::ENABLED)
    }
}
#[doc = "Field `FIELDLOST_SENSE` reader - Shortcut between event FIELDLOST and task SENSE"]
pub type FIELDLOST_SENSE_R = crate::BitReader<FIELDLOST_SENSE_A>;
#[doc = "Shortcut between event FIELDLOST and task SENSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIELDLOST_SENSE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<FIELDLOST_SENSE_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_SENSE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIELDLOST_SENSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDLOST_SENSE_A {
        match self.bits {
            false => FIELDLOST_SENSE_A::DISABLED,
            true => FIELDLOST_SENSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDLOST_SENSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDLOST_SENSE_A::ENABLED
    }
}
#[doc = "Field `FIELDLOST_SENSE` writer - Shortcut between event FIELDLOST and task SENSE"]
pub type FIELDLOST_SENSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, FIELDLOST_SENSE_A, O>;
impl<'a, const O: u8> FIELDLOST_SENSE_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIELDLOST_SENSE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    pub fn fielddetected_activate(&self) -> FIELDDETECTED_ACTIVATE_R {
        FIELDDETECTED_ACTIVATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    pub fn fieldlost_sense(&self) -> FIELDLOST_SENSE_R {
        FIELDLOST_SENSE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event FIELDDETECTED and task ACTIVATE"]
    #[inline(always)]
    #[must_use]
    pub fn fielddetected_activate(&mut self) -> FIELDDETECTED_ACTIVATE_W<0> {
        FIELDDETECTED_ACTIVATE_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event FIELDLOST and task SENSE"]
    #[inline(always)]
    #[must_use]
    pub fn fieldlost_sense(&mut self) -> FIELDLOST_SENSE_W<1> {
        FIELDLOST_SENSE_W::new(self)
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
