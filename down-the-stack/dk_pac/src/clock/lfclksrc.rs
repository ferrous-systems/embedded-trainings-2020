#[doc = "Register `LFCLKSRC` reader"]
pub struct R(crate::R<LFCLKSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCLKSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCLKSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCLKSRC` writer"]
pub struct W(crate::W<LFCLKSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCLKSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LFCLKSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCLKSRC_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SRC` writer - Clock source"]
pub type SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LFCLKSRC_SPEC, u8, SRC_A, 2, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn rc(self) -> &'a mut W {
        self.variant(SRC_A::RC)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SRC_A::XTAL)
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn synth(self) -> &'a mut W {
        self.variant(SRC_A::SYNTH)
    }
}
#[doc = "Field `BYPASS` reader - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_A {
    #[doc = "0: Disable (use with Xtal or low-swing external source)"]
    DISABLED = 0,
    #[doc = "1: Enable (use with rail-to-rail external source)"]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::ENABLED
    }
}
#[doc = "Field `BYPASS` writer - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFCLKSRC_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
}
#[doc = "Field `EXTERNAL` reader - Enable or disable external source for LFCLK"]
pub type EXTERNAL_R = crate::BitReader<EXTERNAL_A>;
#[doc = "Enable or disable external source for LFCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTERNAL_A {
    #[doc = "0: Disable external source (use with Xtal)"]
    DISABLED = 0,
    #[doc = "1: Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    ENABLED = 1,
}
impl From<EXTERNAL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTERNAL_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTERNAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTERNAL_A {
        match self.bits {
            false => EXTERNAL_A::DISABLED,
            true => EXTERNAL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTERNAL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTERNAL_A::ENABLED
    }
}
#[doc = "Field `EXTERNAL` writer - Enable or disable external source for LFCLK"]
pub type EXTERNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFCLKSRC_SPEC, EXTERNAL_A, O>;
impl<'a, const O: u8> EXTERNAL_W<'a, O> {
    #[doc = "Disable external source (use with Xtal)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTERNAL_A::DISABLED)
    }
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTERNAL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<16> {
        BYPASS_W::new(self)
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> EXTERNAL_W<17> {
        EXTERNAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock source for the LFCLK\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfclksrc](index.html) module"]
pub struct LFCLKSRC_SPEC;
impl crate::RegisterSpec for LFCLKSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfclksrc::R](R) reader structure"]
impl crate::Readable for LFCLKSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfclksrc::W](W) writer structure"]
impl crate::Writable for LFCLKSRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFCLKSRC to value 0"]
impl crate::Resettable for LFCLKSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
