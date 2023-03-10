#[doc = "Register `POWER` reader"]
pub struct R(crate::R<POWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWER` writer"]
pub struct W(crate::W<POWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWER_SPEC>;
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
impl From<crate::W<POWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER` reader - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
pub type POWER_R = crate::BitReader<POWER_A>;
#[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWER_A {
    #[doc = "0: Peripheral is powered off"]
    DISABLED = 0,
    #[doc = "1: Peripheral is powered on"]
    ENABLED = 1,
}
impl From<POWER_A> for bool {
    #[inline(always)]
    fn from(variant: POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_A {
        match self.bits {
            false => POWER_A::DISABLED,
            true => POWER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POWER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POWER_A::ENABLED
    }
}
#[doc = "Field `POWER` writer - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
pub type POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, POWER_A, O>;
impl<'a, const O: u8> POWER_W<'a, O> {
    #[doc = "Peripheral is powered off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POWER_A::DISABLED)
    }
    #[doc = "Peripheral is powered on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POWER_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    #[must_use]
    pub fn power(&mut self) -> POWER_W<0> {
        POWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](index.html) module"]
pub struct POWER_SPEC;
impl crate::RegisterSpec for POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [power::R](R) reader structure"]
impl crate::Readable for POWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [power::W](W) writer structure"]
impl crate::Writable for POWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POWER to value 0x01"]
impl crate::Resettable for POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
