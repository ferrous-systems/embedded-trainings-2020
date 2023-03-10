#[doc = "Register `PSELCTS` reader"]
pub struct R(crate::R<PSELCTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELCTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELCTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELCTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELCTS` writer"]
pub struct W(crate::W<PSELCTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELCTS_SPEC>;
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
impl From<crate::W<PSELCTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELCTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSELCTS` reader - Pin number configuration for UART CTS signal"]
pub type PSELCTS_R = crate::FieldReader<u32, PSELCTS_A>;
#[doc = "Pin number configuration for UART CTS signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PSELCTS_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELCTS_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELCTS_A) -> Self {
        variant as _
    }
}
impl PSELCTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELCTS_A> {
        match self.bits {
            4294967295 => Some(PSELCTS_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELCTS_A::DISCONNECTED
    }
}
#[doc = "Field `PSELCTS` writer - Pin number configuration for UART CTS signal"]
pub type PSELCTS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSELCTS_SPEC, u32, PSELCTS_A, 32, O>;
impl<'a, const O: u8> PSELCTS_W<'a, O> {
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELCTS_A::DISCONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    pub fn pselcts(&self) -> PSELCTS_R {
        PSELCTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    #[must_use]
    pub fn pselcts(&mut self) -> PSELCTS_W<0> {
        PSELCTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for CTS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselcts](index.html) module"]
pub struct PSELCTS_SPEC;
impl crate::RegisterSpec for PSELCTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselcts::R](R) reader structure"]
impl crate::Readable for PSELCTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselcts::W](W) writer structure"]
impl crate::Writable for PSELCTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSELCTS to value 0xffff_ffff"]
impl crate::Resettable for PSELCTS_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
