#[doc = "Register `FRAMEDELAYMIN` reader"]
pub struct R(crate::R<FRAMEDELAYMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEDELAYMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEDELAYMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEDELAYMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEDELAYMIN` writer"]
pub struct W(crate::W<FRAMEDELAYMIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEDELAYMIN_SPEC>;
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
impl From<crate::W<FRAMEDELAYMIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEDELAYMIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDELAYMIN` reader - Minimum frame delay in number of 13.56 MHz clocks"]
pub type FRAMEDELAYMIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRAMEDELAYMIN` writer - Minimum frame delay in number of 13.56 MHz clocks"]
pub type FRAMEDELAYMIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAMEDELAYMIN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymin(&self) -> FRAMEDELAYMIN_R {
        FRAMEDELAYMIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    #[must_use]
    pub fn framedelaymin(&mut self) -> FRAMEDELAYMIN_W<0> {
        FRAMEDELAYMIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Minimum frame delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymin](index.html) module"]
pub struct FRAMEDELAYMIN_SPEC;
impl crate::RegisterSpec for FRAMEDELAYMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framedelaymin::R](R) reader structure"]
impl crate::Readable for FRAMEDELAYMIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framedelaymin::W](W) writer structure"]
impl crate::Writable for FRAMEDELAYMIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMEDELAYMIN to value 0x0480"]
impl crate::Resettable for FRAMEDELAYMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x0480;
}
