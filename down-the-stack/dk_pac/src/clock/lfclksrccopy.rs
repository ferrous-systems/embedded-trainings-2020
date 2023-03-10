#[doc = "Register `LFCLKSRCCOPY` reader"]
pub struct R(crate::R<LFCLKSRCCOPY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSRCCOPY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSRCCOPY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSRCCOPY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC` reader - Clock source"]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: 32.768 kHz RC oscillator"]
    RC = 0,
    #[doc = "1: 32.768 kHz crystal oscillator"]
    XTAL = 1,
    #[doc = "2: 32.768 kHz synthesized from HFCLK"]
    SYNTH = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::RC),
            1 => Some(SRC_A::XTAL),
            2 => Some(SRC_A::SYNTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        *self == SRC_A::SYNTH
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
}
#[doc = "Copy of LFCLKSRC register, set when LFCLKSTART task was triggered\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrccopy](index.html) module"]
pub struct LFCLKSRCCOPY_SPEC;
impl crate::RegisterSpec for LFCLKSRCCOPY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclksrccopy::R](R) reader structure"]
impl crate::Readable for LFCLKSRCCOPY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LFCLKSRCCOPY to value 0"]
impl crate::Resettable for LFCLKSRCCOPY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
