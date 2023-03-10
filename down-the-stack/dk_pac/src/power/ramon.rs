#[doc = "Register `RAMON` reader"]
pub struct R(crate::R<RAMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMON` writer"]
pub struct W(crate::W<RAMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMON_SPEC>;
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
impl From<crate::W<RAMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ONRAM0` reader - Keep RAM block 0 on or off in system ON Mode"]
pub type ONRAM0_R = crate::BitReader<ONRAM0_A>;
#[doc = "Keep RAM block 0 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONRAM0_A {
    #[doc = "0: Off"]
    RAM0OFF = 0,
    #[doc = "1: On"]
    RAM0ON = 1,
}
impl From<ONRAM0_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM0_A) -> Self {
        variant as u8 != 0
    }
}
impl ONRAM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM0_A {
        match self.bits {
            false => ONRAM0_A::RAM0OFF,
            true => ONRAM0_A::RAM0ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0OFF`"]
    #[inline(always)]
    pub fn is_ram0off(&self) -> bool {
        *self == ONRAM0_A::RAM0OFF
    }
    #[doc = "Checks if the value of the field is `RAM0ON`"]
    #[inline(always)]
    pub fn is_ram0on(&self) -> bool {
        *self == ONRAM0_A::RAM0ON
    }
}
#[doc = "Field `ONRAM0` writer - Keep RAM block 0 on or off in system ON Mode"]
pub type ONRAM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMON_SPEC, ONRAM0_A, O>;
impl<'a, const O: u8> ONRAM0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram0off(self) -> &'a mut W {
        self.variant(ONRAM0_A::RAM0OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram0on(self) -> &'a mut W {
        self.variant(ONRAM0_A::RAM0ON)
    }
}
#[doc = "Field `ONRAM1` reader - Keep RAM block 1 on or off in system ON Mode"]
pub type ONRAM1_R = crate::BitReader<ONRAM1_A>;
#[doc = "Keep RAM block 1 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONRAM1_A {
    #[doc = "0: Off"]
    RAM1OFF = 0,
    #[doc = "1: On"]
    RAM1ON = 1,
}
impl From<ONRAM1_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM1_A) -> Self {
        variant as u8 != 0
    }
}
impl ONRAM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM1_A {
        match self.bits {
            false => ONRAM1_A::RAM1OFF,
            true => ONRAM1_A::RAM1ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM1OFF`"]
    #[inline(always)]
    pub fn is_ram1off(&self) -> bool {
        *self == ONRAM1_A::RAM1OFF
    }
    #[doc = "Checks if the value of the field is `RAM1ON`"]
    #[inline(always)]
    pub fn is_ram1on(&self) -> bool {
        *self == ONRAM1_A::RAM1ON
    }
}
#[doc = "Field `ONRAM1` writer - Keep RAM block 1 on or off in system ON Mode"]
pub type ONRAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMON_SPEC, ONRAM1_A, O>;
impl<'a, const O: u8> ONRAM1_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram1off(self) -> &'a mut W {
        self.variant(ONRAM1_A::RAM1OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram1on(self) -> &'a mut W {
        self.variant(ONRAM1_A::RAM1ON)
    }
}
#[doc = "Field `OFFRAM0` reader - Keep retention on RAM block 0 when RAM block is switched off"]
pub type OFFRAM0_R = crate::BitReader<OFFRAM0_A>;
#[doc = "Keep retention on RAM block 0 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFRAM0_A {
    #[doc = "0: Off"]
    RAM0OFF = 0,
    #[doc = "1: On"]
    RAM0ON = 1,
}
impl From<OFFRAM0_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM0_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFRAM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM0_A {
        match self.bits {
            false => OFFRAM0_A::RAM0OFF,
            true => OFFRAM0_A::RAM0ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0OFF`"]
    #[inline(always)]
    pub fn is_ram0off(&self) -> bool {
        *self == OFFRAM0_A::RAM0OFF
    }
    #[doc = "Checks if the value of the field is `RAM0ON`"]
    #[inline(always)]
    pub fn is_ram0on(&self) -> bool {
        *self == OFFRAM0_A::RAM0ON
    }
}
#[doc = "Field `OFFRAM0` writer - Keep retention on RAM block 0 when RAM block is switched off"]
pub type OFFRAM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMON_SPEC, OFFRAM0_A, O>;
impl<'a, const O: u8> OFFRAM0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram0off(self) -> &'a mut W {
        self.variant(OFFRAM0_A::RAM0OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram0on(self) -> &'a mut W {
        self.variant(OFFRAM0_A::RAM0ON)
    }
}
#[doc = "Field `OFFRAM1` reader - Keep retention on RAM block 1 when RAM block is switched off"]
pub type OFFRAM1_R = crate::BitReader<OFFRAM1_A>;
#[doc = "Keep retention on RAM block 1 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFRAM1_A {
    #[doc = "0: Off"]
    RAM1OFF = 0,
    #[doc = "1: On"]
    RAM1ON = 1,
}
impl From<OFFRAM1_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM1_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFRAM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM1_A {
        match self.bits {
            false => OFFRAM1_A::RAM1OFF,
            true => OFFRAM1_A::RAM1ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM1OFF`"]
    #[inline(always)]
    pub fn is_ram1off(&self) -> bool {
        *self == OFFRAM1_A::RAM1OFF
    }
    #[doc = "Checks if the value of the field is `RAM1ON`"]
    #[inline(always)]
    pub fn is_ram1on(&self) -> bool {
        *self == OFFRAM1_A::RAM1ON
    }
}
#[doc = "Field `OFFRAM1` writer - Keep retention on RAM block 1 when RAM block is switched off"]
pub type OFFRAM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAMON_SPEC, OFFRAM1_A, O>;
impl<'a, const O: u8> OFFRAM1_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram1off(self) -> &'a mut W {
        self.variant(OFFRAM1_A::RAM1OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram1on(self) -> &'a mut W {
        self.variant(OFFRAM1_A::RAM1ON)
    }
}
impl R {
    #[doc = "Bit 0 - Keep RAM block 0 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram0(&self) -> ONRAM0_R {
        ONRAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Keep RAM block 1 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram1(&self) -> ONRAM1_R {
        ONRAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM block 0 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram0(&self) -> OFFRAM0_R {
        OFFRAM0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM block 1 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram1(&self) -> OFFRAM1_R {
        OFFRAM1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM block 0 on or off in system ON Mode"]
    #[inline(always)]
    #[must_use]
    pub fn onram0(&mut self) -> ONRAM0_W<0> {
        ONRAM0_W::new(self)
    }
    #[doc = "Bit 1 - Keep RAM block 1 on or off in system ON Mode"]
    #[inline(always)]
    #[must_use]
    pub fn onram1(&mut self) -> ONRAM1_W<1> {
        ONRAM1_W::new(self)
    }
    #[doc = "Bit 16 - Keep retention on RAM block 0 when RAM block is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn offram0(&mut self) -> OFFRAM0_W<16> {
        OFFRAM0_W::new(self)
    }
    #[doc = "Bit 17 - Keep retention on RAM block 1 when RAM block is switched off"]
    #[inline(always)]
    #[must_use]
    pub fn offram1(&mut self) -> OFFRAM1_W<17> {
        OFFRAM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Deprecated register - RAM on/off register (this register is retained)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramon](index.html) module"]
pub struct RAMON_SPEC;
impl crate::RegisterSpec for RAMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramon::R](R) reader structure"]
impl crate::Readable for RAMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramon::W](W) writer structure"]
impl crate::Writable for RAMON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAMON to value 0x03"]
impl crate::Resettable for RAMON_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
