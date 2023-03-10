#[doc = "Register `TXEN` reader"]
pub struct R(crate::R<TXEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXEN` writer"]
pub struct W(crate::W<TXEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXEN_SPEC>;
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
impl From<crate::W<TXEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXEN` reader - Transmission (TX) enable."]
pub type TXEN_R = crate::BitReader<TXEN_A>;
#[doc = "Transmission (TX) enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEN_A {
    #[doc = "0: Transmission disabled and now data will be read from the RXD.TXD address."]
    DISABLED = 0,
    #[doc = "1: Transmission enabled."]
    ENABLED = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::DISABLED,
            true => TXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEN_A::ENABLED
    }
}
#[doc = "Field `TXEN` writer - Transmission (TX) enable."]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXEN_SPEC, TXEN_A, O>;
impl<'a, const O: u8> TXEN_W<'a, O> {
    #[doc = "Transmission disabled and now data will be read from the RXD.TXD address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEN_A::DISABLED)
    }
    #[doc = "Transmission enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Transmission (TX) enable."]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission (TX) enable."]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<0> {
        TXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmission (TX) enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txen](index.html) module"]
pub struct TXEN_SPEC;
impl crate::RegisterSpec for TXEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txen::R](R) reader structure"]
impl crate::Readable for TXEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txen::W](W) writer structure"]
impl crate::Writable for TXEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXEN to value 0x01"]
impl crate::Resettable for TXEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
