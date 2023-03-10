#[doc = "Register `AMOUNT` reader"]
pub struct R(crate::R<AMOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMOUNT` writer"]
pub struct W(crate::W<AMOUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMOUNT_SPEC>;
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
impl From<crate::W<AMOUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMOUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATABITS` reader - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
pub type TXDATABITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXDATABITS` writer - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
pub type TXDATABITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMOUNT_SPEC, u8, u8, 3, O>;
#[doc = "Field `TXDATABYTES` reader - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
pub type TXDATABYTES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXDATABYTES` writer - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
pub type TXDATABYTES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMOUNT_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn txdatabits(&self) -> TXDATABITS_R {
        TXDATABITS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
    #[inline(always)]
    pub fn txdatabytes(&self) -> TXDATABYTES_R {
        TXDATABYTES_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    #[must_use]
    pub fn txdatabits(&mut self) -> TXDATABITS_W<0> {
        TXDATABITS_W::new(self)
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
    #[inline(always)]
    #[must_use]
    pub fn txdatabytes(&mut self) -> TXDATABYTES_W<3> {
        TXDATABYTES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Size of outgoing frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amount](index.html) module"]
pub struct AMOUNT_SPEC;
impl crate::RegisterSpec for AMOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amount::R](R) reader structure"]
impl crate::Readable for AMOUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amount::W](W) writer structure"]
impl crate::Writable for AMOUNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMOUNT to value 0"]
impl crate::Resettable for AMOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
