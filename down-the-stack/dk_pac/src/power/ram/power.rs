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
#[doc = "Field `S0POWER` reader - Keep RAM section S0 ON or OFF in System ON mode."]
pub type S0POWER_R = crate::BitReader<S0POWER_A>;
#[doc = "Keep RAM section S0 ON or OFF in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S0POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0POWER_A {
        match self.bits {
            false => S0POWER_A::OFF,
            true => S0POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0POWER_A::ON
    }
}
#[doc = "Field `S0POWER` writer - Keep RAM section S0 ON or OFF in System ON mode."]
pub type S0POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S0POWER_A, O>;
impl<'a, const O: u8> S0POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWER_A::ON)
    }
}
#[doc = "Field `S1POWER` reader - Keep RAM section S1 ON or OFF in System ON mode."]
pub type S1POWER_R = crate::BitReader<S1POWER_A>;
#[doc = "Keep RAM section S1 ON or OFF in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_A) -> Self {
        variant as u8 != 0
    }
}
impl S1POWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1POWER_A {
        match self.bits {
            false => S1POWER_A::OFF,
            true => S1POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1POWER_A::ON
    }
}
#[doc = "Field `S1POWER` writer - Keep RAM section S1 ON or OFF in System ON mode."]
pub type S1POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S1POWER_A, O>;
impl<'a, const O: u8> S1POWER_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWER_A::ON)
    }
}
#[doc = "Field `S0RETENTION` reader - Keep retention on RAM section S0 when RAM section is in OFF"]
pub type S0RETENTION_R = crate::BitReader<S0RETENTION_A>;
#[doc = "Keep retention on RAM section S0 when RAM section is in OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S0RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S0RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S0RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0RETENTION_A {
        match self.bits {
            false => S0RETENTION_A::OFF,
            true => S0RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0RETENTION_A::ON
    }
}
#[doc = "Field `S0RETENTION` writer - Keep retention on RAM section S0 when RAM section is in OFF"]
pub type S0RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S0RETENTION_A, O>;
impl<'a, const O: u8> S0RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTION_A::ON)
    }
}
#[doc = "Field `S1RETENTION` reader - Keep retention on RAM section S1 when RAM section is in OFF"]
pub type S1RETENTION_R = crate::BitReader<S1RETENTION_A>;
#[doc = "Keep retention on RAM section S1 when RAM section is in OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S1RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
impl S1RETENTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1RETENTION_A {
        match self.bits {
            false => S1RETENTION_A::OFF,
            true => S1RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1RETENTION_A::ON
    }
}
#[doc = "Field `S1RETENTION` writer - Keep retention on RAM section S1 when RAM section is in OFF"]
pub type S1RETENTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, POWER_SPEC, S1RETENTION_A, O>;
impl<'a, const O: u8> S1RETENTION_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTION_A::ON)
    }
}
impl R {
    #[doc = "Bit 0 - Keep RAM section S0 ON or OFF in System ON mode."]
    #[inline(always)]
    pub fn s0power(&self) -> S0POWER_R {
        S0POWER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 ON or OFF in System ON mode."]
    #[inline(always)]
    pub fn s1power(&self) -> S1POWER_R {
        S1POWER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is in OFF"]
    #[inline(always)]
    pub fn s0retention(&self) -> S0RETENTION_R {
        S0RETENTION_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is in OFF"]
    #[inline(always)]
    pub fn s1retention(&self) -> S1RETENTION_R {
        S1RETENTION_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 ON or OFF in System ON mode."]
    #[inline(always)]
    #[must_use]
    pub fn s0power(&mut self) -> S0POWER_W<0> {
        S0POWER_W::new(self)
    }
    #[doc = "Bit 1 - Keep RAM section S1 ON or OFF in System ON mode."]
    #[inline(always)]
    #[must_use]
    pub fn s1power(&mut self) -> S1POWER_W<1> {
        S1POWER_W::new(self)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is in OFF"]
    #[inline(always)]
    #[must_use]
    pub fn s0retention(&mut self) -> S0RETENTION_W<16> {
        S0RETENTION_W::new(self)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is in OFF"]
    #[inline(always)]
    #[must_use]
    pub fn s1retention(&mut self) -> S1RETENTION_W<17> {
        S1RETENTION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: RAMn power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](index.html) module"]
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
#[doc = "`reset()` method sets POWER to value 0xffff"]
impl crate::Resettable for POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
