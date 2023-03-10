#[doc = "Register `BCC` reader"]
pub struct R(crate::R<BCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCC` writer"]
pub struct W(crate::W<BCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCC_SPEC>;
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
impl From<crate::W<BCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCC` reader - Bit counter compare"]
pub type BCC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BCC` writer - Bit counter compare"]
pub type BCC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BCC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Bit counter compare"]
    #[inline(always)]
    pub fn bcc(&self) -> BCC_R {
        BCC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bit counter compare"]
    #[inline(always)]
    #[must_use]
    pub fn bcc(&mut self) -> BCC_W<0> {
        BCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit counter compare\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcc](index.html) module"]
pub struct BCC_SPEC;
impl crate::RegisterSpec for BCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcc::R](R) reader structure"]
impl crate::Readable for BCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcc::W](W) writer structure"]
impl crate::Writable for BCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BCC to value 0"]
impl crate::Resettable for BCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
