#[doc = "Register `PCNF1` reader"]
pub struct R(crate::R<PCNF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNF1` writer"]
pub struct W(crate::W<PCNF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNF1_SPEC>;
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
impl From<crate::W<PCNF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXLEN` reader - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
pub type MAXLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXLEN` writer - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
pub type MAXLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNF1_SPEC, u8, u8, 8, O>;
#[doc = "Field `STATLEN` reader - Static length in number of bytes"]
pub type STATLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATLEN` writer - Static length in number of bytes"]
pub type STATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNF1_SPEC, u8, u8, 8, O>;
#[doc = "Field `BALEN` reader - Base address length in number of bytes"]
pub type BALEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BALEN` writer - Base address length in number of bytes"]
pub type BALEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNF1_SPEC, u8, u8, 3, O>;
#[doc = "Field `ENDIAN` reader - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
pub type ENDIAN_R = crate::BitReader<ENDIAN_A>;
#[doc = "On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIAN_A {
    #[doc = "0: Least Significant bit on air first"]
    LITTLE = 0,
    #[doc = "1: Most significant bit on air first"]
    BIG = 1,
}
impl From<ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::LITTLE,
            true => ENDIAN_A::BIG,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIAN_A::LITTLE
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIAN_A::BIG
    }
}
#[doc = "Field `ENDIAN` writer - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
pub type ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNF1_SPEC, ENDIAN_A, O>;
impl<'a, const O: u8> ENDIAN_W<'a, O> {
    #[doc = "Least Significant bit on air first"]
    #[inline(always)]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIAN_A::LITTLE)
    }
    #[doc = "Most significant bit on air first"]
    #[inline(always)]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIAN_A::BIG)
    }
}
#[doc = "Field `WHITEEN` reader - Enable or disable packet whitening"]
pub type WHITEEN_R = crate::BitReader<WHITEEN_A>;
#[doc = "Enable or disable packet whitening\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WHITEEN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<WHITEEN_A> for bool {
    #[inline(always)]
    fn from(variant: WHITEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WHITEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WHITEEN_A {
        match self.bits {
            false => WHITEEN_A::DISABLED,
            true => WHITEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WHITEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WHITEEN_A::ENABLED
    }
}
#[doc = "Field `WHITEEN` writer - Enable or disable packet whitening"]
pub type WHITEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNF1_SPEC, WHITEEN_A, O>;
impl<'a, const O: u8> WHITEEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WHITEEN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WHITEEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn maxlen(&self) -> MAXLEN_R {
        MAXLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    pub fn statlen(&self) -> STATLEN_R {
        STATLEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    pub fn balen(&self) -> BALEN_R {
        BALEN_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    pub fn whiteen(&self) -> WHITEEN_R {
        WHITEEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    #[must_use]
    pub fn maxlen(&mut self) -> MAXLEN_W<0> {
        MAXLEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    #[must_use]
    pub fn statlen(&mut self) -> STATLEN_W<8> {
        STATLEN_W::new(self)
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    #[must_use]
    pub fn balen(&mut self) -> BALEN_W<16> {
        BALEN_W::new(self)
    }
    #[doc = "Bit 24 - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> ENDIAN_W<24> {
        ENDIAN_W::new(self)
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    #[must_use]
    pub fn whiteen(&mut self) -> WHITEEN_W<25> {
        WHITEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf1](index.html) module"]
pub struct PCNF1_SPEC;
impl crate::RegisterSpec for PCNF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnf1::R](R) reader structure"]
impl crate::Readable for PCNF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnf1::W](W) writer structure"]
impl crate::Writable for PCNF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNF1 to value 0"]
impl crate::Resettable for PCNF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
