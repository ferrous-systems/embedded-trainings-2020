#[doc = "Register `B2` reader"]
pub struct R(crate::R<B2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B2` writer"]
pub struct W(crate::W<B2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B2_SPEC>;
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
impl From<crate::W<B2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B2` reader - y-intercept of 3rd piece wise linear function"]
pub type B2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `B2` writer - y-intercept of 3rd piece wise linear function"]
pub type B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, B2_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn b2(&self) -> B2_R {
        B2_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn b2(&mut self) -> B2_W<0> {
        B2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "y-intercept of 3rd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b2](index.html) module"]
pub struct B2_SPEC;
impl crate::RegisterSpec for B2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b2::R](R) reader structure"]
impl crate::Readable for B2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b2::W](W) writer structure"]
impl crate::Writable for B2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets B2 to value 0x3f98"]
impl crate::Resettable for B2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f98;
}
