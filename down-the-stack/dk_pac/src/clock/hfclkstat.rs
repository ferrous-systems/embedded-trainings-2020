#[doc = "Register `HFCLKSTAT` reader"]
pub struct R(crate::R<HFCLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC` reader - Source of HFCLK"]
pub type SRC_R = crate::BitReader<SRC_A>;
#[doc = "Source of HFCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRC_A {
    #[doc = "0: 64 MHz internal oscillator (HFINT)"]
    RC = 0,
    #[doc = "1: 64 MHz crystal oscillator (HFXO)"]
    XTAL = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::RC,
            true => SRC_A::XTAL,
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
}
#[doc = "Field `STATE` reader - HFCLK state"]
pub type STATE_R = crate::BitReader<STATE_A>;
#[doc = "HFCLK state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STATE_A {
    #[doc = "0: HFCLK not running"]
    NOT_RUNNING = 0,
    #[doc = "1: HFCLK running"]
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
    #[doc = "Bit 0 - Source of HFCLK"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - HFCLK state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "HFCLK status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkstat](index.html) module"]
pub struct HFCLKSTAT_SPEC;
impl crate::RegisterSpec for HFCLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkstat::R](R) reader structure"]
impl crate::Readable for HFCLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKSTAT to value 0"]
impl crate::Resettable for HFCLKSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
