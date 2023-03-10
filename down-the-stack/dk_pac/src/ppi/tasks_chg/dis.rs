#[doc = "Register `DIS` writer"]
pub struct W(crate::W<DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIS_SPEC>;
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
impl From<crate::W<DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Disable channel group n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<DIS_AW> for bool {
    #[inline(always)]
    fn from(variant: DIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIS` writer - Disable channel group n"]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIS_SPEC, DIS_AW, O>;
impl<'a, const O: u8> DIS_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(DIS_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Disable channel group n"]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DIS_W<0> {
        DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Disable channel group n\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dis](index.html) module"]
pub struct DIS_SPEC;
impl crate::RegisterSpec for DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dis::W](W) writer structure"]
impl crate::Writable for DIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIS to value 0"]
impl crate::Resettable for DIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
