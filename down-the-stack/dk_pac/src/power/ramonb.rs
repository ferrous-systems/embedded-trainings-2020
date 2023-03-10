#[doc = "Register `RAMONB` reader"]
pub struct R(crate::R<RAMONB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMONB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMONB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMONB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMONB` writer"]
pub struct W(crate::W<RAMONB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMONB_SPEC>;
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
impl From<crate::W<RAMONB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMONB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ONRAM2` reader - Keep RAM block 2 on or off in system ON Mode"]
pub type ONRAM2_R = crate::BitReader<ONRAM2_A>;
#[doc = "Keep RAM block 2 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONRAM2_A {
    #[doc = "0: Off"]
    RAM2OFF = 0,
    #[doc = "1: On"]
    RAM2ON = 1,
}
impl From<ONRAM2_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM2_A) -> Self {
        variant as u8 != 0
    }
}
impl ONRAM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM2_A {
        match self.bits {
            false => ONRAM2_A::RAM2OFF,
            true => ONRAM2_A::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        *self == ONRAM2_A::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        *self == ONRAM2_A::RAM2ON
    }
}
#[doc = "Field `ONRAM2` writer - Keep RAM block 2 on or off in system ON Mode"]
pub type ONRAM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMONB_SPEC, ONRAM2_A, O>;
impl<'a, const O: u8> ONRAM2_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(ONRAM2_A::RAM2OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(ONRAM2_A::RAM2ON)
    }
}
#[doc = "Field `ONRAM3` reader - Keep RAM block 3 on or off in system ON Mode"]
pub type ONRAM3_R = crate::BitReader<ONRAM3_A>;
#[doc = "Keep RAM block 3 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONRAM3_A {
    #[doc = "0: Off"]
    RAM3OFF = 0,
    #[doc = "1: On"]
    RAM3ON = 1,
}
impl From<ONRAM3_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM3_A) -> Self {
        variant as u8 != 0
    }
}
impl ONRAM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM3_A {
        match self.bits {
            false => ONRAM3_A::RAM3OFF,
            true => ONRAM3_A::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        *self == ONRAM3_A::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        *self == ONRAM3_A::RAM3ON
    }
}
#[doc = "Field `ONRAM3` writer - Keep RAM block 3 on or off in system ON Mode"]
pub type ONRAM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMONB_SPEC, ONRAM3_A, O>;
impl<'a, const O: u8> ONRAM3_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(ONRAM3_A::RAM3OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(ONRAM3_A::RAM3ON)
    }
}
#[doc = "Field `OFFRAM2` reader - Keep retention on RAM block 2 when RAM block is switched off"]
pub type OFFRAM2_R = crate::BitReader<OFFRAM2_A>;
#[doc = "Keep retention on RAM block 2 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFRAM2_A {
    #[doc = "0: Off"]
    RAM2OFF = 0,
    #[doc = "1: On"]
    RAM2ON = 1,
}
impl From<OFFRAM2_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM2_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFRAM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM2_A {
        match self.bits {
            false => OFFRAM2_A::RAM2OFF,
            true => OFFRAM2_A::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        *self == OFFRAM2_A::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        *self == OFFRAM2_A::RAM2ON
    }
}
#[doc = "Field `OFFRAM2` writer - Keep retention on RAM block 2 when RAM block is switched off"]
pub type OFFRAM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMONB_SPEC, OFFRAM2_A, O>;
impl<'a, const O: u8> OFFRAM2_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(OFFRAM2_A::RAM2OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(OFFRAM2_A::RAM2ON)
    }
}
#[doc = "Field `OFFRAM3` reader - Keep retention on RAM block 3 when RAM block is switched off"]
pub type OFFRAM3_R = crate::BitReader<OFFRAM3_A>;
#[doc = "Keep retention on RAM block 3 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFRAM3_A {
    #[doc = "0: Off"]
    RAM3OFF = 0,
    #[doc = "1: On"]
    RAM3ON = 1,
}
impl From<OFFRAM3_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM3_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFRAM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM3_A {
        match self.bits {
            false => OFFRAM3_A::RAM3OFF,
            true => OFFRAM3_A::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        *self == OFFRAM3_A::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        *self == OFFRAM3_A::RAM3ON
    }
}
#[doc = "Field `OFFRAM3` writer - Keep retention on RAM block 3 when RAM block is switched off"]
pub type OFFRAM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMONB_SPEC, OFFRAM3_A, O>;
impl<'a, const O: u8> OFFRAM3_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(OFFRAM3_A::RAM3OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(OFFRAM3_A::RAM3ON)
    }
}
impl R {
    #[doc = "Bit 0 - Keep RAM block 2 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram2(&self) -> ONRAM2_R {
        ONRAM2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep RAM block 3 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram3(&self) -> ONRAM3_R {
        ONRAM3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM block 2 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram2(&self) -> OFFRAM2_R {
        OFFRAM2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM block 3 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram3(&self) -> OFFRAM3_R {
        OFFRAM3_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM block 2 on or off in system ON Mode"]
    #[inline(always)]
    #[must_use]
    pub fn onram2(&mut self) -> ONRAM2_W<0> {
        ONRAM2_W::new(self)
    }
    #[doc = "Bit 1 - Keep RAM block 3 on or off in system ON Mode"]
    #[inline(always)]
    #[must_use]
    pub fn onram3(&mut self) -> ONRAM3_W<1> {
        ONRAM3_W::new(self)
    }
    #[doc = "Bit 16 - Keep retention on RAM block 2 when RAM block is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn offram2(&mut self) -> OFFRAM2_W<16> {
        OFFRAM2_W::new(self)
    }
    #[doc = "Bit 17 - Keep retention on RAM block 3 when RAM block is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn offram3(&mut self) -> OFFRAM3_W<17> {
        OFFRAM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deprecated register - RAM on/off register (this register is retained)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramonb](index.html) module"]
pub struct RAMONB_SPEC;
impl crate::RegisterSpec for RAMONB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramonb::R](R) reader structure"]
impl crate::Readable for RAMONB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramonb::W](W) writer structure"]
impl crate::Writable for RAMONB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMONB to value 0x03"]
impl crate::Resettable for RAMONB_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
