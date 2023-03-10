#[doc = "Register `MICSTATUS` reader"]
pub struct R(crate::R<MICSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MICSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MICSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MICSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MICSTATUS` reader - The result of the MIC check performed during the previous decryption operation"]
pub type MICSTATUS_R = crate::BitReader<MICSTATUS_A>;
#[doc = "The result of the MIC check performed during the previous decryption operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MICSTATUS_A {
    #[doc = "0: MIC check failed"]
    CHECK_FAILED = 0,
    #[doc = "1: MIC check passed"]
    CHECK_PASSED = 1,
}
impl From<MICSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: MICSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl MICSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MICSTATUS_A {
        match self.bits {
            false => MICSTATUS_A::CHECK_FAILED,
            true => MICSTATUS_A::CHECK_PASSED,
        }
    }
    #[doc = "Checks if the value of the field is `CHECK_FAILED`"]
    #[inline(always)]
    pub fn is_check_failed(&self) -> bool {
        *self == MICSTATUS_A::CHECK_FAILED
    }
    #[doc = "Checks if the value of the field is `CHECK_PASSED`"]
    #[inline(always)]
    pub fn is_check_passed(&self) -> bool {
        *self == MICSTATUS_A::CHECK_PASSED
    }
}
impl R {
    #[doc = "Bit 0 - The result of the MIC check performed during the previous decryption operation"]
    #[inline(always)]
    pub fn micstatus(&self) -> MICSTATUS_R {
        MICSTATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "MIC check result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micstatus](index.html) module"]
pub struct MICSTATUS_SPEC;
impl crate::RegisterSpec for MICSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [micstatus::R](R) reader structure"]
impl crate::Readable for MICSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MICSTATUS to value 0"]
impl crate::Resettable for MICSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
