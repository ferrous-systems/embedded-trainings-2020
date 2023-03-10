#[doc = "Register `A5` reader"]
pub struct R(crate::R<A5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A5` writer"]
pub struct W(crate::W<A5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A5_SPEC>;
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
impl From<crate::W<A5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A5` reader - Slope of 6th piece wise linear function"]
pub type A5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A5` writer - Slope of 6th piece wise linear function"]
pub type A5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, A5_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub fn a5(&self) -> A5_R {
        A5_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 6th piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn a5(&mut self) -> A5_W<0> {
        A5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope of 6th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a5](index.html) module"]
pub struct A5_SPEC;
impl crate::RegisterSpec for A5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a5::R](R) reader structure"]
impl crate::Readable for A5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a5::W](W) writer structure"]
impl crate::Writable for A5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A5 to value 0x037b"]
impl crate::Resettable for A5_SPEC {
    const RESET_VALUE: Self::Ux = 0x037b;
}
