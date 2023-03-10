#[doc = "Register `RUNSTATUS` reader"]
pub struct R(crate::R<RUNSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RUNSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RUNSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RUNSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RUNSTATUS` reader - Indicates whether or not the watchdog is running"]
pub type RUNSTATUS_R = crate::BitReader<RUNSTATUS_A>;
#[doc = "Indicates whether or not the watchdog is running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUNSTATUS_A {
    #[doc = "0: Watchdog not running"]
    NOT_RUNNING = 0,
    #[doc = "1: Watchdog is running"]
    RUNNING = 1,
}
impl From<RUNSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl RUNSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNSTATUS_A {
        match self.bits {
            false => RUNSTATUS_A::NOT_RUNNING,
            true => RUNSTATUS_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == RUNSTATUS_A::NOT_RUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUNSTATUS_A::RUNNING
    }
}
impl R {
    #[doc = "Bit 0 - Indicates whether or not the watchdog is running"]
    #[inline(always)]
    pub fn runstatus(&self) -> RUNSTATUS_R {
        RUNSTATUS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Run status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [runstatus](index.html) module"]
pub struct RUNSTATUS_SPEC;
impl crate::RegisterSpec for RUNSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [runstatus::R](R) reader structure"]
impl crate::Readable for RUNSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RUNSTATUS to value 0"]
impl crate::Resettable for RUNSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
