#[doc = "Register `TXPOWER` reader"]
pub struct R(crate::R<TXPOWER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXPOWER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXPOWER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXPOWER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPOWER` writer"]
pub struct W(crate::W<TXPOWER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXPOWER_SPEC>;
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
impl From<crate::W<TXPOWER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXPOWER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPOWER` reader - RADIO output power."]
pub type TXPOWER_R = crate::FieldReader<u8, TXPOWER_A>;
#[doc = "RADIO output power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXPOWER_A {
    #[doc = "4: +4 dBm"]
    POS4D_BM = 4,
    #[doc = "3: +3 dBm"]
    POS3D_BM = 3,
    #[doc = "0: 0 dBm"]
    _0D_BM = 0,
    #[doc = "252: -4 dBm"]
    NEG4D_BM = 252,
    #[doc = "248: -8 dBm"]
    NEG8D_BM = 248,
    #[doc = "244: -12 dBm"]
    NEG12D_BM = 244,
    #[doc = "240: -16 dBm"]
    NEG16D_BM = 240,
    #[doc = "236: -20 dBm"]
    NEG20D_BM = 236,
    #[doc = "255: Deprecated enumerator - -40 dBm"]
    NEG30D_BM = 255,
    #[doc = "216: -40 dBm"]
    NEG40D_BM = 216,
}
impl From<TXPOWER_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPOWER_A) -> Self {
        variant as _
    }
}
impl TXPOWER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXPOWER_A> {
        match self.bits {
            4 => Some(TXPOWER_A::POS4D_BM),
            3 => Some(TXPOWER_A::POS3D_BM),
            0 => Some(TXPOWER_A::_0D_BM),
            252 => Some(TXPOWER_A::NEG4D_BM),
            248 => Some(TXPOWER_A::NEG8D_BM),
            244 => Some(TXPOWER_A::NEG12D_BM),
            240 => Some(TXPOWER_A::NEG16D_BM),
            236 => Some(TXPOWER_A::NEG20D_BM),
            255 => Some(TXPOWER_A::NEG30D_BM),
            216 => Some(TXPOWER_A::NEG40D_BM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `POS4D_BM`"]
    #[inline(always)]
    pub fn is_pos4d_bm(&self) -> bool {
        *self == TXPOWER_A::POS4D_BM
    }
    #[doc = "Checks if the value of the field is `POS3D_BM`"]
    #[inline(always)]
    pub fn is_pos3d_bm(&self) -> bool {
        *self == TXPOWER_A::POS3D_BM
    }
    #[doc = "Checks if the value of the field is `_0D_BM`"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        *self == TXPOWER_A::_0D_BM
    }
    #[doc = "Checks if the value of the field is `NEG4D_BM`"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG4D_BM
    }
    #[doc = "Checks if the value of the field is `NEG8D_BM`"]
    #[inline(always)]
    pub fn is_neg8d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG8D_BM
    }
    #[doc = "Checks if the value of the field is `NEG12D_BM`"]
    #[inline(always)]
    pub fn is_neg12d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG12D_BM
    }
    #[doc = "Checks if the value of the field is `NEG16D_BM`"]
    #[inline(always)]
    pub fn is_neg16d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG16D_BM
    }
    #[doc = "Checks if the value of the field is `NEG20D_BM`"]
    #[inline(always)]
    pub fn is_neg20d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG20D_BM
    }
    #[doc = "Checks if the value of the field is `NEG30D_BM`"]
    #[inline(always)]
    pub fn is_neg30d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG30D_BM
    }
    #[doc = "Checks if the value of the field is `NEG40D_BM`"]
    #[inline(always)]
    pub fn is_neg40d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG40D_BM
    }
}
#[doc = "Field `TXPOWER` writer - RADIO output power."]
pub type TXPOWER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXPOWER_SPEC, u8, TXPOWER_A, 8, O>;
impl<'a, const O: u8> TXPOWER_W<'a, O> {
    #[doc = "+4 dBm"]
    #[inline(always)]
    pub fn pos4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS4D_BM)
    }
    #[doc = "+3 dBm"]
    #[inline(always)]
    pub fn pos3d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS3D_BM)
    }
    #[doc = "0 dBm"]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::_0D_BM)
    }
    #[doc = "-4 dBm"]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG4D_BM)
    }
    #[doc = "-8 dBm"]
    #[inline(always)]
    pub fn neg8d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG8D_BM)
    }
    #[doc = "-12 dBm"]
    #[inline(always)]
    pub fn neg12d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG12D_BM)
    }
    #[doc = "-16 dBm"]
    #[inline(always)]
    pub fn neg16d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG16D_BM)
    }
    #[doc = "-20 dBm"]
    #[inline(always)]
    pub fn neg20d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG20D_BM)
    }
    #[doc = "Deprecated enumerator - -40 dBm"]
    #[inline(always)]
    pub fn neg30d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG30D_BM)
    }
    #[doc = "-40 dBm"]
    #[inline(always)]
    pub fn neg40d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG40D_BM)
    }
}
impl R {
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline(always)]
    pub fn txpower(&self) -> TXPOWER_R {
        TXPOWER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - RADIO output power."]
    #[inline(always)]
    #[must_use]
    pub fn txpower(&mut self) -> TXPOWER_W<0> {
        TXPOWER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output power\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txpower](index.html) module"]
pub struct TXPOWER_SPEC;
impl crate::RegisterSpec for TXPOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txpower::R](R) reader structure"]
impl crate::Readable for TXPOWER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txpower::W](W) writer structure"]
impl crate::Writable for TXPOWER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXPOWER to value 0"]
impl crate::Resettable for TXPOWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
