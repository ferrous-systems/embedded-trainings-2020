#[doc = "Register `CRCPOLY` reader"]
pub struct R(crate::R<CRCPOLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCPOLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCPOLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCPOLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCPOLY` writer"]
pub struct W(crate::W<CRCPOLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCPOLY_SPEC>;
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
impl From<crate::W<CRCPOLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCPOLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCPOLY` reader - CRC polynomial"]
pub type CRCPOLY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CRCPOLY` writer - CRC polynomial"]
pub type CRCPOLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCPOLY_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - CRC polynomial"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CRC polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<0> {
        CRCPOLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC polynomial\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcpoly](index.html) module"]
pub struct CRCPOLY_SPEC;
impl crate::RegisterSpec for CRCPOLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcpoly::R](R) reader structure"]
impl crate::Readable for CRCPOLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcpoly::W](W) writer structure"]
impl crate::Writable for CRCPOLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRCPOLY to value 0"]
impl crate::Resettable for CRCPOLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
