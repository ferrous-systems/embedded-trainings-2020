#[doc = "Register `FREQUENCY` reader"]
pub struct R(crate::R<FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQUENCY` writer"]
pub struct W(crate::W<FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQUENCY_SPEC>;
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
impl From<crate::W<FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQUENCY` reader - Radio channel frequency"]
pub type FREQUENCY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FREQUENCY` writer - Radio channel frequency"]
pub type FREQUENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FREQUENCY_SPEC, u8, u8, 7, O>;
#[doc = "Field `MAP` reader - Channel map selection."]
pub type MAP_R = crate::BitReader<MAP_A>;
#[doc = "Channel map selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAP_A {
    #[doc = "0: Channel map between 2400 MHZ .. 2500 MHz"]
    DEFAULT = 0,
    #[doc = "1: Channel map between 2360 MHZ .. 2460 MHz"]
    LOW = 1,
}
impl From<MAP_A> for bool {
    #[inline(always)]
    fn from(variant: MAP_A) -> Self {
        variant as u8 != 0
    }
}
impl MAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAP_A {
        match self.bits {
            false => MAP_A::DEFAULT,
            true => MAP_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == MAP_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == MAP_A::LOW
    }
}
#[doc = "Field `MAP` writer - Channel map selection."]
pub type MAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FREQUENCY_SPEC, MAP_A, O>;
impl<'a, const O: u8> MAP_W<'a, O> {
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(MAP_A::DEFAULT)
    }
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(MAP_A::LOW)
    }
}
impl R {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    #[must_use]
    pub fn frequency(&mut self) -> FREQUENCY_W<0> {
        FREQUENCY_W::new(self)
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    #[must_use]
    pub fn map(&mut self) -> MAP_W<8> {
        MAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](index.html) module"]
pub struct FREQUENCY_SPEC;
impl crate::RegisterSpec for FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frequency::R](R) reader structure"]
impl crate::Readable for FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frequency::W](W) writer structure"]
impl crate::Writable for FREQUENCY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x02"]
impl crate::Resettable for FREQUENCY_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
