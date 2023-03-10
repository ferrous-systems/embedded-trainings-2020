#[doc = "Register `DECODER` reader"]
pub struct R(crate::R<DECODER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECODER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECODER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECODER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECODER` writer"]
pub struct W(crate::W<DECODER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECODER_SPEC>;
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
impl From<crate::W<DECODER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECODER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - How a sequence is read from RAM and spread to the compare register"]
pub type LOAD_R = crate::FieldReader<u8, LOAD_A>;
#[doc = "How a sequence is read from RAM and spread to the compare register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOAD_A {
    #[doc = "0: 1st half word (16-bit) used in all PWM channels 0..3"]
    COMMON = 0,
    #[doc = "1: 1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    GROUPED = 1,
    #[doc = "2: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    INDIVIDUAL = 2,
    #[doc = "3: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WAVE_FORM = 3,
}
impl From<LOAD_A> for u8 {
    #[inline(always)]
    fn from(variant: LOAD_A) -> Self {
        variant as _
    }
}
impl LOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOAD_A {
        match self.bits {
            0 => LOAD_A::COMMON,
            1 => LOAD_A::GROUPED,
            2 => LOAD_A::INDIVIDUAL,
            3 => LOAD_A::WAVE_FORM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COMMON`"]
    #[inline(always)]
    pub fn is_common(&self) -> bool {
        *self == LOAD_A::COMMON
    }
    #[doc = "Checks if the value of the field is `GROUPED`"]
    #[inline(always)]
    pub fn is_grouped(&self) -> bool {
        *self == LOAD_A::GROUPED
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL`"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        *self == LOAD_A::INDIVIDUAL
    }
    #[doc = "Checks if the value of the field is `WAVE_FORM`"]
    #[inline(always)]
    pub fn is_wave_form(&self) -> bool {
        *self == LOAD_A::WAVE_FORM
    }
}
#[doc = "Field `LOAD` writer - How a sequence is read from RAM and spread to the compare register"]
pub type LOAD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DECODER_SPEC, u8, LOAD_A, 2, O>;
impl<'a, const O: u8> LOAD_W<'a, O> {
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    #[inline(always)]
    pub fn common(self) -> &'a mut W {
        self.variant(LOAD_A::COMMON)
    }
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    #[inline(always)]
    pub fn grouped(self) -> &'a mut W {
        self.variant(LOAD_A::GROUPED)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    #[inline(always)]
    pub fn individual(self) -> &'a mut W {
        self.variant(LOAD_A::INDIVIDUAL)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    #[inline(always)]
    pub fn wave_form(self) -> &'a mut W {
        self.variant(LOAD_A::WAVE_FORM)
    }
}
#[doc = "Field `MODE` reader - Selects source for advancing the active sequence"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Selects source for advancing the active sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    REFRESH_COUNT = 0,
    #[doc = "1: NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NEXT_STEP = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::REFRESH_COUNT,
            true => MODE_A::NEXT_STEP,
        }
    }
    #[doc = "Checks if the value of the field is `REFRESH_COUNT`"]
    #[inline(always)]
    pub fn is_refresh_count(&self) -> bool {
        *self == MODE_A::REFRESH_COUNT
    }
    #[doc = "Checks if the value of the field is `NEXT_STEP`"]
    #[inline(always)]
    pub fn is_next_step(&self) -> bool {
        *self == MODE_A::NEXT_STEP
    }
}
#[doc = "Field `MODE` writer - Selects source for advancing the active sequence"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DECODER_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    #[inline(always)]
    pub fn refresh_count(self) -> &'a mut W {
        self.variant(MODE_A::REFRESH_COUNT)
    }
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    #[inline(always)]
    pub fn next_step(self) -> &'a mut W {
        self.variant(MODE_A::NEXT_STEP)
    }
}
impl R {
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration of the decoder\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decoder](index.html) module"]
pub struct DECODER_SPEC;
impl crate::RegisterSpec for DECODER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decoder::R](R) reader structure"]
impl crate::Readable for DECODER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decoder::W](W) writer structure"]
impl crate::Writable for DECODER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DECODER to value 0"]
impl crate::Resettable for DECODER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
