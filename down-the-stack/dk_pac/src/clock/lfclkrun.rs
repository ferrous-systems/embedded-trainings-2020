#[doc = "Register `LFCLKRUN` reader"]
pub struct R(crate::R<LFCLKRUN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKRUN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKRUN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKRUN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATUS` reader - LFCLKSTART task triggered or not"]
pub type STATUS_R = crate::BitReader<STATUS_A>;
#[doc = "LFCLKSTART task triggered or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATUS_A {
    #[doc = "0: Task not triggered"]
    NOT_TRIGGERED = 0,
    #[doc = "1: Task triggered"]
    TRIGGERED = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl STATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NOT_TRIGGERED,
            true => STATUS_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_TRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == STATUS_A::NOT_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == STATUS_A::TRIGGERED
    }
}
impl R {
    #[doc = "Bit 0 - LFCLKSTART task triggered or not"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status indicating that LFCLKSTART task has been triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclkrun](index.html) module"]
pub struct LFCLKRUN_SPEC;
impl crate::RegisterSpec for LFCLKRUN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclkrun::R](R) reader structure"]
impl crate::Readable for LFCLKRUN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LFCLKRUN to value 0"]
impl crate::Resettable for LFCLKRUN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
