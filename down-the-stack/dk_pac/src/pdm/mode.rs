#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPERATION` reader - Mono or stereo operation"]
pub type OPERATION_R = crate::BitReader<OPERATION_A>;
#[doc = "Mono or stereo operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPERATION_A {
    #[doc = "0: Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    STEREO = 0,
    #[doc = "1: Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    MONO = 1,
}
impl From<OPERATION_A> for bool {
    #[inline(always)]
    fn from(variant: OPERATION_A) -> Self {
        variant as u8 != 0
    }
}
impl OPERATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPERATION_A {
        match self.bits {
            false => OPERATION_A::STEREO,
            true => OPERATION_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == OPERATION_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == OPERATION_A::MONO
    }
}
#[doc = "Field `OPERATION` writer - Mono or stereo operation"]
pub type OPERATION_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, OPERATION_A, O>;
impl<'a, const O: u8> OPERATION_W<'a, O> {
    #[doc = "Sample and store one pair (Left + Right) of 16bit samples per RAM word R=\\[31:16\\]; L=\\[15:0\\]"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(OPERATION_A::STEREO)
    }
    #[doc = "Sample and store two successive Left samples (16 bit each) per RAM word L1=\\[31:16\\]; L0=\\[15:0\\]"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(OPERATION_A::MONO)
    }
}
#[doc = "Field `EDGE` reader - Defines on which PDM_CLK edge Left (or mono) is sampled"]
pub type EDGE_R = crate::BitReader<EDGE_A>;
#[doc = "Defines on which PDM_CLK edge Left (or mono) is sampled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDGE_A {
    #[doc = "0: Left (or mono) is sampled on falling edge of PDM_CLK"]
    LEFT_FALLING = 0,
    #[doc = "1: Left (or mono) is sampled on rising edge of PDM_CLK"]
    LEFT_RISING = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::LEFT_FALLING,
            true => EDGE_A::LEFT_RISING,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT_FALLING`"]
    #[inline(always)]
    pub fn is_left_falling(&self) -> bool {
        *self == EDGE_A::LEFT_FALLING
    }
    #[doc = "Checks if the value of the field is `LEFT_RISING`"]
    #[inline(always)]
    pub fn is_left_rising(&self) -> bool {
        *self == EDGE_A::LEFT_RISING
    }
}
#[doc = "Field `EDGE` writer - Defines on which PDM_CLK edge Left (or mono) is sampled"]
pub type EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, EDGE_A, O>;
impl<'a, const O: u8> EDGE_W<'a, O> {
    #[doc = "Left (or mono) is sampled on falling edge of PDM_CLK"]
    #[inline(always)]
    pub fn left_falling(self) -> &'a mut W {
        self.variant(EDGE_A::LEFT_FALLING)
    }
    #[doc = "Left (or mono) is sampled on rising edge of PDM_CLK"]
    #[inline(always)]
    pub fn left_rising(self) -> &'a mut W {
        self.variant(EDGE_A::LEFT_RISING)
    }
}
impl R {
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mono or stereo operation"]
    #[inline(always)]
    #[must_use]
    pub fn operation(&mut self) -> OPERATION_W<0> {
        OPERATION_W::new(self)
    }
    #[doc = "Bit 1 - Defines on which PDM_CLK edge Left (or mono) is sampled"]
    #[inline(always)]
    #[must_use]
    pub fn edge(&mut self) -> EDGE_W<1> {
        EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Defines the routing of the connected PDM microphones' signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
