#[doc = "Register `TXD` writer"]
pub struct W(crate::W<TXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXD_SPEC>;
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
impl From<crate::W<TXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXD` writer - TX data to be transferred"]
pub type TXD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXD_SPEC, u8, u8, 8, O>;
impl W {
    #[doc = "Bits 0:7 - TX data to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn txd(&mut self) -> TXD_W<0> {
        TXD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXD register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd](index.html) module"]
pub struct TXD_SPEC;
impl crate::RegisterSpec for TXD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [txd::W](W) writer structure"]
impl crate::Writable for TXD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXD to value 0"]
impl crate::Resettable for TXD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
