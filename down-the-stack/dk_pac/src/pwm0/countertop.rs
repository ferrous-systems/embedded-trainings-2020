#[doc = "Register `COUNTERTOP` reader"]
pub struct R(crate::R<COUNTERTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTERTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNTERTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNTERTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTERTOP` writer"]
pub struct W(crate::W<COUNTERTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTERTOP_SPEC>;
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
impl From<crate::W<COUNTERTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNTERTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTERTOP` reader - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
pub type COUNTERTOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNTERTOP` writer - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
pub type COUNTERTOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COUNTERTOP_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
    #[inline(always)]
    pub fn countertop(&self) -> COUNTERTOP_R {
        COUNTERTOP_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
    #[inline(always)]
    #[must_use]
    pub fn countertop(&mut self) -> COUNTERTOP_W<0> {
        COUNTERTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value up to which the pulse generator counter counts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [countertop](index.html) module"]
pub struct COUNTERTOP_SPEC;
impl crate::RegisterSpec for COUNTERTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [countertop::R](R) reader structure"]
impl crate::Readable for COUNTERTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [countertop::W](W) writer structure"]
impl crate::Writable for COUNTERTOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COUNTERTOP to value 0x03ff"]
impl crate::Resettable for COUNTERTOP_SPEC {
    const RESET_VALUE: Self::Ux = 0x03ff;
}
