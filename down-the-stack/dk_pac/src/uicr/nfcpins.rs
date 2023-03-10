#[doc = "Register `NFCPINS` reader"]
pub struct R(crate::R<NFCPINS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCPINS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCPINS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCPINS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCPINS` writer"]
pub struct W(crate::W<NFCPINS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCPINS_SPEC>;
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
impl From<crate::W<NFCPINS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NFCPINS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTECT` reader - Setting of pins dedicated to NFC functionality"]
pub type PROTECT_R = crate::BitReader<PROTECT_A>;
#[doc = "Setting of pins dedicated to NFC functionality\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROTECT_A {
    #[doc = "0: Operation as GPIO pins. Same protection as normal GPIO pins."]
    DISABLED = 0,
    #[doc = "1: Operation as NFC antenna pins. Configures the protection for NFC operation."]
    NFC = 1,
}
impl From<PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTECT_A {
        match self.bits {
            false => PROTECT_A::DISABLED,
            true => PROTECT_A::NFC,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PROTECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `NFC`"]
    #[inline(always)]
    pub fn is_nfc(&self) -> bool {
        *self == PROTECT_A::NFC
    }
}
#[doc = "Field `PROTECT` writer - Setting of pins dedicated to NFC functionality"]
pub type PROTECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, NFCPINS_SPEC, PROTECT_A, O>;
impl<'a, const O: u8> PROTECT_W<'a, O> {
    #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PROTECT_A::DISABLED)
    }
    #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation."]
    #[inline(always)]
    pub fn nfc(self) -> &'a mut W {
        self.variant(PROTECT_A::NFC)
    }
}
impl R {
    #[doc = "Bit 0 - Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<0> {
        PROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcpins](index.html) module"]
pub struct NFCPINS_SPEC;
impl crate::RegisterSpec for NFCPINS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcpins::R](R) reader structure"]
impl crate::Readable for NFCPINS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcpins::W](W) writer structure"]
impl crate::Writable for NFCPINS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NFCPINS to value 0xffff_ffff"]
impl crate::Resettable for NFCPINS_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
