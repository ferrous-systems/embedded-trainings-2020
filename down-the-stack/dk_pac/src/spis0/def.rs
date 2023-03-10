#[doc = "Register `DEF` reader"]
pub struct R(crate::R<DEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEF` writer"]
pub struct W(crate::W<DEF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEF_SPEC>;
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
impl From<crate::W<DEF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEF` reader - Default character. Character clocked out in case of an ignored transaction."]
pub type DEF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEF` writer - Default character. Character clocked out in case of an ignored transaction."]
pub type DEF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    #[must_use]
    pub fn def(&mut self) -> DEF_W<0> {
        DEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Default character. Character clocked out in case of an ignored transaction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [def](index.html) module"]
pub struct DEF_SPEC;
impl crate::RegisterSpec for DEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [def::R](R) reader structure"]
impl crate::Readable for DEF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [def::W](W) writer structure"]
impl crate::Writable for DEF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEF to value 0"]
impl crate::Resettable for DEF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
