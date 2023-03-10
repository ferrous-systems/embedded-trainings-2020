#[doc = "Register `HYST` reader"]
pub struct R(crate::R<HYST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HYST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HYST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HYST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HYST` writer"]
pub struct W(crate::W<HYST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HYST_SPEC>;
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
impl From<crate::W<HYST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HYST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYST` reader - Comparator hysteresis enable"]
pub type HYST_R = crate::BitReader<HYST_A>;
#[doc = "Comparator hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HYST_A {
    #[doc = "0: Comparator hysteresis disabled"]
    NO_HYST = 0,
    #[doc = "1: Comparator hysteresis disabled (typ. 50 mV)"]
    HYST50M_V = 1,
}
impl From<HYST_A> for bool {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        variant as u8 != 0
    }
}
impl HYST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            false => HYST_A::NO_HYST,
            true => HYST_A::HYST50M_V,
        }
    }
    #[doc = "Checks if the value of the field is `NO_HYST`"]
    #[inline(always)]
    pub fn is_no_hyst(&self) -> bool {
        *self == HYST_A::NO_HYST
    }
    #[doc = "Checks if the value of the field is `HYST50M_V`"]
    #[inline(always)]
    pub fn is_hyst50m_v(&self) -> bool {
        *self == HYST_A::HYST50M_V
    }
}
#[doc = "Field `HYST` writer - Comparator hysteresis enable"]
pub type HYST_W<'a, const O: u8> = crate::BitWriter<'a, u32, HYST_SPEC, HYST_A, O>;
impl<'a, const O: u8> HYST_W<'a, O> {
    #[doc = "Comparator hysteresis disabled"]
    #[inline(always)]
    pub fn no_hyst(self) -> &'a mut W {
        self.variant(HYST_A::NO_HYST)
    }
    #[doc = "Comparator hysteresis disabled (typ. 50 mV)"]
    #[inline(always)]
    pub fn hyst50m_v(self) -> &'a mut W {
        self.variant(HYST_A::HYST50M_V)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator hysteresis enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HYST_W<0> {
        HYST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator hysteresis enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hyst](index.html) module"]
pub struct HYST_SPEC;
impl crate::RegisterSpec for HYST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hyst::R](R) reader structure"]
impl crate::Readable for HYST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hyst::W](W) writer structure"]
impl crate::Writable for HYST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HYST to value 0"]
impl crate::Resettable for HYST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
