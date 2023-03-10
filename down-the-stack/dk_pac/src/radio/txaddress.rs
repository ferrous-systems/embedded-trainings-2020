#[doc = "Register `TXADDRESS` reader"]
pub struct R(crate::R<TXADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXADDRESS` writer"]
pub struct W(crate::W<TXADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXADDRESS_SPEC>;
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
impl From<crate::W<TXADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXADDRESS` reader - Transmit address select"]
pub type TXADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXADDRESS` writer - Transmit address select"]
pub type TXADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXADDRESS_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Transmit address select"]
    #[inline(always)]
    pub fn txaddress(&self) -> TXADDRESS_R {
        TXADDRESS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit address select"]
    #[inline(always)]
    #[must_use]
    pub fn txaddress(&mut self) -> TXADDRESS_W<0> {
        TXADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit address select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txaddress](index.html) module"]
pub struct TXADDRESS_SPEC;
impl crate::RegisterSpec for TXADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txaddress::R](R) reader structure"]
impl crate::Readable for TXADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txaddress::W](W) writer structure"]
impl crate::Writable for TXADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXADDRESS to value 0"]
impl crate::Resettable for TXADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
