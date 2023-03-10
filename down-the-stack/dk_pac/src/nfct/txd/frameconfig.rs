#[doc = "Register `FRAMECONFIG` reader"]
pub struct R(crate::R<FRAMECONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMECONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMECONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMECONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMECONFIG` writer"]
pub struct W(crate::W<FRAMECONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMECONFIG_SPEC>;
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
impl From<crate::W<FRAMECONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMECONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARITY` reader - Adding parity or not in the frame"]
pub type PARITY_R = crate::BitReader<PARITY_A>;
#[doc = "Adding parity or not in the frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_A {
    #[doc = "0: Parity is not added in TX frames"]
    NO_PARITY = 0,
    #[doc = "1: Parity is added TX frames"]
    PARITY = 1,
}
impl From<PARITY_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_A {
        match self.bits {
            false => PARITY_A::NO_PARITY,
            true => PARITY_A::PARITY,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY`"]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        *self == PARITY_A::NO_PARITY
    }
    #[doc = "Checks if the value of the field is `PARITY`"]
    #[inline(always)]
    pub fn is_parity(&self) -> bool {
        *self == PARITY_A::PARITY
    }
}
#[doc = "Field `PARITY` writer - Adding parity or not in the frame"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMECONFIG_SPEC, PARITY_A, O>;
impl<'a, const O: u8> PARITY_W<'a, O> {
    #[doc = "Parity is not added in TX frames"]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITY_A::NO_PARITY)
    }
    #[doc = "Parity is added TX frames"]
    #[inline(always)]
    pub fn parity(self) -> &'a mut W {
        self.variant(PARITY_A::PARITY)
    }
}
#[doc = "Field `DISCARDMODE` reader - Discarding unused bits in start or at end of a Frame"]
pub type DISCARDMODE_R = crate::BitReader<DISCARDMODE_A>;
#[doc = "Discarding unused bits in start or at end of a Frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCARDMODE_A {
    #[doc = "0: Unused bits is discarded at end of frame"]
    DISCARD_END = 0,
    #[doc = "1: Unused bits is discarded at start of frame"]
    DISCARD_START = 1,
}
impl From<DISCARDMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DISCARDMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCARDMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCARDMODE_A {
        match self.bits {
            false => DISCARDMODE_A::DISCARD_END,
            true => DISCARDMODE_A::DISCARD_START,
        }
    }
    #[doc = "Checks if the value of the field is `DISCARD_END`"]
    #[inline(always)]
    pub fn is_discard_end(&self) -> bool {
        *self == DISCARDMODE_A::DISCARD_END
    }
    #[doc = "Checks if the value of the field is `DISCARD_START`"]
    #[inline(always)]
    pub fn is_discard_start(&self) -> bool {
        *self == DISCARDMODE_A::DISCARD_START
    }
}
#[doc = "Field `DISCARDMODE` writer - Discarding unused bits in start or at end of a Frame"]
pub type DISCARDMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FRAMECONFIG_SPEC, DISCARDMODE_A, O>;
impl<'a, const O: u8> DISCARDMODE_W<'a, O> {
    #[doc = "Unused bits is discarded at end of frame"]
    #[inline(always)]
    pub fn discard_end(self) -> &'a mut W {
        self.variant(DISCARDMODE_A::DISCARD_END)
    }
    #[doc = "Unused bits is discarded at start of frame"]
    #[inline(always)]
    pub fn discard_start(self) -> &'a mut W {
        self.variant(DISCARDMODE_A::DISCARD_START)
    }
}
#[doc = "Field `SOF` reader - Adding SoF or not in TX frames"]
pub type SOF_R = crate::BitReader<SOF_A>;
#[doc = "Adding SoF or not in TX frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF_A {
    #[doc = "0: Start of Frame symbol not added"]
    NO_SO_F = 0,
    #[doc = "1: Start of Frame symbol added"]
    SO_F = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
impl SOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::NO_SO_F,
            true => SOF_A::SO_F,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SO_F`"]
    #[inline(always)]
    pub fn is_no_so_f(&self) -> bool {
        *self == SOF_A::NO_SO_F
    }
    #[doc = "Checks if the value of the field is `SO_F`"]
    #[inline(always)]
    pub fn is_so_f(&self) -> bool {
        *self == SOF_A::SO_F
    }
}
#[doc = "Field `SOF` writer - Adding SoF or not in TX frames"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMECONFIG_SPEC, SOF_A, O>;
impl<'a, const O: u8> SOF_W<'a, O> {
    #[doc = "Start of Frame symbol not added"]
    #[inline(always)]
    pub fn no_so_f(self) -> &'a mut W {
        self.variant(SOF_A::NO_SO_F)
    }
    #[doc = "Start of Frame symbol added"]
    #[inline(always)]
    pub fn so_f(self) -> &'a mut W {
        self.variant(SOF_A::SO_F)
    }
}
#[doc = "Field `CRCMODETX` reader - CRC mode for outgoing frames"]
pub type CRCMODETX_R = crate::BitReader<CRCMODETX_A>;
#[doc = "CRC mode for outgoing frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCMODETX_A {
    #[doc = "0: CRC is not added to the frame"]
    NO_CRCTX = 0,
    #[doc = "1: 16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    CRC16TX = 1,
}
impl From<CRCMODETX_A> for bool {
    #[inline(always)]
    fn from(variant: CRCMODETX_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCMODETX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCMODETX_A {
        match self.bits {
            false => CRCMODETX_A::NO_CRCTX,
            true => CRCMODETX_A::CRC16TX,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CRCTX`"]
    #[inline(always)]
    pub fn is_no_crctx(&self) -> bool {
        *self == CRCMODETX_A::NO_CRCTX
    }
    #[doc = "Checks if the value of the field is `CRC16TX`"]
    #[inline(always)]
    pub fn is_crc16tx(&self) -> bool {
        *self == CRCMODETX_A::CRC16TX
    }
}
#[doc = "Field `CRCMODETX` writer - CRC mode for outgoing frames"]
pub type CRCMODETX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMECONFIG_SPEC, CRCMODETX_A, O>;
impl<'a, const O: u8> CRCMODETX_W<'a, O> {
    #[doc = "CRC is not added to the frame"]
    #[inline(always)]
    pub fn no_crctx(self) -> &'a mut W {
        self.variant(CRCMODETX_A::NO_CRCTX)
    }
    #[doc = "16 bit CRC added to the frame based on all the data read from RAM that is used in the frame"]
    #[inline(always)]
    pub fn crc16tx(self) -> &'a mut W {
        self.variant(CRCMODETX_A::CRC16TX)
    }
}
impl R {
    #[doc = "Bit 0 - Adding parity or not in the frame"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Discarding unused bits in start or at end of a Frame"]
    #[inline(always)]
    pub fn discardmode(&self) -> DISCARDMODE_R {
        DISCARDMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline(always)]
    pub fn crcmodetx(&self) -> CRCMODETX_R {
        CRCMODETX_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Adding parity or not in the frame"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<0> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 1 - Discarding unused bits in start or at end of a Frame"]
    #[inline(always)]
    #[must_use]
    pub fn discardmode(&mut self) -> DISCARDMODE_W<1> {
        DISCARDMODE_W::new(self)
    }
    #[doc = "Bit 2 - Adding SoF or not in TX frames"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<2> {
        SOF_W::new(self)
    }
    #[doc = "Bit 4 - CRC mode for outgoing frames"]
    #[inline(always)]
    #[must_use]
    pub fn crcmodetx(&mut self) -> CRCMODETX_W<4> {
        CRCMODETX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of outgoing frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameconfig](index.html) module"]
pub struct FRAMECONFIG_SPEC;
impl crate::RegisterSpec for FRAMECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frameconfig::R](R) reader structure"]
impl crate::Readable for FRAMECONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frameconfig::W](W) writer structure"]
impl crate::Writable for FRAMECONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMECONFIG to value 0x17"]
impl crate::Resettable for FRAMECONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x17;
}
