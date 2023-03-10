#[doc = "Register `RR[%s]` writer"]
pub struct W(crate::W<RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RR_SPEC>;
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
impl From<crate::W<RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Reload request register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RR_AW {
    #[doc = "1850885685: Value to request a reload of the watchdog timer"]
    RELOAD = 1850885685,
}
impl From<RR_AW> for u32 {
    #[inline(always)]
    fn from(variant: RR_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RR` writer - Reload request register"]
pub type RR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RR_SPEC, u32, RR_AW, 32, O>;
impl<'a, const O: u8> RR_W<'a, O> {
    #[doc = "Value to request a reload of the watchdog timer"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(RR_AW::RELOAD)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reload request register"]
    #[inline(always)]
    #[must_use]
    pub fn rr(&mut self) -> RR_W<0> {
        RR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Reload request n\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rr](index.html) module"]
pub struct RR_SPEC;
impl crate::RegisterSpec for RR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [rr::W](W) writer structure"]
impl crate::Writable for RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RR[%s]
to value 0"]
impl crate::Resettable for RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
