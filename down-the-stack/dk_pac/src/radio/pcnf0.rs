#[doc = "Register `PCNF0` reader"]
pub struct R(crate::R<PCNF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCNF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCNF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCNF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCNF0` writer"]
pub struct W(crate::W<PCNF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCNF0_SPEC>;
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
impl From<crate::W<PCNF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCNF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFLEN` reader - Length on air of LENGTH field in number of bits."]
pub type LFLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LFLEN` writer - Length on air of LENGTH field in number of bits."]
pub type LFLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNF0_SPEC, u8, u8, 4, O>;
#[doc = "Field `S0LEN` reader - Length on air of S0 field in number of bytes."]
pub type S0LEN_R = crate::BitReader<bool>;
#[doc = "Field `S0LEN` writer - Length on air of S0 field in number of bytes."]
pub type S0LEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNF0_SPEC, bool, O>;
#[doc = "Field `S1LEN` reader - Length on air of S1 field in number of bits."]
pub type S1LEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `S1LEN` writer - Length on air of S1 field in number of bits."]
pub type S1LEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCNF0_SPEC, u8, u8, 4, O>;
#[doc = "Field `S1INCL` reader - Include or exclude S1 field in RAM"]
pub type S1INCL_R = crate::BitReader<S1INCL_A>;
#[doc = "Include or exclude S1 field in RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S1INCL_A {
    #[doc = "0: Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC = 0,
    #[doc = "1: Always include S1 field in RAM independent of S1LEN"]
    INCLUDE = 1,
}
impl From<S1INCL_A> for bool {
    #[inline(always)]
    fn from(variant: S1INCL_A) -> Self {
        variant as u8 != 0
    }
}
impl S1INCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1INCL_A {
        match self.bits {
            false => S1INCL_A::AUTOMATIC,
            true => S1INCL_A::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == S1INCL_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline(always)]
    pub fn is_include(&self) -> bool {
        *self == S1INCL_A::INCLUDE
    }
}
#[doc = "Field `S1INCL` writer - Include or exclude S1 field in RAM"]
pub type S1INCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNF0_SPEC, S1INCL_A, O>;
impl<'a, const O: u8> S1INCL_W<'a, O> {
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(S1INCL_A::AUTOMATIC)
    }
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    #[inline(always)]
    pub fn include(self) -> &'a mut W {
        self.variant(S1INCL_A::INCLUDE)
    }
}
#[doc = "Field `PLEN` reader - Length of preamble on air. Decision point: TASKS_START task"]
pub type PLEN_R = crate::BitReader<PLEN_A>;
#[doc = "Length of preamble on air. Decision point: TASKS_START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLEN_A {
    #[doc = "0: 8-bit preamble"]
    _8BIT = 0,
    #[doc = "1: 16-bit preamble"]
    _16BIT = 1,
}
impl From<PLEN_A> for bool {
    #[inline(always)]
    fn from(variant: PLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLEN_A {
        match self.bits {
            false => PLEN_A::_8BIT,
            true => PLEN_A::_16BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == PLEN_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == PLEN_A::_16BIT
    }
}
#[doc = "Field `PLEN` writer - Length of preamble on air. Decision point: TASKS_START task"]
pub type PLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCNF0_SPEC, PLEN_A, O>;
impl<'a, const O: u8> PLEN_W<'a, O> {
    #[doc = "8-bit preamble"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(PLEN_A::_8BIT)
    }
    #[doc = "16-bit preamble"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(PLEN_A::_16BIT)
    }
}
impl R {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    pub fn lflen(&self) -> LFLEN_R {
        LFLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline(always)]
    pub fn s0len(&self) -> S0LEN_R {
        S0LEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline(always)]
    pub fn s1len(&self) -> S1LEN_R {
        S1LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    pub fn s1incl(&self) -> S1INCL_R {
        S1INCL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    pub fn plen(&self) -> PLEN_R {
        PLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline(always)]
    #[must_use]
    pub fn lflen(&mut self) -> LFLEN_W<0> {
        LFLEN_W::new(self)
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline(always)]
    #[must_use]
    pub fn s0len(&mut self) -> S0LEN_W<8> {
        S0LEN_W::new(self)
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline(always)]
    #[must_use]
    pub fn s1len(&mut self) -> S1LEN_W<16> {
        S1LEN_W::new(self)
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline(always)]
    #[must_use]
    pub fn s1incl(&mut self) -> S1INCL_W<20> {
        S1INCL_W::new(self)
    }
    #[doc = "Bit 24 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline(always)]
    #[must_use]
    pub fn plen(&mut self) -> PLEN_W<24> {
        PLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Packet configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcnf0](index.html) module"]
pub struct PCNF0_SPEC;
impl crate::RegisterSpec for PCNF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcnf0::R](R) reader structure"]
impl crate::Readable for PCNF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcnf0::W](W) writer structure"]
impl crate::Writable for PCNF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCNF0 to value 0"]
impl crate::Resettable for PCNF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
