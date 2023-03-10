#[doc = "Register `MODECNF0` reader"]
pub struct R(crate::R<MODECNF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODECNF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODECNF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODECNF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODECNF0` writer"]
pub struct W(crate::W<MODECNF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODECNF0_SPEC>;
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
impl From<crate::W<MODECNF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODECNF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RU` reader - Radio ramp-up time"]
pub type RU_R = crate::BitReader<RU_A>;
#[doc = "Radio ramp-up time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RU_A {
    #[doc = "0: Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    DEFAULT = 0,
    #[doc = "1: Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    FAST = 1,
}
impl From<RU_A> for bool {
    #[inline(always)]
    fn from(variant: RU_A) -> Self {
        variant as u8 != 0
    }
}
impl RU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RU_A {
        match self.bits {
            false => RU_A::DEFAULT,
            true => RU_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RU_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == RU_A::FAST
    }
}
#[doc = "Field `RU` writer - Radio ramp-up time"]
pub type RU_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODECNF0_SPEC, RU_A, O>;
impl<'a, const O: u8> RU_W<'a, O> {
    #[doc = "Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RU_A::DEFAULT)
    }
    #[doc = "Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(RU_A::FAST)
    }
}
#[doc = "Field `DTX` reader - Default TX value"]
pub type DTX_R = crate::FieldReader<u8, DTX_A>;
#[doc = "Default TX value\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTX_A {
    #[doc = "0: Transmit '1'"]
    B1 = 0,
    #[doc = "1: Transmit '0'"]
    B0 = 1,
    #[doc = "2: Transmit center frequency"]
    CENTER = 2,
}
impl From<DTX_A> for u8 {
    #[inline(always)]
    fn from(variant: DTX_A) -> Self {
        variant as _
    }
}
impl DTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTX_A> {
        match self.bits {
            0 => Some(DTX_A::B1),
            1 => Some(DTX_A::B0),
            2 => Some(DTX_A::CENTER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DTX_A::B1
    }
    #[doc = "Checks if the value of the field is `B0`"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DTX_A::B0
    }
    #[doc = "Checks if the value of the field is `CENTER`"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == DTX_A::CENTER
    }
}
#[doc = "Field `DTX` writer - Default TX value"]
pub type DTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODECNF0_SPEC, u8, DTX_A, 2, O>;
impl<'a, const O: u8> DTX_W<'a, O> {
    #[doc = "Transmit '1'"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut W {
        self.variant(DTX_A::B1)
    }
    #[doc = "Transmit '0'"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut W {
        self.variant(DTX_A::B0)
    }
    #[doc = "Transmit center frequency"]
    #[inline(always)]
    pub fn center(self) -> &'a mut W {
        self.variant(DTX_A::CENTER)
    }
}
impl R {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    pub fn dtx(&self) -> DTX_R {
        DTX_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    #[must_use]
    pub fn ru(&mut self) -> RU_W<0> {
        RU_W::new(self)
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    #[must_use]
    pub fn dtx(&mut self) -> DTX_W<8> {
        DTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Radio mode configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modecnf0](index.html) module"]
pub struct MODECNF0_SPEC;
impl crate::RegisterSpec for MODECNF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modecnf0::R](R) reader structure"]
impl crate::Readable for MODECNF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modecnf0::W](W) writer structure"]
impl crate::Writable for MODECNF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODECNF0 to value 0x0200"]
impl crate::Resettable for MODECNF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
