#[doc = "Register `PSELRXD` reader"]
pub struct R(crate::R<PSELRXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELRXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELRXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELRXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELRXD` writer"]
pub struct W(crate::W<PSELRXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELRXD_SPEC>;
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
impl From<crate::W<PSELRXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELRXD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSELRXD` reader - Pin number configuration for UART RXD signal"]
pub type PSELRXD_R = crate::FieldReader<u32, PSELRXD_A>;
#[doc = "Pin number configuration for UART RXD signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PSELRXD_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELRXD_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELRXD_A) -> Self {
        variant as _
    }
}
impl PSELRXD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELRXD_A> {
        match self.bits {
            4294967295 => Some(PSELRXD_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELRXD_A::DISCONNECTED
    }
}
#[doc = "Field `PSELRXD` writer - Pin number configuration for UART RXD signal"]
pub type PSELRXD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSELRXD_SPEC, u32, PSELRXD_A, 32, O>;
impl<'a, const O: u8> PSELRXD_W<'a, O> {
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELRXD_A::DISCONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART RXD signal"]
    #[inline(always)]
    pub fn pselrxd(&self) -> PSELRXD_R {
        PSELRXD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART RXD signal"]
    #[inline(always)]
    #[must_use]
    pub fn pselrxd(&mut self) -> PSELRXD_W<0> {
        PSELRXD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for RXD\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselrxd](index.html) module"]
pub struct PSELRXD_SPEC;
impl crate::RegisterSpec for PSELRXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselrxd::R](R) reader structure"]
impl crate::Readable for PSELRXD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselrxd::W](W) writer structure"]
impl crate::Writable for PSELRXD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSELRXD to value 0xffff_ffff"]
impl crate::Resettable for PSELRXD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
