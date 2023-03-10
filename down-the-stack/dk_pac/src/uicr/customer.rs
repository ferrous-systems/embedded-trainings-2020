#[doc = "Register `CUSTOMER[%s]` reader"]
pub struct R(crate::R<CUSTOMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUSTOMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CUSTOMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CUSTOMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CUSTOMER[%s]` writer"]
pub struct W(crate::W<CUSTOMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUSTOMER_SPEC>;
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
impl From<crate::W<CUSTOMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUSTOMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CUSTOMER` reader - Reserved for customer"]
pub type CUSTOMER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CUSTOMER` writer - Reserved for customer"]
pub type CUSTOMER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CUSTOMER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Reserved for customer"]
    #[inline(always)]
    pub fn customer(&self) -> CUSTOMER_R {
        CUSTOMER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for customer"]
    #[inline(always)]
    #[must_use]
    pub fn customer(&mut self) -> CUSTOMER_W<0> {
        CUSTOMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Reserved for customer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [customer](index.html) module"]
pub struct CUSTOMER_SPEC;
impl crate::RegisterSpec for CUSTOMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [customer::R](R) reader structure"]
impl crate::Readable for CUSTOMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [customer::W](W) writer structure"]
impl crate::Writable for CUSTOMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CUSTOMER[%s]
to value 0xffff_ffff"]
impl crate::Resettable for CUSTOMER_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
