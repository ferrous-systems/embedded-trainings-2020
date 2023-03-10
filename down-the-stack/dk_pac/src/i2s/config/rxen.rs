#[doc = "Register `RXEN` reader"]
pub struct R(crate::R<RXEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXEN` writer"]
pub struct W(crate::W<RXEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXEN_SPEC>;
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
impl From<crate::W<RXEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXEN` reader - Reception (RX) enable."]
pub type RXEN_R = crate::BitReader<RXEN_A>;
#[doc = "Reception (RX) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEN_A {
    #[doc = "0: Reception disabled and now data will be written to the RXD.PTR address."]
    DISABLED = 0,
    #[doc = "1: Reception enabled."]
    ENABLED = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::DISABLED,
            true => RXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXEN_A::ENABLED
    }
}
#[doc = "Field `RXEN` writer - Reception (RX) enable."]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXEN_SPEC, RXEN_A, O>;
impl<'a, const O: u8> RXEN_W<'a, O> {
    #[doc = "Reception disabled and now data will be written to the RXD.PTR address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXEN_A::DISABLED)
    }
    #[doc = "Reception enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Reception (RX) enable."]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception (RX) enable."]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<0> {
        RXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception (RX) enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxen](index.html) module"]
pub struct RXEN_SPEC;
impl crate::RegisterSpec for RXEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxen::R](R) reader structure"]
impl crate::Readable for RXEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxen::W](W) writer structure"]
impl crate::Writable for RXEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXEN to value 0"]
impl crate::Resettable for RXEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
