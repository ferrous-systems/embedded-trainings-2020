#[doc = "Register `SENSRES` reader"]
pub struct R(crate::R<SENSRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SENSRES` writer"]
pub struct W(crate::W<SENSRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SENSRES_SPEC>;
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
impl From<crate::W<SENSRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SENSRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITFRAMESDD` reader - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type BITFRAMESDD_R = crate::FieldReader<u8, BITFRAMESDD_A>;
#[doc = "Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BITFRAMESDD_A {
    #[doc = "0: SDD pattern 00000"]
    SDD00000 = 0,
    #[doc = "1: SDD pattern 00001"]
    SDD00001 = 1,
    #[doc = "2: SDD pattern 00010"]
    SDD00010 = 2,
    #[doc = "4: SDD pattern 00100"]
    SDD00100 = 4,
    #[doc = "8: SDD pattern 01000"]
    SDD01000 = 8,
    #[doc = "16: SDD pattern 10000"]
    SDD10000 = 16,
}
impl From<BITFRAMESDD_A> for u8 {
    #[inline(always)]
    fn from(variant: BITFRAMESDD_A) -> Self {
        variant as _
    }
}
impl BITFRAMESDD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BITFRAMESDD_A> {
        match self.bits {
            0 => Some(BITFRAMESDD_A::SDD00000),
            1 => Some(BITFRAMESDD_A::SDD00001),
            2 => Some(BITFRAMESDD_A::SDD00010),
            4 => Some(BITFRAMESDD_A::SDD00100),
            8 => Some(BITFRAMESDD_A::SDD01000),
            16 => Some(BITFRAMESDD_A::SDD10000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDD00000`"]
    #[inline(always)]
    pub fn is_sdd00000(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00000
    }
    #[doc = "Checks if the value of the field is `SDD00001`"]
    #[inline(always)]
    pub fn is_sdd00001(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00001
    }
    #[doc = "Checks if the value of the field is `SDD00010`"]
    #[inline(always)]
    pub fn is_sdd00010(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00010
    }
    #[doc = "Checks if the value of the field is `SDD00100`"]
    #[inline(always)]
    pub fn is_sdd00100(&self) -> bool {
        *self == BITFRAMESDD_A::SDD00100
    }
    #[doc = "Checks if the value of the field is `SDD01000`"]
    #[inline(always)]
    pub fn is_sdd01000(&self) -> bool {
        *self == BITFRAMESDD_A::SDD01000
    }
    #[doc = "Checks if the value of the field is `SDD10000`"]
    #[inline(always)]
    pub fn is_sdd10000(&self) -> bool {
        *self == BITFRAMESDD_A::SDD10000
    }
}
#[doc = "Field `BITFRAMESDD` writer - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type BITFRAMESDD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSRES_SPEC, u8, BITFRAMESDD_A, 5, O>;
impl<'a, const O: u8> BITFRAMESDD_W<'a, O> {
    #[doc = "SDD pattern 00000"]
    #[inline(always)]
    pub fn sdd00000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00000)
    }
    #[doc = "SDD pattern 00001"]
    #[inline(always)]
    pub fn sdd00001(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00001)
    }
    #[doc = "SDD pattern 00010"]
    #[inline(always)]
    pub fn sdd00010(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00010)
    }
    #[doc = "SDD pattern 00100"]
    #[inline(always)]
    pub fn sdd00100(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD00100)
    }
    #[doc = "SDD pattern 01000"]
    #[inline(always)]
    pub fn sdd01000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD01000)
    }
    #[doc = "SDD pattern 10000"]
    #[inline(always)]
    pub fn sdd10000(self) -> &'a mut W {
        self.variant(BITFRAMESDD_A::SDD10000)
    }
}
#[doc = "Field `RFU5` reader - Reserved for future use. Shall be 0."]
pub type RFU5_R = crate::BitReader<bool>;
#[doc = "Field `RFU5` writer - Reserved for future use. Shall be 0."]
pub type RFU5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SENSRES_SPEC, bool, O>;
#[doc = "Field `NFCIDSIZE` reader - NFCID1 size. This value is used by the Auto collision resolution engine."]
pub type NFCIDSIZE_R = crate::FieldReader<u8, NFCIDSIZE_A>;
#[doc = "NFCID1 size. This value is used by the Auto collision resolution engine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NFCIDSIZE_A {
    #[doc = "0: NFCID1 size: single (4 bytes)"]
    NFCID1SINGLE = 0,
    #[doc = "1: NFCID1 size: double (7 bytes)"]
    NFCID1DOUBLE = 1,
    #[doc = "2: NFCID1 size: triple (10 bytes)"]
    NFCID1TRIPLE = 2,
}
impl From<NFCIDSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: NFCIDSIZE_A) -> Self {
        variant as _
    }
}
impl NFCIDSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NFCIDSIZE_A> {
        match self.bits {
            0 => Some(NFCIDSIZE_A::NFCID1SINGLE),
            1 => Some(NFCIDSIZE_A::NFCID1DOUBLE),
            2 => Some(NFCIDSIZE_A::NFCID1TRIPLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NFCID1SINGLE`"]
    #[inline(always)]
    pub fn is_nfcid1single(&self) -> bool {
        *self == NFCIDSIZE_A::NFCID1SINGLE
    }
    #[doc = "Checks if the value of the field is `NFCID1DOUBLE`"]
    #[inline(always)]
    pub fn is_nfcid1double(&self) -> bool {
        *self == NFCIDSIZE_A::NFCID1DOUBLE
    }
    #[doc = "Checks if the value of the field is `NFCID1TRIPLE`"]
    #[inline(always)]
    pub fn is_nfcid1triple(&self) -> bool {
        *self == NFCIDSIZE_A::NFCID1TRIPLE
    }
}
#[doc = "Field `NFCIDSIZE` writer - NFCID1 size. This value is used by the Auto collision resolution engine."]
pub type NFCIDSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SENSRES_SPEC, u8, NFCIDSIZE_A, 2, O>;
impl<'a, const O: u8> NFCIDSIZE_W<'a, O> {
    #[doc = "NFCID1 size: single (4 bytes)"]
    #[inline(always)]
    pub fn nfcid1single(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1SINGLE)
    }
    #[doc = "NFCID1 size: double (7 bytes)"]
    #[inline(always)]
    pub fn nfcid1double(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1DOUBLE)
    }
    #[doc = "NFCID1 size: triple (10 bytes)"]
    #[inline(always)]
    pub fn nfcid1triple(self) -> &'a mut W {
        self.variant(NFCIDSIZE_A::NFCID1TRIPLE)
    }
}
#[doc = "Field `PLATFCONFIG` reader - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type PLATFCONFIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLATFCONFIG` writer - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type PLATFCONFIG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SENSRES_SPEC, u8, u8, 4, O>;
#[doc = "Field `RFU74` reader - Reserved for future use. Shall be 0."]
pub type RFU74_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFU74` writer - Reserved for future use. Shall be 0."]
pub type RFU74_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SENSRES_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn bitframesdd(&self) -> BITFRAMESDD_R {
        BITFRAMESDD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu5(&self) -> RFU5_R {
        RFU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the Auto collision resolution engine."]
    #[inline(always)]
    pub fn nfcidsize(&self) -> NFCIDSIZE_R {
        NFCIDSIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn platfconfig(&self) -> PLATFCONFIG_R {
        PLATFCONFIG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu74(&self) -> RFU74_R {
        RFU74_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Bit frame SDD as defined by the b5:b1 of byte 1 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    #[must_use]
    pub fn bitframesdd(&mut self) -> BITFRAMESDD_W<0> {
        BITFRAMESDD_W::new(self)
    }
    #[doc = "Bit 5 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    #[must_use]
    pub fn rfu5(&mut self) -> RFU5_W<5> {
        RFU5_W::new(self)
    }
    #[doc = "Bits 6:7 - NFCID1 size. This value is used by the Auto collision resolution engine."]
    #[inline(always)]
    #[must_use]
    pub fn nfcidsize(&mut self) -> NFCIDSIZE_W<6> {
        NFCIDSIZE_W::new(self)
    }
    #[doc = "Bits 8:11 - Tag platform configuration as defined by the b4:b1 of byte 2 in SENS_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    #[must_use]
    pub fn platfconfig(&mut self) -> PLATFCONFIG_W<8> {
        PLATFCONFIG_W::new(self)
    }
    #[doc = "Bits 12:15 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    #[must_use]
    pub fn rfu74(&mut self) -> RFU74_W<12> {
        RFU74_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NFC-A SENS_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensres](index.html) module"]
pub struct SENSRES_SPEC;
impl crate::RegisterSpec for SENSRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensres::R](R) reader structure"]
impl crate::Readable for SENSRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sensres::W](W) writer structure"]
impl crate::Writable for SENSRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SENSRES to value 0x01"]
impl crate::Resettable for SENSRES_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
