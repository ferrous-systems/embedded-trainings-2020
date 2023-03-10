#[doc = "Register `DAB[%s]` reader"]
pub struct R(crate::R<DAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAB[%s]` writer"]
pub struct W(crate::W<DAB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAB_SPEC>;
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
impl From<crate::W<DAB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAB` reader - Device address base segment n"]
pub type DAB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DAB` writer - Device address base segment n"]
pub type DAB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Device address base segment n"]
    #[inline(always)]
    pub fn dab(&self) -> DAB_R {
        DAB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device address base segment n"]
    #[inline(always)]
    #[must_use]
    pub fn dab(&mut self) -> DAB_W<0> {
        DAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Device address base segment n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dab](index.html) module"]
pub struct DAB_SPEC;
impl crate::RegisterSpec for DAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dab::R](R) reader structure"]
impl crate::Readable for DAB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dab::W](W) writer structure"]
impl crate::Writable for DAB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAB[%s]
to value 0"]
impl crate::Resettable for DAB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
