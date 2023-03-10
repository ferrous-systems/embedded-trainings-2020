#[doc = "Register `POFCON` reader"]
pub struct R(crate::R<POFCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POFCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POFCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POFCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POFCON` writer"]
pub struct W(crate::W<POFCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POFCON_SPEC>;
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
impl From<crate::W<POFCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POFCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POF` reader - Enable or disable power failure comparator"]
pub type POF_R = crate::BitReader<POF_A>;
#[doc = "Enable or disable power failure comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POF_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<POF_A> for bool {
    #[inline(always)]
    fn from(variant: POF_A) -> Self {
        variant as u8 != 0
    }
}
impl POF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POF_A {
        match self.bits {
            false => POF_A::DISABLED,
            true => POF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POF_A::ENABLED
    }
}
#[doc = "Field `POF` writer - Enable or disable power failure comparator"]
pub type POF_W<'a, const O: u8> = crate::BitWriter<'a, u32, POFCON_SPEC, POF_A, O>;
impl<'a, const O: u8> POF_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POF_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POF_A::ENABLED)
    }
}
#[doc = "Field `THRESHOLD` reader - Power failure comparator threshold setting"]
pub type THRESHOLD_R = crate::FieldReader<u8, THRESHOLD_A>;
#[doc = "Power failure comparator threshold setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum THRESHOLD_A {
    #[doc = "4: Set threshold to 1.7 V"]
    V17 = 4,
    #[doc = "5: Set threshold to 1.8 V"]
    V18 = 5,
    #[doc = "6: Set threshold to 1.9 V"]
    V19 = 6,
    #[doc = "7: Set threshold to 2.0 V"]
    V20 = 7,
    #[doc = "8: Set threshold to 2.1 V"]
    V21 = 8,
    #[doc = "9: Set threshold to 2.2 V"]
    V22 = 9,
    #[doc = "10: Set threshold to 2.3 V"]
    V23 = 10,
    #[doc = "11: Set threshold to 2.4 V"]
    V24 = 11,
    #[doc = "12: Set threshold to 2.5 V"]
    V25 = 12,
    #[doc = "13: Set threshold to 2.6 V"]
    V26 = 13,
    #[doc = "14: Set threshold to 2.7 V"]
    V27 = 14,
    #[doc = "15: Set threshold to 2.8 V"]
    V28 = 15,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        variant as _
    }
}
impl THRESHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<THRESHOLD_A> {
        match self.bits {
            4 => Some(THRESHOLD_A::V17),
            5 => Some(THRESHOLD_A::V18),
            6 => Some(THRESHOLD_A::V19),
            7 => Some(THRESHOLD_A::V20),
            8 => Some(THRESHOLD_A::V21),
            9 => Some(THRESHOLD_A::V22),
            10 => Some(THRESHOLD_A::V23),
            11 => Some(THRESHOLD_A::V24),
            12 => Some(THRESHOLD_A::V25),
            13 => Some(THRESHOLD_A::V26),
            14 => Some(THRESHOLD_A::V27),
            15 => Some(THRESHOLD_A::V28),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `V17`"]
    #[inline(always)]
    pub fn is_v17(&self) -> bool {
        *self == THRESHOLD_A::V17
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == THRESHOLD_A::V18
    }
    #[doc = "Checks if the value of the field is `V19`"]
    #[inline(always)]
    pub fn is_v19(&self) -> bool {
        *self == THRESHOLD_A::V19
    }
    #[doc = "Checks if the value of the field is `V20`"]
    #[inline(always)]
    pub fn is_v20(&self) -> bool {
        *self == THRESHOLD_A::V20
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        *self == THRESHOLD_A::V21
    }
    #[doc = "Checks if the value of the field is `V22`"]
    #[inline(always)]
    pub fn is_v22(&self) -> bool {
        *self == THRESHOLD_A::V22
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        *self == THRESHOLD_A::V23
    }
    #[doc = "Checks if the value of the field is `V24`"]
    #[inline(always)]
    pub fn is_v24(&self) -> bool {
        *self == THRESHOLD_A::V24
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == THRESHOLD_A::V25
    }
    #[doc = "Checks if the value of the field is `V26`"]
    #[inline(always)]
    pub fn is_v26(&self) -> bool {
        *self == THRESHOLD_A::V26
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLD_A::V27
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline(always)]
    pub fn is_v28(&self) -> bool {
        *self == THRESHOLD_A::V28
    }
}
#[doc = "Field `THRESHOLD` writer - Power failure comparator threshold setting"]
pub type THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, POFCON_SPEC, u8, THRESHOLD_A, 4, O>;
impl<'a, const O: u8> THRESHOLD_W<'a, O> {
    #[doc = "Set threshold to 1.7 V"]
    #[inline(always)]
    pub fn v17(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V17)
    }
    #[doc = "Set threshold to 1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V18)
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline(always)]
    pub fn v19(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V19)
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline(always)]
    pub fn v20(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V20)
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline(always)]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V21)
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline(always)]
    pub fn v22(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V22)
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline(always)]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V23)
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline(always)]
    pub fn v24(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V24)
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline(always)]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V25)
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline(always)]
    pub fn v26(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V26)
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn v28(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V28)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable power failure comparator"]
    #[inline(always)]
    pub fn pof(&self) -> POF_R {
        POF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Power failure comparator threshold setting"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable power failure comparator"]
    #[inline(always)]
    #[must_use]
    pub fn pof(&mut self) -> POF_W<0> {
        POF_W::new(self)
    }
    #[doc = "Bits 1:4 - Power failure comparator threshold setting"]
    #[inline(always)]
    #[must_use]
    pub fn threshold(&mut self) -> THRESHOLD_W<1> {
        THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power failure comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pofcon](index.html) module"]
pub struct POFCON_SPEC;
impl crate::RegisterSpec for POFCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pofcon::R](R) reader structure"]
impl crate::Readable for POFCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pofcon::W](W) writer structure"]
impl crate::Writable for POFCON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POFCON to value 0"]
impl crate::Resettable for POFCON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
