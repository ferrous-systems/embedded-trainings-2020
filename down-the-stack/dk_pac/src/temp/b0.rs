#[doc = "Register `B0` reader"]
pub struct R(crate::R<B0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<B0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<B0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<B0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `B0` writer"]
pub struct W(crate::W<B0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<B0_SPEC>;
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
impl From<crate::W<B0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<B0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B0` reader - y-intercept of 1st piece wise linear function"]
pub type B0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `B0` writer - y-intercept of 1st piece wise linear function"]
pub type B0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, B0_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub fn b0(&self) -> B0_R {
        B0_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn b0(&mut self) -> B0_W<0> {
        B0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "y-intercept of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0](index.html) module"]
pub struct B0_SPEC;
impl crate::RegisterSpec for B0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [b0::R](R) reader structure"]
impl crate::Readable for B0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [b0::W](W) writer structure"]
impl crate::Writable for B0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets B0 to value 0x3fcc"]
impl crate::Resettable for B0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fcc;
}
