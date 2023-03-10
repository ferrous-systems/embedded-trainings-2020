#[doc = "Register `PTR` reader"]
pub struct R(crate::R<PTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTR` writer"]
pub struct W(crate::W<PTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTR_SPEC>;
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
impl From<crate::W<PTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - Transmit buffer Data RAM start address. When transmitting, words containing samples will be fetched from this address. This address is a word aligned Data RAM address."]
pub type PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PTR` writer - Transmit buffer Data RAM start address. When transmitting, words containing samples will be fetched from this address. This address is a word aligned Data RAM address."]
pub type PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit buffer Data RAM start address. When transmitting, words containing samples will be fetched from this address. This address is a word aligned Data RAM address."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit buffer Data RAM start address. When transmitting, words containing samples will be fetched from this address. This address is a word aligned Data RAM address."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PTR_W<0> {
        PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit buffer RAM start address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](index.html) module"]
pub struct PTR_SPEC;
impl crate::RegisterSpec for PTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptr::R](R) reader structure"]
impl crate::Readable for PTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptr::W](W) writer structure"]
impl crate::Writable for PTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTR to value 0"]
impl crate::Resettable for PTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
