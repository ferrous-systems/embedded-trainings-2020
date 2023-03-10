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
    #[doc = "0: VDD * 1/8 selected as reference"]
    REF1_8VDD = 0,
    #[doc = "1: VDD * 2/8 selected as reference"]
    REF2_8VDD = 1,
    #[doc = "2: VDD * 3/8 selected as reference"]
    REF3_8VDD = 2,
    #[doc = "3: VDD * 4/8 selected as reference"]
    REF4_8VDD = 3,
    #[doc = "4: VDD * 5/8 selected as reference"]
    REF5_8VDD = 4,
    #[doc = "5: VDD * 6/8 selected as reference"]
    REF6_8VDD = 5,
    #[doc = "6: VDD * 7/8 selected as reference"]
    REF7_8VDD = 6,
    #[doc = "7: External analog reference selected"]
    AREF = 7,
    #[doc = "8: VDD * 1/16 selected as reference"]
    REF1_16VDD = 8,
    #[doc = "9: VDD * 3/16 selected as reference"]
    REF3_16VDD = 9,
    #[doc = "10: VDD * 5/16 selected as reference"]
    REF5_16VDD = 10,
    #[doc = "11: VDD * 7/16 selected as reference"]
    REF7_16VDD = 11,
    #[doc = "12: VDD * 9/16 selected as reference"]
    REF9_16VDD = 12,
    #[doc = "13: VDD * 11/16 selected as reference"]
    REF11_16VDD = 13,
    #[doc = "14: VDD * 13/16 selected as reference"]
    REF13_16VDD = 14,
    #[doc = "15: VDD * 15/16 selected as reference"]
    REF15_16VDD = 15,
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
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::REF1_8VDD,
            1 => REFSEL_A::REF2_8VDD,
            2 => REFSEL_A::REF3_8VDD,
            3 => REFSEL_A::REF4_8VDD,
            4 => REFSEL_A::REF5_8VDD,
            5 => REFSEL_A::REF6_8VDD,
            6 => REFSEL_A::REF7_8VDD,
            7 => REFSEL_A::AREF,
            8 => REFSEL_A::REF1_16VDD,
            9 => REFSEL_A::REF3_16VDD,
            10 => REFSEL_A::REF5_16VDD,
            11 => REFSEL_A::REF7_16VDD,
            12 => REFSEL_A::REF9_16VDD,
            13 => REFSEL_A::REF11_16VDD,
            14 => REFSEL_A::REF13_16VDD,
            15 => REFSEL_A::REF15_16VDD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REF1_8VDD`"]
    #[inline(always)]
    pub fn is_ref1_8vdd(&self) -> bool {
        *self == REFSEL_A::REF1_8VDD
    }
    #[doc = "Checks if the value of the field is `REF2_8VDD`"]
    #[inline(always)]
    pub fn is_ref2_8vdd(&self) -> bool {
        *self == REFSEL_A::REF2_8VDD
    }
    #[doc = "Checks if the value of the field is `REF3_8VDD`"]
    #[inline(always)]
    pub fn is_ref3_8vdd(&self) -> bool {
        *self == REFSEL_A::REF3_8VDD
    }
    #[doc = "Checks if the value of the field is `REF4_8VDD`"]
    #[inline(always)]
    pub fn is_ref4_8vdd(&self) -> bool {
        *self == REFSEL_A::REF4_8VDD
    }
    #[doc = "Checks if the value of the field is `REF5_8VDD`"]
    #[inline(always)]
    pub fn is_ref5_8vdd(&self) -> bool {
        *self == REFSEL_A::REF5_8VDD
    }
    #[doc = "Checks if the value of the field is `REF6_8VDD`"]
    #[inline(always)]
    pub fn is_ref6_8vdd(&self) -> bool {
        *self == REFSEL_A::REF6_8VDD
    }
    #[doc = "Checks if the value of the field is `REF7_8VDD`"]
    #[inline(always)]
    pub fn is_ref7_8vdd(&self) -> bool {
        *self == REFSEL_A::REF7_8VDD
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFSEL_A::AREF
    }
    #[doc = "Checks if the value of the field is `REF1_16VDD`"]
    #[inline(always)]
    pub fn is_ref1_16vdd(&self) -> bool {
        *self == REFSEL_A::REF1_16VDD
    }
    #[doc = "Checks if the value of the field is `REF3_16VDD`"]
    #[inline(always)]
    pub fn is_ref3_16vdd(&self) -> bool {
        *self == REFSEL_A::REF3_16VDD
    }
    #[doc = "Checks if the value of the field is `REF5_16VDD`"]
    #[inline(always)]
    pub fn is_ref5_16vdd(&self) -> bool {
        *self == REFSEL_A::REF5_16VDD
    }
    #[doc = "Checks if the value of the field is `REF7_16VDD`"]
    #[inline(always)]
    pub fn is_ref7_16vdd(&self) -> bool {
        *self == REFSEL_A::REF7_16VDD
    }
    #[doc = "Checks if the value of the field is `REF9_16VDD`"]
    #[inline(always)]
    pub fn is_ref9_16vdd(&self) -> bool {
        *self == REFSEL_A::REF9_16VDD
    }
    #[doc = "Checks if the value of the field is `REF11_16VDD`"]
    #[inline(always)]
    pub fn is_ref11_16vdd(&self) -> bool {
        *self == REFSEL_A::REF11_16VDD
    }
    #[doc = "Checks if the value of the field is `REF13_16VDD`"]
    #[inline(always)]
    pub fn is_ref13_16vdd(&self) -> bool {
        *self == REFSEL_A::REF13_16VDD
    }
    #[doc = "Checks if the value of the field is `REF15_16VDD`"]
    #[inline(always)]
    pub fn is_ref15_16vdd(&self) -> bool {
        *self == REFSEL_A::REF15_16VDD
    }
}
#[doc = "Field `REFSEL` writer - Reference select"]
pub type REFSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, REFSEL_SPEC, u8, REFSEL_A, 4, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "VDD * 1/8 selected as reference"]
    #[inline(always)]
    pub fn ref1_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF1_8VDD)
    }
    #[doc = "VDD * 2/8 selected as reference"]
    #[inline(always)]
    pub fn ref2_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF2_8VDD)
    }
    #[doc = "VDD * 3/8 selected as reference"]
    #[inline(always)]
    pub fn ref3_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF3_8VDD)
    }
    #[doc = "VDD * 4/8 selected as reference"]
    #[inline(always)]
    pub fn ref4_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF4_8VDD)
    }
    #[doc = "VDD * 5/8 selected as reference"]
    #[inline(always)]
    pub fn ref5_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF5_8VDD)
    }
    #[doc = "VDD * 6/8 selected as reference"]
    #[inline(always)]
    pub fn ref6_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF6_8VDD)
    }
    #[doc = "VDD * 7/8 selected as reference"]
    #[inline(always)]
    pub fn ref7_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF7_8VDD)
    }
    #[doc = "External analog reference selected"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSEL_A::AREF)
    }
    #[doc = "VDD * 1/16 selected as reference"]
    #[inline(always)]
    pub fn ref1_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF1_16VDD)
    }
    #[doc = "VDD * 3/16 selected as reference"]
    #[inline(always)]
    pub fn ref3_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF3_16VDD)
    }
    #[doc = "VDD * 5/16 selected as reference"]
    #[inline(always)]
    pub fn ref5_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF5_16VDD)
    }
    #[doc = "VDD * 7/16 selected as reference"]
    #[inline(always)]
    pub fn ref7_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF7_16VDD)
    }
    #[doc = "VDD * 9/16 selected as reference"]
    #[inline(always)]
    pub fn ref9_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF9_16VDD)
    }
    #[doc = "VDD * 11/16 selected as reference"]
    #[inline(always)]
    pub fn ref11_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF11_16VDD)
    }
    #[doc = "VDD * 13/16 selected as reference"]
    #[inline(always)]
    pub fn ref13_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF13_16VDD)
    }
    #[doc = "VDD * 15/16 selected as reference"]
    #[inline(always)]
    pub fn ref15_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF15_16VDD)
    }
}
impl R {
    #[doc = "Bits 0:3 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference select"]
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
#[doc = "Reference select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refsel](index.html) module"]
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
