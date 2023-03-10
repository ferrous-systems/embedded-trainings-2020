#[doc = "Register `A2` reader"]
pub struct R(crate::R<A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A2` writer"]
pub struct W(crate::W<A2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A2_SPEC>;
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
impl From<crate::W<A2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A2` reader - Slope of 3rd piece wise linear function"]
pub type A2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A2` writer - Slope of 3rd piece wise linear function"]
pub type A2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, A2_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn a2(&self) -> A2_R {
        A2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 3rd piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn a2(&mut self) -> A2_W<0> {
        A2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope of 3rd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a2](index.html) module"]
pub struct A2_SPEC;
impl crate::RegisterSpec for A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a2::R](R) reader structure"]
impl crate::Readable for A2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a2::W](W) writer structure"]
impl crate::Writable for A2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A2 to value 0x035d"]
impl crate::Resettable for A2_SPEC {
    const RESET_VALUE: Self::Ux = 0x035d;
}
