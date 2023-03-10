#[doc = "Register `CRCINIT` reader"]
pub struct R(crate::R<CRCINIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCINIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCINIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCINIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCINIT` writer"]
pub struct W(crate::W<CRCINIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCINIT_SPEC>;
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
impl From<crate::W<CRCINIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCINIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCINIT` reader - CRC initial value"]
pub type CRCINIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRCINIT` writer - CRC initial value"]
pub type CRCINIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCINIT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - CRC initial value"]
    #[inline(always)]
    pub fn crcinit(&self) -> CRCINIT_R {
        CRCINIT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CRC initial value"]
    #[inline(always)]
    #[must_use]
    pub fn crcinit(&mut self) -> CRCINIT_W<0> {
        CRCINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC initial value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcinit](index.html) module"]
pub struct CRCINIT_SPEC;
impl crate::RegisterSpec for CRCINIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcinit::R](R) reader structure"]
impl crate::Readable for CRCINIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcinit::W](W) writer structure"]
impl crate::Writable for CRCINIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCINIT to value 0"]
impl crate::Resettable for CRCINIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
