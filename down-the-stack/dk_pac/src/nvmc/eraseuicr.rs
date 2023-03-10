#[doc = "Register `ERASEUICR` reader"]
pub struct R(crate::R<ERASEUICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEUICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEUICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEUICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEUICR` writer"]
pub struct W(crate::W<ERASEUICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEUICR_SPEC>;
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
impl From<crate::W<ERASEUICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEUICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEUICR` reader - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
pub type ERASEUICR_R = crate::BitReader<ERASEUICR_A>;
#[doc = "Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERASEUICR_A {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Start erase of UICR"]
    ERASE = 1,
}
impl From<ERASEUICR_A> for bool {
    #[inline(always)]
    fn from(variant: ERASEUICR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERASEUICR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASEUICR_A {
        match self.bits {
            false => ERASEUICR_A::NO_OPERATION,
            true => ERASEUICR_A::ERASE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == ERASEUICR_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ERASEUICR_A::ERASE
    }
}
#[doc = "Field `ERASEUICR` writer - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
pub type ERASEUICR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERASEUICR_SPEC, ERASEUICR_A, O>;
impl<'a, const O: u8> ERASEUICR_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEUICR_A::NO_OPERATION)
    }
    #[doc = "Start erase of UICR"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEUICR_A::ERASE)
    }
}
impl R {
    #[doc = "Bit 0 - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&self) -> ERASEUICR_R {
        ERASEUICR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    #[must_use]
    pub fn eraseuicr(&mut self) -> ERASEUICR_W<0> {
        ERASEUICR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for erasing User Information Configuration Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseuicr](index.html) module"]
pub struct ERASEUICR_SPEC;
impl crate::RegisterSpec for ERASEUICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eraseuicr::R](R) reader structure"]
impl crate::Readable for ERASEUICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eraseuicr::W](W) writer structure"]
impl crate::Writable for ERASEUICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASEUICR to value 0"]
impl crate::Resettable for ERASEUICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
