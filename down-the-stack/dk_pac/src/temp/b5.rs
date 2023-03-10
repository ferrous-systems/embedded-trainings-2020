#[doc = "Register `B5` reader"]
pub struct R(crate::R<B5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B5` writer"]
pub struct W(crate::W<B5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B5_SPEC>;
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
impl From<crate::W<B5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B5` reader - y-intercept of 6th piece wise linear function"]
pub type B5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `B5` writer - y-intercept of 6th piece wise linear function"]
pub type B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, B5_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    pub fn b5(&self) -> B5_R {
        B5_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn b5(&mut self) -> B5_W<0> {
        B5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "y-intercept of 6th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b5](index.html) module"]
pub struct B5_SPEC;
impl crate::RegisterSpec for B5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b5::R](R) reader structure"]
impl crate::Readable for B5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b5::W](W) writer structure"]
impl crate::Writable for B5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets B5 to value 0x3dd0"]
impl crate::Resettable for B5_SPEC {
    const RESET_VALUE: Self::Ux = 0x3dd0;
}
