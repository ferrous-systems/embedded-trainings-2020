#[doc = "Register `GPREGRET` reader"]
pub struct R(crate::R<GPREGRET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPREGRET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPREGRET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPREGRET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPREGRET` writer"]
pub struct W(crate::W<GPREGRET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPREGRET_SPEC>;
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
impl From<crate::W<GPREGRET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPREGRET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPREGRET` reader - General purpose retention register"]
pub type GPREGRET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPREGRET` writer - General purpose retention register"]
pub type GPREGRET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPREGRET_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    pub fn gpregret(&self) -> GPREGRET_R {
        GPREGRET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - General purpose retention register"]
    #[inline(always)]
    #[must_use]
    pub fn gpregret(&mut self) -> GPREGRET_W<0> {
        GPREGRET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General purpose retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpregret](index.html) module"]
pub struct GPREGRET_SPEC;
impl crate::RegisterSpec for GPREGRET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpregret::R](R) reader structure"]
impl crate::Readable for GPREGRET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpregret::W](W) writer structure"]
impl crate::Writable for GPREGRET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPREGRET to value 0"]
impl crate::Resettable for GPREGRET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
