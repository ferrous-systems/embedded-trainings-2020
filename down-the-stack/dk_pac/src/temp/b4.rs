#[doc = "Register `B4` reader"]
pub struct R(crate::R<B4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B4` writer"]
pub struct W(crate::W<B4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B4_SPEC>;
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
impl From<crate::W<B4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B4` reader - y-intercept of 5th piece wise linear function"]
pub type B4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `B4` writer - y-intercept of 5th piece wise linear function"]
pub type B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, B4_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    pub fn b4(&self) -> B4_R {
        B4_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn b4(&mut self) -> B4_W<0> {
        B4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "y-intercept of 5th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b4](index.html) module"]
pub struct B4_SPEC;
impl crate::RegisterSpec for B4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b4::R](R) reader structure"]
impl crate::Readable for B4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b4::W](W) writer structure"]
impl crate::Writable for B4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets B4 to value 0x6a"]
impl crate::Resettable for B4_SPEC {
    const RESET_VALUE: Self::Ux = 0x6a;
}
