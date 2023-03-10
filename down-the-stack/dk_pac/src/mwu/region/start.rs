#[doc = "Register `START` reader"]
pub struct R(crate::R<START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `START` writer"]
pub struct W(crate::W<START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_SPEC>;
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
impl From<crate::W<START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Start address for region"]
pub type START_R = crate::FieldReader<u32, u32>;
#[doc = "Field `START` writer - Start address for region"]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, START_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start address for region"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for region"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Start address for region n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start](index.html) module"]
pub struct START_SPEC;
impl crate::RegisterSpec for START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [start::R](R) reader structure"]
impl crate::Readable for START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [start::W](W) writer structure"]
impl crate::Writable for START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
