#[doc = "Register `REQSTATUS` reader"]
pub struct R(crate::R<REQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RR0` reader - Request status for RR\\[0\\]
register"]
pub type RR0_R = crate::BitReader<RR0_A>;
#[doc = "Request status for RR\\[0\\]
register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR0_A {
    #[doc = "0: RR\\[0\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[0\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR0_A> for bool {
    #[inline(always)]
    fn from(variant: RR0_A) -> Self {
        variant as u8 != 0
    }
}
impl RR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR0_A {
        match self.bits {
            false => RR0_A::DISABLED_OR_REQUESTED,
            true => RR0_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR0_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR0_A::ENABLED_AND_UNREQUESTED
    }
}
#[doc = "Field `RR1` reader - Request status for RR\\[1\\]
register"]
pub type RR1_R = crate::BitReader<RR1_A>;
#[doc = "Request status for RR\\[1\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR1_A {
    #[doc = "0: RR\\[1\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[1\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR1_A> for bool {
    #[inline(always)]
    fn from(variant: RR1_A) -> Self {
        variant as u8 != 0
    }
}
impl RR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR1_A {
        match self.bits {
            false => RR1_A::DISABLED_OR_REQUESTED,
            true => RR1_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR1_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR1_A::ENABLED_AND_UNREQUESTED
    }
}
#[doc = "Field `RR2` reader - Request status for RR\\[2\\]
register"]
pub type RR2_R = crate::BitReader<RR2_A>;
#[doc = "Request status for RR\\[2\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR2_A {
    #[doc = "0: RR\\[2\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[2\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR2_A> for bool {
    #[inline(always)]
    fn from(variant: RR2_A) -> Self {
        variant as u8 != 0
    }
}
impl RR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR2_A {
        match self.bits {
            false => RR2_A::DISABLED_OR_REQUESTED,
            true => RR2_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR2_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR2_A::ENABLED_AND_UNREQUESTED
    }
}
#[doc = "Field `RR3` reader - Request status for RR\\[3\\]
register"]
pub type RR3_R = crate::BitReader<RR3_A>;
#[doc = "Request status for RR\\[3\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR3_A {
    #[doc = "0: RR\\[3\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[3\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR3_A> for bool {
    #[inline(always)]
    fn from(variant: RR3_A) -> Self {
        variant as u8 != 0
    }
}
impl RR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR3_A {
        match self.bits {
            false => RR3_A::DISABLED_OR_REQUESTED,
            true => RR3_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR3_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR3_A::ENABLED_AND_UNREQUESTED
    }
}
#[doc = "Field `RR4` reader - Request status for RR\\[4\\]
register"]
pub type RR4_R = crate::BitReader<RR4_A>;
#[doc = "Request status for RR\\[4\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR4_A {
    #[doc = "0: RR\\[4\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[4\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR4_A> for bool {
    #[inline(always)]
    fn from(variant: RR4_A) -> Self {
        variant as u8 != 0
    }
}
impl RR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR4_A {
        match self.bits {
            false => RR4_A::DISABLED_OR_REQUESTED,
            true => RR4_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR4_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR4_A::ENABLED_AND_UNREQUESTED
    }
}
#[doc = "Field `RR5` reader - Request status for RR\\[5\\]
register"]
pub type RR5_R = crate::BitReader<RR5_A>;
#[doc = "Request status for RR\\[5\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR5_A {
    #[doc = "0: RR\\[5\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[5\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR5_A> for bool {
    #[inline(always)]
    fn from(variant: RR5_A) -> Self {
        variant as u8 != 0
    }
}
impl RR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR5_A {
        match self.bits {
            false => RR5_A::DISABLED_OR_REQUESTED,
            true => RR5_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR5_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR5_A::ENABLED_AND_UNREQUESTED
    }
}
#[doc = "Field `RR6` reader - Request status for RR\\[6\\]
register"]
pub type RR6_R = crate::BitReader<RR6_A>;
#[doc = "Request status for RR\\[6\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR6_A {
    #[doc = "0: RR\\[6\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[6\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR6_A> for bool {
    #[inline(always)]
    fn from(variant: RR6_A) -> Self {
        variant as u8 != 0
    }
}
impl RR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR6_A {
        match self.bits {
            false => RR6_A::DISABLED_OR_REQUESTED,
            true => RR6_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR6_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR6_A::ENABLED_AND_UNREQUESTED
    }
}
#[doc = "Field `RR7` reader - Request status for RR\\[7\\]
register"]
pub type RR7_R = crate::BitReader<RR7_A>;
#[doc = "Request status for RR\\[7\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RR7_A {
    #[doc = "0: RR\\[7\\]
register is not enabled, or are already requesting reload"]
    DISABLED_OR_REQUESTED = 0,
    #[doc = "1: RR\\[7\\]
register is enabled, and are not yet requesting reload"]
    ENABLED_AND_UNREQUESTED = 1,
}
impl From<RR7_A> for bool {
    #[inline(always)]
    fn from(variant: RR7_A) -> Self {
        variant as u8 != 0
    }
}
impl RR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR7_A {
        match self.bits {
            false => RR7_A::DISABLED_OR_REQUESTED,
            true => RR7_A::ENABLED_AND_UNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_OR_REQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR7_A::DISABLED_OR_REQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AND_UNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR7_A::ENABLED_AND_UNREQUESTED
    }
}
impl R {
    #[doc = "Bit 0 - Request status for RR\\[0\\]
register"]
    #[inline(always)]
    pub fn rr0(&self) -> RR0_R {
        RR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request status for RR\\[1\\]
register"]
    #[inline(always)]
    pub fn rr1(&self) -> RR1_R {
        RR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Request status for RR\\[2\\]
register"]
    #[inline(always)]
    pub fn rr2(&self) -> RR2_R {
        RR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Request status for RR\\[3\\]
register"]
    #[inline(always)]
    pub fn rr3(&self) -> RR3_R {
        RR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Request status for RR\\[4\\]
register"]
    #[inline(always)]
    pub fn rr4(&self) -> RR4_R {
        RR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Request status for RR\\[5\\]
register"]
    #[inline(always)]
    pub fn rr5(&self) -> RR5_R {
        RR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Request status for RR\\[6\\]
register"]
    #[inline(always)]
    pub fn rr6(&self) -> RR6_R {
        RR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Request status for RR\\[7\\]
register"]
    #[inline(always)]
    pub fn rr7(&self) -> RR7_R {
        RR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Request status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqstatus](index.html) module"]
pub struct REQSTATUS_SPEC;
impl crate::RegisterSpec for REQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reqstatus::R](R) reader structure"]
impl crate::Readable for REQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REQSTATUS to value 0x01"]
impl crate::Resettable for REQSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
