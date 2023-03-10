#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDECB` reader - Write '1' to disable interrupt for event ENDECB"]
pub type ENDECB_R = crate::BitReader<ENDECB_A>;
#[doc = "Write '1' to disable interrupt for event ENDECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDECB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDECB_A> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDECB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDECB_A {
        match self.bits {
            false => ENDECB_A::DISABLED,
            true => ENDECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDECB_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event ENDECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDECB_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` writer - Write '1' to disable interrupt for event ENDECB"]
pub type ENDECB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, ENDECB_AW, O>;
impl<'a, const O: u8> ENDECB_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDECB_AW::CLEAR)
    }
}
#[doc = "Field `ERRORECB` reader - Write '1' to disable interrupt for event ERRORECB"]
pub type ERRORECB_R = crate::BitReader<ERRORECB_A>;
#[doc = "Write '1' to disable interrupt for event ERRORECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRORECB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERRORECB_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRORECB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORECB_A {
        match self.bits {
            false => ERRORECB_A::DISABLED,
            true => ERRORECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRORECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRORECB_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event ERRORECB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRORECB_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ERRORECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` writer - Write '1' to disable interrupt for event ERRORECB"]
pub type ERRORECB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENCLR_SPEC, ERRORECB_AW, O>;
impl<'a, const O: u8> ERRORECB_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRORECB_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event ENDECB"]
    #[inline(always)]
    pub fn endecb(&self) -> ENDECB_R {
        ENDECB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ERRORECB"]
    #[inline(always)]
    pub fn errorecb(&self) -> ERRORECB_R {
        ERRORECB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event ENDECB"]
    #[inline(always)]
    #[must_use]
    pub fn endecb(&mut self) -> ENDECB_W<0> {
        ENDECB_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ERRORECB"]
    #[inline(always)]
    #[must_use]
    pub fn errorecb(&mut self) -> ERRORECB_W<1> {
        ERRORECB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
