#[doc = "Register `LIMIT` reader"]
pub struct R(crate::R<LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMIT` writer"]
pub struct W(crate::W<LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMIT_SPEC>;
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
impl From<crate::W<LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW` reader - Low level limit"]
pub type LOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOW` writer - Low level limit"]
pub type LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LIMIT_SPEC, u16, u16, 16, O>;
#[doc = "Field `HIGH` reader - High level limit"]
pub type HIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HIGH` writer - High level limit"]
pub type HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LIMIT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Low level limit"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High level limit"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low level limit"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<0> {
        LOW_W::new(self)
    }
    #[doc = "Bits 16:31 - High level limit"]
    #[inline(always)]
    #[must_use]
    pub fn high(&mut self) -> HIGH_W<16> {
        HIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: High/low limits for event monitoring a channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit](index.html) module"]
pub struct LIMIT_SPEC;
impl crate::RegisterSpec for LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limit::R](R) reader structure"]
impl crate::Readable for LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limit::W](W) writer structure"]
impl crate::Writable for LIMIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LIMIT to value 0x7fff_8000"]
impl crate::Resettable for LIMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_8000;
}
