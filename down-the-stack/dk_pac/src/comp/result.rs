#[doc = "Register `RESULT` reader"]
pub struct R(crate::R<RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT` reader - Result of last compare. Decision point SAMPLE task."]
pub type RESULT_R = crate::BitReader<RESULT_A>;
#[doc = "Result of last compare. Decision point SAMPLE task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESULT_A {
    #[doc = "0: Input voltage is below the threshold (VIN+ &lt; VIN-)"]
    BELOW = 0,
    #[doc = "1: Input voltage is above the threshold (VIN+ &gt; VIN-)"]
    ABOVE = 1,
}
impl From<RESULT_A> for bool {
    #[inline(always)]
    fn from(variant: RESULT_A) -> Self {
        variant as u8 != 0
    }
}
impl RESULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESULT_A {
        match self.bits {
            false => RESULT_A::BELOW,
            true => RESULT_A::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RESULT_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RESULT_A::ABOVE
    }
}
impl R {
    #[doc = "Bit 0 - Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Compare result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](index.html) module"]
pub struct RESULT_SPEC;
impl crate::RegisterSpec for RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result::R](R) reader structure"]
impl crate::Readable for RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for RESULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
