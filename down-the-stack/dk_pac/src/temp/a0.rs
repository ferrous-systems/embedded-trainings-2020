#[doc = "Register `A0` reader"]
pub struct R(crate::R<A0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A0` writer"]
pub struct W(crate::W<A0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A0_SPEC>;
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
impl From<crate::W<A0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A0` reader - Slope of 1st piece wise linear function"]
pub type A0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `A0` writer - Slope of 1st piece wise linear function"]
pub type A0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, A0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Slope of 1st piece wise linear function"]
    #[inline(always)]
    pub fn a0(&self) -> A0_R {
        A0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 1st piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn a0(&mut self) -> A0_W<0> {
        A0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a0](index.html) module"]
pub struct A0_SPEC;
impl crate::RegisterSpec for A0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a0::R](R) reader structure"]
impl crate::Readable for A0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a0::W](W) writer structure"]
impl crate::Writable for A0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A0 to value 0x0320"]
impl crate::Resettable for A0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0320;
}
