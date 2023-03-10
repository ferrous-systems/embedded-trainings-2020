#[doc = "Register `PSELSCL` reader"]
pub struct R(crate::R<PSELSCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSELSCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSELSCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSELSCL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSELSCL` writer"]
pub struct W(crate::W<PSELSCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSELSCL_SPEC>;
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
impl From<crate::W<PSELSCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSELSCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSELSCL` reader - Pin number configuration for TWI SCL signal"]
pub type PSELSCL_R = crate::FieldReader<u32, PSELSCL_A>;
#[doc = "Pin number configuration for TWI SCL signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PSELSCL_A {
    #[doc = "4294967295: Disconnect"]
    DISCONNECTED = 4294967295,
}
impl From<PSELSCL_A> for u32 {
    #[inline(always)]
    fn from(variant: PSELSCL_A) -> Self {
        variant as _
    }
}
impl PSELSCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSELSCL_A> {
        match self.bits {
            4294967295 => Some(PSELSCL_A::DISCONNECTED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELSCL_A::DISCONNECTED
    }
}
#[doc = "Field `PSELSCL` writer - Pin number configuration for TWI SCL signal"]
pub type PSELSCL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PSELSCL_SPEC, u32, PSELSCL_A, 32, O>;
impl<'a, const O: u8> PSELSCL_W<'a, O> {
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELSCL_A::DISCONNECTED)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline(always)]
    pub fn pselscl(&self) -> PSELSCL_R {
        PSELSCL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline(always)]
    #[must_use]
    pub fn pselscl(&mut self) -> PSELSCL_W<0> {
        PSELSCL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin select for SCL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselscl](index.html) module"]
pub struct PSELSCL_SPEC;
impl crate::RegisterSpec for PSELSCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pselscl::R](R) reader structure"]
impl crate::Readable for PSELSCL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pselscl::W](W) writer structure"]
impl crate::Writable for PSELSCL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSELSCL to value 0xffff_ffff"]
impl crate::Resettable for PSELSCL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
