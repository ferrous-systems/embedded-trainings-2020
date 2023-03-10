#[doc = "Register `TIFS` reader"]
pub struct R(crate::R<TIFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIFS` writer"]
pub struct W(crate::W<TIFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIFS_SPEC>;
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
impl From<crate::W<TIFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIFS` reader - Inter Frame Spacing in us"]
pub type TIFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIFS` writer - Inter Frame Spacing in us"]
pub type TIFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIFS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Inter Frame Spacing in us"]
    #[inline(always)]
    pub fn tifs(&self) -> TIFS_R {
        TIFS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Inter Frame Spacing in us"]
    #[inline(always)]
    #[must_use]
    pub fn tifs(&mut self) -> TIFS_W<0> {
        TIFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Inter Frame Spacing in us\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tifs](index.html) module"]
pub struct TIFS_SPEC;
impl crate::RegisterSpec for TIFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tifs::R](R) reader structure"]
impl crate::Readable for TIFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tifs::W](W) writer structure"]
impl crate::Writable for TIFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIFS to value 0"]
impl crate::Resettable for TIFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
