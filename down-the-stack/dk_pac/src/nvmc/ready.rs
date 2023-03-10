#[doc = "Register `READY` reader"]
pub struct R(crate::R<READY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READY` reader - NVMC is ready or busy"]
pub type READY_R = crate::BitReader<READY_A>;
#[doc = "NVMC is ready or busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_A {
    #[doc = "0: NVMC is busy (on-going write or erase operation)"]
    BUSY = 0,
    #[doc = "1: NVMC is ready"]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::BUSY,
            true => READY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == READY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == READY_A::READY
    }
}
impl R {
    #[doc = "Bit 0 - NVMC is ready or busy"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Ready flag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ready](index.html) module"]
pub struct READY_SPEC;
impl crate::RegisterSpec for READY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ready::R](R) reader structure"]
impl crate::Readable for READY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets READY to value 0"]
impl crate::Resettable for READY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
