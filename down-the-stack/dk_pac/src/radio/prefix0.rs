#[doc = "Register `PREFIX0` reader"]
pub struct R(crate::R<PREFIX0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PREFIX0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PREFIX0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PREFIX0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PREFIX0` writer"]
pub struct W(crate::W<PREFIX0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PREFIX0_SPEC>;
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
impl From<crate::W<PREFIX0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PREFIX0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP0` reader - Address prefix 0."]
pub type AP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP0` writer - Address prefix 0."]
pub type AP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX0_SPEC, u8, u8, 8, O>;
#[doc = "Field `AP1` reader - Address prefix 1."]
pub type AP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP1` writer - Address prefix 1."]
pub type AP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX0_SPEC, u8, u8, 8, O>;
#[doc = "Field `AP2` reader - Address prefix 2."]
pub type AP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP2` writer - Address prefix 2."]
pub type AP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX0_SPEC, u8, u8, 8, O>;
#[doc = "Field `AP3` reader - Address prefix 3."]
pub type AP3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP3` writer - Address prefix 3."]
pub type AP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PREFIX0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    pub fn ap0(&self) -> AP0_R {
        AP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    pub fn ap1(&self) -> AP1_R {
        AP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    pub fn ap2(&self) -> AP2_R {
        AP2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    pub fn ap3(&self) -> AP3_R {
        AP3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    #[must_use]
    pub fn ap0(&mut self) -> AP0_W<0> {
        AP0_W::new(self)
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    #[must_use]
    pub fn ap1(&mut self) -> AP1_W<8> {
        AP1_W::new(self)
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    #[must_use]
    pub fn ap2(&mut self) -> AP2_W<16> {
        AP2_W::new(self)
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    #[must_use]
    pub fn ap3(&mut self) -> AP3_W<24> {
        AP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prefixes bytes for logical addresses 0-3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prefix0](index.html) module"]
pub struct PREFIX0_SPEC;
impl crate::RegisterSpec for PREFIX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prefix0::R](R) reader structure"]
impl crate::Readable for PREFIX0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prefix0::W](W) writer structure"]
impl crate::Writable for PREFIX0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PREFIX0 to value 0"]
impl crate::Resettable for PREFIX0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
