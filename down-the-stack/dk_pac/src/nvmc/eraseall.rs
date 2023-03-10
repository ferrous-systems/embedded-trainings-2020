#[doc = "Register `ERASEALL` reader"]
pub struct R(crate::R<ERASEALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASEALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASEALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASEALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASEALL` writer"]
pub struct W(crate::W<ERASEALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASEALL_SPEC>;
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
impl From<crate::W<ERASEALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASEALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERASEALL` reader - Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
pub type ERASEALL_R = crate::BitReader<ERASEALL_A>;
#[doc = "Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERASEALL_A {
    #[doc = "0: No operation"]
    NO_OPERATION = 0,
    #[doc = "1: Start chip erase"]
    ERASE = 1,
}
impl From<ERASEALL_A> for bool {
    #[inline(always)]
    fn from(variant: ERASEALL_A) -> Self {
        variant as u8 != 0
    }
}
impl ERASEALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASEALL_A {
        match self.bits {
            false => ERASEALL_A::NO_OPERATION,
            true => ERASEALL_A::ERASE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == ERASEALL_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ERASEALL_A::ERASE
    }
}
#[doc = "Field `ERASEALL` writer - Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
pub type ERASEALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERASEALL_SPEC, ERASEALL_A, O>;
impl<'a, const O: u8> ERASEALL_W<'a, O> {
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEALL_A::NO_OPERATION)
    }
    #[doc = "Start chip erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEALL_A::ERASE)
    }
}
impl R {
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseall(&self) -> ERASEALL_R {
        ERASEALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    #[must_use]
    pub fn eraseall(&mut self) -> ERASEALL_W<0> {
        ERASEALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register for erasing all non-volatile user memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eraseall](index.html) module"]
pub struct ERASEALL_SPEC;
impl crate::RegisterSpec for ERASEALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eraseall::R](R) reader structure"]
impl crate::Readable for ERASEALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eraseall::W](W) writer structure"]
impl crate::Writable for ERASEALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASEALL to value 0"]
impl crate::Resettable for ERASEALL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
