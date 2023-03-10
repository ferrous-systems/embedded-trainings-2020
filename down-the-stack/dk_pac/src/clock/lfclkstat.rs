#[doc = "Register `LFCLKSTAT` reader"]
pub struct R(crate::R<LFCLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC` reader - Source of LFCLK"]
pub type SRC_R = crate::FieldReader<u8, SRC_A>;
#[doc = "Source of LFCLK\n\nValue on reset: 0"]
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
#[doc = "Field `STATE` reader - LFCLK state"]
pub type STATE_R = crate::BitReader<STATE_A>;
#[doc = "LFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATE_A {
    #[doc = "0: LFCLK not running"]
    NOT_RUNNING = 0,
    #[doc = "1: LFCLK running"]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::NOT_RUNNING,
            true => STATE_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == STATE_A::NOT_RUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == STATE_A::RUNNING
    }
}
impl R {
    #[doc = "Bits 0:1 - Source of LFCLK"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - LFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "LFCLK status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclkstat](index.html) module"]
pub struct LFCLKSTAT_SPEC;
impl crate::RegisterSpec for LFCLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclkstat::R](R) reader structure"]
impl crate::Readable for LFCLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LFCLKSTAT to value 0"]
impl crate::Resettable for LFCLKSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
