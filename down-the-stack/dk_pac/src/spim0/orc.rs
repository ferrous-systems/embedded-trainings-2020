#[doc = "Register `ORC` reader"]
pub struct R(crate::R<ORC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ORC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ORC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ORC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ORC` writer"]
pub struct W(crate::W<ORC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ORC_SPEC>;
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
impl From<crate::W<ORC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ORC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ORC` reader - Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub type ORC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ORC` writer - Over-read character. Character clocked out in case and over-read of the TXD buffer."]
pub type ORC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ORC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Over-read character. Character clocked out in case and over-read of the TXD buffer."]
    #[inline(always)]
    pub fn orc(&self) -> ORC_R {
        ORC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Over-read character. Character clocked out in case and over-read of the TXD buffer."]
    #[inline(always)]
    #[must_use]
    pub fn orc(&mut self) -> ORC_W<0> {
        ORC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Over-read character. Character clocked out in case and over-read of the TXD buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orc](index.html) module"]
pub struct ORC_SPEC;
impl crate::RegisterSpec for ORC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [orc::R](R) reader structure"]
impl crate::Readable for ORC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [orc::W](W) writer structure"]
impl crate::Writable for ORC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ORC to value 0"]
impl crate::Resettable for ORC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
