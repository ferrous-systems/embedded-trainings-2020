#[doc = "Register `BASE1` reader"]
pub struct R(crate::R<BASE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASE1` writer"]
pub struct W(crate::W<BASE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASE1_SPEC>;
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
impl From<crate::W<BASE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE1` reader - Base address 1"]
pub type BASE1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASE1` writer - Base address 1"]
pub type BASE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BASE1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base address 1"]
    #[inline(always)]
    pub fn base1(&self) -> BASE1_R {
        BASE1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address 1"]
    #[inline(always)]
    #[must_use]
    pub fn base1(&mut self) -> BASE1_W<0> {
        BASE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [base1](index.html) module"]
pub struct BASE1_SPEC;
impl crate::RegisterSpec for BASE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [base1::R](R) reader structure"]
impl crate::Readable for BASE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [base1::W](W) writer structure"]
impl crate::Writable for BASE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BASE1 to value 0"]
impl crate::Resettable for BASE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
