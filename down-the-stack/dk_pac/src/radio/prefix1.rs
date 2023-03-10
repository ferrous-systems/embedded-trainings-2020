#[doc = "Register `PREFIX1` reader"]
pub struct R(crate::R<PREFIX1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREFIX1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREFIX1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREFIX1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREFIX1` writer"]
pub struct W(crate::W<PREFIX1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREFIX1_SPEC>;
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
impl From<crate::W<PREFIX1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREFIX1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP4` reader - Address prefix 4."]
pub type AP4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP4` writer - Address prefix 4."]
pub type AP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX1_SPEC, u8, u8, 8, O>;
#[doc = "Field `AP5` reader - Address prefix 5."]
pub type AP5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP5` writer - Address prefix 5."]
pub type AP5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX1_SPEC, u8, u8, 8, O>;
#[doc = "Field `AP6` reader - Address prefix 6."]
pub type AP6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP6` writer - Address prefix 6."]
pub type AP6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX1_SPEC, u8, u8, 8, O>;
#[doc = "Field `AP7` reader - Address prefix 7."]
pub type AP7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP7` writer - Address prefix 7."]
pub type AP7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    pub fn ap4(&self) -> AP4_R {
        AP4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    pub fn ap5(&self) -> AP5_R {
        AP5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    pub fn ap6(&self) -> AP6_R {
        AP6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    pub fn ap7(&self) -> AP7_R {
        AP7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    #[must_use]
    pub fn ap4(&mut self) -> AP4_W<0> {
        AP4_W::new(self)
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    #[must_use]
    pub fn ap5(&mut self) -> AP5_W<8> {
        AP5_W::new(self)
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    #[must_use]
    pub fn ap6(&mut self) -> AP6_W<16> {
        AP6_W::new(self)
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    #[must_use]
    pub fn ap7(&mut self) -> AP7_W<24> {
        AP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prefixes bytes for logical addresses 4-7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prefix1](index.html) module"]
pub struct PREFIX1_SPEC;
impl crate::RegisterSpec for PREFIX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prefix1::R](R) reader structure"]
impl crate::Readable for PREFIX1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prefix1::W](W) writer structure"]
impl crate::Writable for PREFIX1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PREFIX1 to value 0"]
impl crate::Resettable for PREFIX1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
