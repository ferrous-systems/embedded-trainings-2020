#[doc = "Register `TH` reader"]
pub struct R(crate::R<TH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TH` writer"]
pub struct W(crate::W<TH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TH_SPEC>;
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
impl From<crate::W<TH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THDOWN` reader - VDOWN = (THDOWN+1)/64*VREF"]
pub type THDOWN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THDOWN` writer - VDOWN = (THDOWN+1)/64*VREF"]
pub type THDOWN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TH_SPEC, u8, u8, 6, O>;
#[doc = "Field `THUP` reader - VUP = (THUP+1)/64*VREF"]
pub type THUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THUP` writer - VUP = (THUP+1)/64*VREF"]
pub type THUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TH_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    pub fn thdown(&self) -> THDOWN_R {
        THDOWN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    pub fn thup(&self) -> THUP_R {
        THUP_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - VDOWN = (THDOWN+1)/64*VREF"]
    #[inline(always)]
    #[must_use]
    pub fn thdown(&mut self) -> THDOWN_W<0> {
        THDOWN_W::new(self)
    }
    #[doc = "Bits 8:13 - VUP = (THUP+1)/64*VREF"]
    #[inline(always)]
    #[must_use]
    pub fn thup(&mut self) -> THUP_W<8> {
        THUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Threshold configuration for hysteresis unit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [th](index.html) module"]
pub struct TH_SPEC;
impl crate::RegisterSpec for TH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [th::R](R) reader structure"]
impl crate::Readable for TH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [th::W](W) writer structure"]
impl crate::Writable for TH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TH to value 0"]
impl crate::Resettable for TH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
