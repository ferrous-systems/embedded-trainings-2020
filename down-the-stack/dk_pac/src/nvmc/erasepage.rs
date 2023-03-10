#[doc = "Register `ERASEPAGE` reader"]
pub struct R(crate::R<ERASEPAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEPAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEPAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEPAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEPAGE` writer"]
pub struct W(crate::W<ERASEPAGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEPAGE_SPEC>;
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
impl From<crate::W<ERASEPAGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEPAGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEPAGE` reader - Register for starting erase of a page in Code area"]
pub type ERASEPAGE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ERASEPAGE` writer - Register for starting erase of a page in Code area"]
pub type ERASEPAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ERASEPAGE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Register for starting erase of a page in Code area"]
    #[inline(always)]
    pub fn erasepage(&self) -> ERASEPAGE_R {
        ERASEPAGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register for starting erase of a page in Code area"]
    #[inline(always)]
    #[must_use]
    pub fn erasepage(&mut self) -> ERASEPAGE_W<0> {
        ERASEPAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for erasing a page in Code area\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erasepage](index.html) module"]
pub struct ERASEPAGE_SPEC;
impl crate::RegisterSpec for ERASEPAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erasepage::R](R) reader structure"]
impl crate::Readable for ERASEPAGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erasepage::W](W) writer structure"]
impl crate::Writable for ERASEPAGE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASEPAGE to value 0"]
impl crate::Resettable for ERASEPAGE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
