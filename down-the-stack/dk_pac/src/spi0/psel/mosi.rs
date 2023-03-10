#[doc = "Register `MOSI` reader"]
pub struct R(crate::R<MOSI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOSI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOSI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOSI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOSI` writer"]
pub struct W(crate::W<MOSI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOSI_SPEC>;
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
impl From<crate::W<MOSI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOSI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSELMOSI` reader - Pin number configuration for SPI MOSI signal"]
pub type PSELMOSI_R = crate::FieldReader<u32, PSELMOSI_A>;
#[doc = "Pin number configuration for SPI MOSI signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PSELMOSI_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELMOSI_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELMOSI_A) -> Self {
        variant as _
    }
}
impl PSELMOSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELMOSI_A> {
        match self.bits {
            4294967295 => Some(PSELMOSI_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELMOSI_A::DISCONNECTED
    }
}
#[doc = "Field `PSELMOSI` writer - Pin number configuration for SPI MOSI signal"]
pub type PSELMOSI_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MOSI_SPEC, u32, PSELMOSI_A, 32, O>;
impl<'a, const O: u8> PSELMOSI_W<'a, O> {
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELMOSI_A::DISCONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MOSI signal"]
    #[inline(always)]
    pub fn pselmosi(&self) -> PSELMOSI_R {
        PSELMOSI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MOSI signal"]
    #[inline(always)]
    #[must_use]
    pub fn pselmosi(&mut self) -> PSELMOSI_W<0> {
        PSELMOSI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for MOSI\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mosi](index.html) module"]
pub struct MOSI_SPEC;
impl crate::RegisterSpec for MOSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mosi::R](R) reader structure"]
impl crate::Readable for MOSI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mosi::W](W) writer structure"]
impl crate::Writable for MOSI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOSI to value 0xffff_ffff"]
impl crate::Resettable for MOSI_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
