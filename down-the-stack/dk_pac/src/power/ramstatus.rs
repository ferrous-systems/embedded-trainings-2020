#[doc = "Register `RAMSTATUS` reader"]
pub struct R(crate::R<RAMSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAMBLOCK0` reader - RAM block 0 is on or off/powering up"]
pub type RAMBLOCK0_R = crate::BitReader<RAMBLOCK0_A>;
#[doc = "RAM block 0 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMBLOCK0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<RAMBLOCK0_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBLOCK0_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMBLOCK0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBLOCK0_A {
        match self.bits {
            false => RAMBLOCK0_A::OFF,
            true => RAMBLOCK0_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK0_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK0_A::ON
    }
}
#[doc = "Field `RAMBLOCK1` reader - RAM block 1 is on or off/powering up"]
pub type RAMBLOCK1_R = crate::BitReader<RAMBLOCK1_A>;
#[doc = "RAM block 1 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMBLOCK1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<RAMBLOCK1_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBLOCK1_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMBLOCK1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBLOCK1_A {
        match self.bits {
            false => RAMBLOCK1_A::OFF,
            true => RAMBLOCK1_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK1_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK1_A::ON
    }
}
#[doc = "Field `RAMBLOCK2` reader - RAM block 2 is on or off/powering up"]
pub type RAMBLOCK2_R = crate::BitReader<RAMBLOCK2_A>;
#[doc = "RAM block 2 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMBLOCK2_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<RAMBLOCK2_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBLOCK2_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMBLOCK2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBLOCK2_A {
        match self.bits {
            false => RAMBLOCK2_A::OFF,
            true => RAMBLOCK2_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK2_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK2_A::ON
    }
}
#[doc = "Field `RAMBLOCK3` reader - RAM block 3 is on or off/powering up"]
pub type RAMBLOCK3_R = crate::BitReader<RAMBLOCK3_A>;
#[doc = "RAM block 3 is on or off/powering up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAMBLOCK3_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<RAMBLOCK3_A> for bool {
    #[inline(always)]
    fn from(variant: RAMBLOCK3_A) -> Self {
        variant as u8 != 0
    }
}
impl RAMBLOCK3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAMBLOCK3_A {
        match self.bits {
            false => RAMBLOCK3_A::OFF,
            true => RAMBLOCK3_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK3_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK3_A::ON
    }
}
impl R {
    #[doc = "Bit 0 - RAM block 0 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock0(&self) -> RAMBLOCK0_R {
        RAMBLOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM block 1 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock1(&self) -> RAMBLOCK1_R {
        RAMBLOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM block 2 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock2(&self) -> RAMBLOCK2_R {
        RAMBLOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM block 3 is on or off/powering up"]
    #[inline(always)]
    pub fn ramblock3(&self) -> RAMBLOCK3_R {
        RAMBLOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Deprecated register - RAM status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramstatus](index.html) module"]
pub struct RAMSTATUS_SPEC;
impl crate::RegisterSpec for RAMSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramstatus::R](R) reader structure"]
impl crate::Readable for RAMSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAMSTATUS to value 0"]
impl crate::Resettable for RAMSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
