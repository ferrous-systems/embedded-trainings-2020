#[doc = "Register `T3` reader"]
pub struct R(crate::R<T3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T3` writer"]
pub struct W(crate::W<T3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T3_SPEC>;
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
impl From<crate::W<T3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T3` reader - End point of 4th piece wise linear function"]
pub type T3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T3` writer - End point of 4th piece wise linear function"]
pub type T3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(&self) -> T3_R {
        T3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn t3(&mut self) -> T3_W<0> {
        T3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End point of 4th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3](index.html) module"]
pub struct T3_SPEC;
impl crate::RegisterSpec for T3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t3::R](R) reader structure"]
impl crate::Readable for T3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t3::W](W) writer structure"]
impl crate::Writable for T3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T3 to value 0x19"]
impl crate::Resettable for T3_SPEC {
    const RESET_VALUE: Self::Ux = 0x19;
}
