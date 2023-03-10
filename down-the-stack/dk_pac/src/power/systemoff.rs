#[doc = "Register `SYSTEMOFF` writer"]
pub struct W(crate::W<SYSTEMOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTEMOFF_SPEC>;
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
impl From<crate::W<SYSTEMOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTEMOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable System OFF mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSTEMOFF_AW {
    #[doc = "1: Enable System OFF mode"]
    ENTER = 1,
}
impl From<SYSTEMOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSTEMOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEMOFF` writer - Enable System OFF mode"]
pub type SYSTEMOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSTEMOFF_SPEC, SYSTEMOFF_AW, O>;
impl<'a, const O: u8> SYSTEMOFF_W<'a, O> {
    #[doc = "Enable System OFF mode"]
    #[inline(always)]
    pub fn enter(self) -> &'a mut W {
        self.variant(SYSTEMOFF_AW::ENTER)
    }
}
impl W {
    #[doc = "Bit 0 - Enable System OFF mode"]
    #[inline(always)]
    #[must_use]
    pub fn systemoff(&mut self) -> SYSTEMOFF_W<0> {
        SYSTEMOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System OFF register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systemoff](index.html) module"]
pub struct SYSTEMOFF_SPEC;
impl crate::RegisterSpec for SYSTEMOFF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [systemoff::W](W) writer structure"]
impl crate::Writable for SYSTEMOFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSTEMOFF to value 0"]
impl crate::Resettable for SYSTEMOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
