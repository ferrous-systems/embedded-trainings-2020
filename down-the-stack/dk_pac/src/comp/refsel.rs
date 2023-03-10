#[doc = "Register `REFSEL` reader"]
pub struct R(crate::R<REFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REFSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REFSEL` writer"]
pub struct W(crate::W<REFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REFSEL_SPEC>;
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
impl From<crate::W<REFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REFSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFSEL` reader - Reference select"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Reference select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    INT1V2 = 0,
    #[doc = "1: VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT1V8 = 1,
    #[doc = "2: VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT2V4 = 2,
    #[doc = "4: VREF = VDD"]
    VDD = 4,
    #[doc = "7: VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    AREF = 7,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::INT1V2),
            1 => Some(REFSEL_A::INT1V8),
            2 => Some(REFSEL_A::INT2V4),
            4 => Some(REFSEL_A::VDD),
            7 => Some(REFSEL_A::AREF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INT1V2`"]
    #[inline(always)]
    pub fn is_int1v2(&self) -> bool {
        *self == REFSEL_A::INT1V2
    }
    #[doc = "Checks if the value of the field is `INT1V8`"]
    #[inline(always)]
    pub fn is_int1v8(&self) -> bool {
        *self == REFSEL_A::INT1V8
    }
    #[doc = "Checks if the value of the field is `INT2V4`"]
    #[inline(always)]
    pub fn is_int2v4(&self) -> bool {
        *self == REFSEL_A::INT2V4
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFSEL_A::AREF
    }
}
#[doc = "Field `REFSEL` writer - Reference select"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REFSEL_SPEC, u8, REFSEL_A, 3, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    #[inline(always)]
    pub fn int1v2(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1V2)
    }
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn int1v8(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1V8)
    }
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn int2v4(self) -> &'a mut W {
        self.variant(REFSEL_A::INT2V4)
    }
    #[doc = "VREF = VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSEL_A::AREF)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reference source select for single-ended mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refsel](index.html) module"]
pub struct REFSEL_SPEC;
impl crate::RegisterSpec for REFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [refsel::R](R) reader structure"]
impl crate::Readable for REFSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [refsel::W](W) writer structure"]
impl crate::Writable for REFSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REFSEL to value 0x04"]
impl crate::Resettable for REFSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
