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
#[doc = "Field `PARITY` reader - Parity expected or not in RX frame"]
pub type PARITY_R = crate::BitReader<PARITY_A>;
#[doc = "Parity expected or not in RX frame\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_A {
    #[doc = "0: Parity is not expected in RX frames"]
    NO_PARITY = 0,
    #[doc = "1: Parity is expected in RX frames"]
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
#[doc = "Field `PARITY` writer - Parity expected or not in RX frame"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMECONFIG_SPEC, PARITY_A, O>;
impl<'a, const O: u8> PARITY_W<'a, O> {
    #[doc = "Parity is not expected in RX frames"]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITY_A::NO_PARITY)
    }
    #[doc = "Parity is expected in RX frames"]
    #[inline(always)]
    pub fn parity(self) -> &'a mut W {
        self.variant(PARITY_A::PARITY)
    }
}
#[doc = "Field `SOF` reader - SoF expected or not in RX frames"]
pub type SOF_R = crate::BitReader<SOF_A>;
#[doc = "SoF expected or not in RX frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOF_A {
    #[doc = "0: Start of Frame symbol is not expected in RX frames"]
    NO_SO_F = 0,
    #[doc = "1: Start of Frame symbol is expected in RX frames"]
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
#[doc = "Field `SOF` writer - SoF expected or not in RX frames"]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMECONFIG_SPEC, SOF_A, O>;
impl<'a, const O: u8> SOF_W<'a, O> {
    #[doc = "Start of Frame symbol is not expected in RX frames"]
    #[inline(always)]
    pub fn no_so_f(self) -> &'a mut W {
        self.variant(SOF_A::NO_SO_F)
    }
    #[doc = "Start of Frame symbol is expected in RX frames"]
    #[inline(always)]
    pub fn so_f(self) -> &'a mut W {
        self.variant(SOF_A::SO_F)
    }
}
#[doc = "Field `CRCMODERX` reader - CRC mode for incoming frames"]
pub type CRCMODERX_R = crate::BitReader<CRCMODERX_A>;
#[doc = "CRC mode for incoming frames\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCMODERX_A {
    #[doc = "0: CRC is not expected in RX frames"]
    NO_CRCRX = 0,
    #[doc = "1: Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    CRC16RX = 1,
}
impl From<CRCMODERX_A> for bool {
    #[inline(always)]
    fn from(variant: CRCMODERX_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCMODERX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCMODERX_A {
        match self.bits {
            false => CRCMODERX_A::NO_CRCRX,
            true => CRCMODERX_A::CRC16RX,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CRCRX`"]
    #[inline(always)]
    pub fn is_no_crcrx(&self) -> bool {
        *self == CRCMODERX_A::NO_CRCRX
    }
    #[doc = "Checks if the value of the field is `CRC16RX`"]
    #[inline(always)]
    pub fn is_crc16rx(&self) -> bool {
        *self == CRCMODERX_A::CRC16RX
    }
}
#[doc = "Field `CRCMODERX` writer - CRC mode for incoming frames"]
pub type CRCMODERX_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRAMECONFIG_SPEC, CRCMODERX_A, O>;
impl<'a, const O: u8> CRCMODERX_W<'a, O> {
    #[doc = "CRC is not expected in RX frames"]
    #[inline(always)]
    pub fn no_crcrx(self) -> &'a mut W {
        self.variant(CRCMODERX_A::NO_CRCRX)
    }
    #[doc = "Last 16 bits in RX frame is CRC, CRC is checked and CRCSTATUS updated"]
    #[inline(always)]
    pub fn crc16rx(self) -> &'a mut W {
        self.variant(CRCMODERX_A::CRC16RX)
    }
}
impl R {
    #[doc = "Bit 0 - Parity expected or not in RX frame"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline(always)]
    pub fn crcmoderx(&self) -> CRCMODERX_R {
        CRCMODERX_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity expected or not in RX frame"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<0> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 2 - SoF expected or not in RX frames"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<2> {
        SOF_W::new(self)
    }
    #[doc = "Bit 4 - CRC mode for incoming frames"]
    #[inline(always)]
    #[must_use]
    pub fn crcmoderx(&mut self) -> CRCMODERX_W<4> {
        CRCMODERX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of incoming frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frameconfig](index.html) module"]
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
#[doc = "`reset()` method sets FRAMECONFIG to value 0x15"]
impl crate::Resettable for FRAMECONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x15;
}
