#[doc = "Register `NIRK` reader"]
pub struct R(crate::R<NIRK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NIRK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NIRK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NIRK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NIRK` writer"]
pub struct W(crate::W<NIRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NIRK_SPEC>;
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
impl From<crate::W<NIRK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NIRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NIRK` reader - Number of Identity root keys available in the IRK data structure"]
pub type NIRK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NIRK` writer - Number of Identity root keys available in the IRK data structure"]
pub type NIRK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NIRK_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of Identity root keys available in the IRK data structure"]
    #[inline(always)]
    pub fn nirk(&self) -> NIRK_R {
        NIRK_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of Identity root keys available in the IRK data structure"]
    #[inline(always)]
    #[must_use]
    pub fn nirk(&mut self) -> NIRK_W<0> {
        NIRK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Number of IRKs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nirk](index.html) module"]
pub struct NIRK_SPEC;
impl crate::RegisterSpec for NIRK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nirk::R](R) reader structure"]
impl crate::Readable for NIRK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nirk::W](W) writer structure"]
impl crate::Writable for NIRK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NIRK to value 0x01"]
impl crate::Resettable for NIRK_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
