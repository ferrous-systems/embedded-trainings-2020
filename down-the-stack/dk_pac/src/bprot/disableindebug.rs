#[doc = "Register `DISABLEINDEBUG` reader"]
pub struct R(crate::R<DISABLEINDEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISABLEINDEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISABLEINDEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISABLEINDEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISABLEINDEBUG` writer"]
pub struct W(crate::W<DISABLEINDEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISABLEINDEBUG_SPEC>;
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
impl From<crate::W<DISABLEINDEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISABLEINDEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISABLEINDEBUG` reader - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
pub type DISABLEINDEBUG_R = crate::BitReader<DISABLEINDEBUG_A>;
#[doc = "Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLEINDEBUG_A {
    #[doc = "1: Disable in debug"]
    DISABLED = 1,
    #[doc = "0: Enable in debug"]
    ENABLED = 0,
}
impl From<DISABLEINDEBUG_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLEINDEBUG_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLEINDEBUG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLEINDEBUG_A {
        match self.bits {
            true => DISABLEINDEBUG_A::DISABLED,
            false => DISABLEINDEBUG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLEINDEBUG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLEINDEBUG_A::ENABLED
    }
}
#[doc = "Field `DISABLEINDEBUG` writer - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
pub type DISABLEINDEBUG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DISABLEINDEBUG_SPEC, DISABLEINDEBUG_A, O>;
impl<'a, const O: u8> DISABLEINDEBUG_W<'a, O> {
    #[doc = "Disable in debug"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLEINDEBUG_A::DISABLED)
    }
    #[doc = "Enable in debug"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLEINDEBUG_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
    #[inline(always)]
    pub fn disableindebug(&self) -> DISABLEINDEBUG_R {
        DISABLEINDEBUG_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
    #[inline(always)]
    #[must_use]
    pub fn disableindebug(&mut self) -> DISABLEINDEBUG_W<0> {
        DISABLEINDEBUG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable protection mechanism in debug interface mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [disableindebug](index.html) module"]
pub struct DISABLEINDEBUG_SPEC;
impl crate::RegisterSpec for DISABLEINDEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [disableindebug::R](R) reader structure"]
impl crate::Readable for DISABLEINDEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [disableindebug::W](W) writer structure"]
impl crate::Writable for DISABLEINDEBUG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DISABLEINDEBUG to value 0x01"]
impl crate::Resettable for DISABLEINDEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
