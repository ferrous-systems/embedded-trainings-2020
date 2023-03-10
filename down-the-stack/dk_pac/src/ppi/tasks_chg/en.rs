#[doc = "Register `EN` writer"]
pub struct W(crate::W<EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EN_SPEC>;
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
impl From<crate::W<EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable channel group n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<EN_AW> for bool {
    #[inline(always)]
    fn from(variant: EN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` writer - Enable channel group n"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EN_SPEC, EN_AW, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(EN_AW::TRIGGER)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel group n"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Enable channel group n\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [en](index.html) module"]
pub struct EN_SPEC;
impl crate::RegisterSpec for EN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [en::W](W) writer structure"]
impl crate::Writable for EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EN to value 0"]
impl crate::Resettable for EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
