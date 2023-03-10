#[doc = "Register `ADDRESS` reader"]
pub struct R(crate::R<ADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRESS` writer"]
pub struct W(crate::W<ADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRESS_SPEC>;
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
impl From<crate::W<ADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS` reader - Address used in the TWI transfer"]
pub type ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRESS` writer - Address used in the TWI transfer"]
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDRESS_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Address used in the TWI transfer"]
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Address used in the TWI transfer"]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Address used in the TWI transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address](index.html) module"]
pub struct ADDRESS_SPEC;
impl crate::RegisterSpec for ADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [address::R](R) reader structure"]
impl crate::Readable for ADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [address::W](W) writer structure"]
impl crate::Writable for ADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDRESS to value 0"]
impl crate::Resettable for ADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
