#[doc = "Register `FRAMEDELAYMAX` reader"]
pub struct R(crate::R<FRAMEDELAYMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEDELAYMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEDELAYMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEDELAYMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEDELAYMAX` writer"]
pub struct W(crate::W<FRAMEDELAYMAX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEDELAYMAX_SPEC>;
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
impl From<crate::W<FRAMEDELAYMAX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEDELAYMAX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDELAYMAX` reader - Maximum frame delay in number of 13.56 MHz clocks"]
pub type FRAMEDELAYMAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEDELAYMAX` writer - Maximum frame delay in number of 13.56 MHz clocks"]
pub type FRAMEDELAYMAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMEDELAYMAX_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Maximum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymax(&self) -> FRAMEDELAYMAX_R {
        FRAMEDELAYMAX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Maximum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    #[must_use]
    pub fn framedelaymax(&mut self) -> FRAMEDELAYMAX_W<0> {
        FRAMEDELAYMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymax](index.html) module"]
pub struct FRAMEDELAYMAX_SPEC;
impl crate::RegisterSpec for FRAMEDELAYMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framedelaymax::R](R) reader structure"]
impl crate::Readable for FRAMEDELAYMAX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framedelaymax::W](W) writer structure"]
impl crate::Writable for FRAMEDELAYMAX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMEDELAYMAX to value 0x1000"]
impl crate::Resettable for FRAMEDELAYMAX_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
