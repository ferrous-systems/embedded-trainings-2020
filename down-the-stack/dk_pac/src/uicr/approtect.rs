#[doc = "Register `APPROTECT` reader"]
pub struct R(crate::R<APPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPROTECT` writer"]
pub struct W(crate::W<APPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPROTECT_SPEC>;
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
impl From<crate::W<APPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALL` reader - Enable or disable access port protection."]
pub type PALL_R = crate::FieldReader<u8, PALL_A>;
#[doc = "Enable or disable access port protection.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PALL_A {
    #[doc = "255: Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
    DISABLED = 255,
    #[doc = "90: Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
    HW_DISABLED = 90,
    #[doc = "0: Enable"]
    ENABLED = 0,
}
impl From<PALL_A> for u8 {
    #[inline(always)]
    fn from(variant: PALL_A) -> Self {
        variant as _
    }
}
impl PALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PALL_A> {
        match self.bits {
            255 => Some(PALL_A::DISABLED),
            90 => Some(PALL_A::HW_DISABLED),
            0 => Some(PALL_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HW_DISABLED`"]
    #[inline(always)]
    pub fn is_hw_disabled(&self) -> bool {
        *self == PALL_A::HW_DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PALL_A::ENABLED
    }
}
#[doc = "Field `PALL` writer - Enable or disable access port protection."]
pub type PALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APPROTECT_SPEC, u8, PALL_A, 8, O>;
impl<'a, const O: u8> PALL_W<'a, O> {
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PALL_A::DISABLED)
    }
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
    #[inline(always)]
    pub fn hw_disabled(self) -> &'a mut W {
        self.variant(PALL_A::HW_DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PALL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - Enable or disable access port protection."]
    #[inline(always)]
    pub fn pall(&self) -> PALL_R {
        PALL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable or disable access port protection."]
    #[inline(always)]
    #[must_use]
    pub fn pall(&mut self) -> PALL_W<0> {
        PALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [approtect](index.html) module"]
pub struct APPROTECT_SPEC;
impl crate::RegisterSpec for APPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [approtect::R](R) reader structure"]
impl crate::Readable for APPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [approtect::W](W) writer structure"]
impl crate::Writable for APPROTECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APPROTECT to value 0xffff_ffff"]
impl crate::Resettable for APPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
