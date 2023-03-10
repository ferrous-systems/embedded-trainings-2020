#[doc = "Register `MCKEN` reader"]
pub struct R(crate::R<MCKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCKEN` writer"]
pub struct W(crate::W<MCKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCKEN_SPEC>;
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
impl From<crate::W<MCKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCKEN` reader - Master clock generator enable."]
pub type MCKEN_R = crate::BitReader<MCKEN_A>;
#[doc = "Master clock generator enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCKEN_A {
    #[doc = "0: Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    DISABLED = 0,
    #[doc = "1: Master clock generator running and MCK output on PSEL.MCK."]
    ENABLED = 1,
}
impl From<MCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKEN_A {
        match self.bits {
            false => MCKEN_A::DISABLED,
            true => MCKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKEN_A::ENABLED
    }
}
#[doc = "Field `MCKEN` writer - Master clock generator enable."]
pub type MCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCKEN_SPEC, MCKEN_A, O>;
impl<'a, const O: u8> MCKEN_W<'a, O> {
    #[doc = "Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKEN_A::DISABLED)
    }
    #[doc = "Master clock generator running and MCK output on PSEL.MCK."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Master clock generator enable."]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master clock generator enable."]
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MCKEN_W<0> {
        MCKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master clock generator enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcken](index.html) module"]
pub struct MCKEN_SPEC;
impl crate::RegisterSpec for MCKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcken::R](R) reader structure"]
impl crate::Readable for MCKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcken::W](W) writer structure"]
impl crate::Writable for MCKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCKEN to value 0x01"]
impl crate::Resettable for MCKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
