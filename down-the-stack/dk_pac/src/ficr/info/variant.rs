#[doc = "Register `VARIANT` reader"]
pub struct R(crate::R<VARIANT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VARIANT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VARIANT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VARIANT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VARIANT` reader - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
pub type VARIANT_R = crate::FieldReader<u32, VARIANT_A>;
#[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII\n\nValue on reset: 1094795586"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA = 1094795585,
    #[doc = "1094795587: AAAC"]
    AAAC = 1094795587,
    #[doc = "1094795841: AABA"]
    AABA = 1094795841,
    #[doc = "1094795842: AABB"]
    AABB = 1094795842,
    #[doc = "1094795824: AAB0"]
    AAB0 = 1094795824,
    #[doc = "1094861360: ABB0"]
    ABB0 = 1094861360,
    #[doc = "1094796592: AAE0"]
    AAE0 = 1094796592,
    #[doc = "1094862128: ABE0"]
    ABE0 = 1094862128,
    #[doc = "1094797122: AAGB"]
    AAGB = 1094797122,
    #[doc = "1094862658: ABGB"]
    ABGB = 1094862658,
    #[doc = "1094797104: AAG0"]
    AAG0 = 1094797104,
    #[doc = "1094862640: ABG0"]
    ABG0 = 1094862640,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
impl VARIANT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VARIANT_A> {
        match self.bits {
            1094795585 => Some(VARIANT_A::AAAA),
            1094795587 => Some(VARIANT_A::AAAC),
            1094795841 => Some(VARIANT_A::AABA),
            1094795842 => Some(VARIANT_A::AABB),
            1094795824 => Some(VARIANT_A::AAB0),
            1094861360 => Some(VARIANT_A::ABB0),
            1094796592 => Some(VARIANT_A::AAE0),
            1094862128 => Some(VARIANT_A::ABE0),
            1094797122 => Some(VARIANT_A::AAGB),
            1094862658 => Some(VARIANT_A::ABGB),
            1094797104 => Some(VARIANT_A::AAG0),
            1094862640 => Some(VARIANT_A::ABG0),
            4294967295 => Some(VARIANT_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `AAAC`"]
    #[inline(always)]
    pub fn is_aaac(&self) -> bool {
        *self == VARIANT_A::AAAC
    }
    #[doc = "Checks if the value of the field is `AABA`"]
    #[inline(always)]
    pub fn is_aaba(&self) -> bool {
        *self == VARIANT_A::AABA
    }
    #[doc = "Checks if the value of the field is `AABB`"]
    #[inline(always)]
    pub fn is_aabb(&self) -> bool {
        *self == VARIANT_A::AABB
    }
    #[doc = "Checks if the value of the field is `AAB0`"]
    #[inline(always)]
    pub fn is_aab0(&self) -> bool {
        *self == VARIANT_A::AAB0
    }
    #[doc = "Checks if the value of the field is `ABB0`"]
    #[inline(always)]
    pub fn is_abb0(&self) -> bool {
        *self == VARIANT_A::ABB0
    }
    #[doc = "Checks if the value of the field is `AAE0`"]
    #[inline(always)]
    pub fn is_aae0(&self) -> bool {
        *self == VARIANT_A::AAE0
    }
    #[doc = "Checks if the value of the field is `ABE0`"]
    #[inline(always)]
    pub fn is_abe0(&self) -> bool {
        *self == VARIANT_A::ABE0
    }
    #[doc = "Checks if the value of the field is `AAGB`"]
    #[inline(always)]
    pub fn is_aagb(&self) -> bool {
        *self == VARIANT_A::AAGB
    }
    #[doc = "Checks if the value of the field is `ABGB`"]
    #[inline(always)]
    pub fn is_abgb(&self) -> bool {
        *self == VARIANT_A::ABGB
    }
    #[doc = "Checks if the value of the field is `AAG0`"]
    #[inline(always)]
    pub fn is_aag0(&self) -> bool {
        *self == VARIANT_A::AAG0
    }
    #[doc = "Checks if the value of the field is `ABG0`"]
    #[inline(always)]
    pub fn is_abg0(&self) -> bool {
        *self == VARIANT_A::ABG0
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANT_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(self.bits)
    }
}
#[doc = "Part Variant, Hardware version and Production configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [variant](index.html) module"]
pub struct VARIANT_SPEC;
impl crate::RegisterSpec for VARIANT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [variant::R](R) reader structure"]
impl crate::Readable for VARIANT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VARIANT to value 0x4141_4142"]
impl crate::Resettable for VARIANT_SPEC {
    const RESET_VALUE: Self::Ux = 0x4141_4142;
}
