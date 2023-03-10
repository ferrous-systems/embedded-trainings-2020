#[doc = "Register `PACKAGE` reader"]
pub struct R(crate::R<PACKAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKAGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PACKAGE` reader - Package option"]
pub type PACKAGE_R = crate::FieldReader<u32, PACKAGE_A>;
#[doc = "Package option\n\nValue on reset: 8192"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PACKAGE_A {
    #[doc = "8192: QFxx - 48-pin QFN"]
    QF = 8192,
    #[doc = "8193: CHxx - 7x8 WLCSP 56 balls"]
    CH = 8193,
    #[doc = "8194: CIxx - 7x8 WLCSP 56 balls"]
    CI = 8194,
    #[doc = "8197: CKxx - 7x8 WLCSP 56 balls with backside coating for light protection"]
    CK = 8197,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
impl PACKAGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PACKAGE_A> {
        match self.bits {
            8192 => Some(PACKAGE_A::QF),
            8193 => Some(PACKAGE_A::CH),
            8194 => Some(PACKAGE_A::CI),
            8197 => Some(PACKAGE_A::CK),
            4294967295 => Some(PACKAGE_A::UNSPECIFIED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `QF`"]
    #[inline(always)]
    pub fn is_qf(&self) -> bool {
        *self == PACKAGE_A::QF
    }
    #[doc = "Checks if the value of the field is `CH`"]
    #[inline(always)]
    pub fn is_ch(&self) -> bool {
        *self == PACKAGE_A::CH
    }
    #[doc = "Checks if the value of the field is `CI`"]
    #[inline(always)]
    pub fn is_ci(&self) -> bool {
        *self == PACKAGE_A::CI
    }
    #[doc = "Checks if the value of the field is `CK`"]
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == PACKAGE_A::CK
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PACKAGE_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new(self.bits)
    }
}
#[doc = "Package option\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [package](index.html) module"]
pub struct PACKAGE_SPEC;
impl crate::RegisterSpec for PACKAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [package::R](R) reader structure"]
impl crate::Readable for PACKAGE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKAGE to value 0x2000"]
impl crate::Resettable for PACKAGE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
