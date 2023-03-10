#[doc = "Register `DEVICEADDRTYPE` reader"]
pub struct R(crate::R<DEVICEADDRTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEADDRTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEADDRTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEADDRTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICEADDRTYPE` reader - Device address type"]
pub type DEVICEADDRTYPE_R = crate::BitReader<DEVICEADDRTYPE_A>;
#[doc = "Device address type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEVICEADDRTYPE_A {
    #[doc = "0: Public address"]
    PUBLIC = 0,
    #[doc = "1: Random address"]
    RANDOM = 1,
}
impl From<DEVICEADDRTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DEVICEADDRTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl DEVICEADDRTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVICEADDRTYPE_A {
        match self.bits {
            false => DEVICEADDRTYPE_A::PUBLIC,
            true => DEVICEADDRTYPE_A::RANDOM,
        }
    }
    #[doc = "Checks if the value of the field is `PUBLIC`"]
    #[inline(always)]
    pub fn is_public(&self) -> bool {
        *self == DEVICEADDRTYPE_A::PUBLIC
    }
    #[doc = "Checks if the value of the field is `RANDOM`"]
    #[inline(always)]
    pub fn is_random(&self) -> bool {
        *self == DEVICEADDRTYPE_A::RANDOM
    }
}
impl R {
    #[doc = "Bit 0 - Device address type"]
    #[inline(always)]
    pub fn deviceaddrtype(&self) -> DEVICEADDRTYPE_R {
        DEVICEADDRTYPE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Device address type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddrtype](index.html) module"]
pub struct DEVICEADDRTYPE_SPEC;
impl crate::RegisterSpec for DEVICEADDRTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceaddrtype::R](R) reader structure"]
impl crate::Readable for DEVICEADDRTYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEADDRTYPE to value 0xffff_ffff"]
impl crate::Resettable for DEVICEADDRTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
