#[doc = "Register `DAP[%s]` reader"]
pub struct R(crate::R<DAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAP[%s]` writer"]
pub struct W(crate::W<DAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAP_SPEC>;
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
impl From<crate::W<DAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAP` reader - Device address prefix n"]
pub type DAP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DAP` writer - Device address prefix n"]
pub type DAP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DAP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Device address prefix n"]
    #[inline(always)]
    pub fn dap(&self) -> DAP_R {
        DAP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Device address prefix n"]
    #[inline(always)]
    #[must_use]
    pub fn dap(&mut self) -> DAP_W<0> {
        DAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Device address prefix n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dap](index.html) module"]
pub struct DAP_SPEC;
impl crate::RegisterSpec for DAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dap::R](R) reader structure"]
impl crate::Readable for DAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dap::W](W) writer structure"]
impl crate::Writable for DAP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAP[%s]
to value 0"]
impl crate::Resettable for DAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
