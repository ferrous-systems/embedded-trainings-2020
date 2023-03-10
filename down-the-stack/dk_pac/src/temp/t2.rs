#[doc = "Register `T2` reader"]
pub struct R(crate::R<T2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T2` writer"]
pub struct W(crate::W<T2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T2_SPEC>;
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
impl From<crate::W<T2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T2` reader - End point of 3rd piece wise linear function"]
pub type T2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T2` writer - End point of 3rd piece wise linear function"]
pub type T2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - End point of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn t2(&self) -> T2_R {
        T2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 3rd piece wise linear function"]
    #[inline(always)]
    #[must_use]
    pub fn t2(&mut self) -> T2_W<0> {
        T2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End point of 3rd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2](index.html) module"]
pub struct T2_SPEC;
impl crate::RegisterSpec for T2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t2::R](R) reader structure"]
impl crate::Readable for T2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t2::W](W) writer structure"]
impl crate::Writable for T2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T2 to value 0x14"]
impl crate::Resettable for T2_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
