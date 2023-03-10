#[doc = "Register `SAMPLEPER` reader"]
pub struct R(crate::R<SAMPLEPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMPLEPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMPLEPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMPLEPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPLEPER` writer"]
pub struct W(crate::W<SAMPLEPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMPLEPER_SPEC>;
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
impl From<crate::W<SAMPLEPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMPLEPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLEPER` reader - Sample period. The SAMPLE register will be updated for every new sample"]
pub type SAMPLEPER_R = crate::FieldReader<u8, SAMPLEPER_A>;
#[doc = "Sample period. The SAMPLE register will be updated for every new sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAMPLEPER_A {
    #[doc = "0: 128 us"]
    _128US = 0,
    #[doc = "1: 256 us"]
    _256US = 1,
    #[doc = "2: 512 us"]
    _512US = 2,
    #[doc = "3: 1024 us"]
    _1024US = 3,
    #[doc = "4: 2048 us"]
    _2048US = 4,
    #[doc = "5: 4096 us"]
    _4096US = 5,
    #[doc = "6: 8192 us"]
    _8192US = 6,
    #[doc = "7: 16384 us"]
    _16384US = 7,
    #[doc = "8: 32768 us"]
    _32MS = 8,
    #[doc = "9: 65536 us"]
    _65MS = 9,
    #[doc = "10: 131072 us"]
    _131MS = 10,
}
impl From<SAMPLEPER_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLEPER_A) -> Self {
        variant as _
    }
}
impl SAMPLEPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAMPLEPER_A> {
        match self.bits {
            0 => Some(SAMPLEPER_A::_128US),
            1 => Some(SAMPLEPER_A::_256US),
            2 => Some(SAMPLEPER_A::_512US),
            3 => Some(SAMPLEPER_A::_1024US),
            4 => Some(SAMPLEPER_A::_2048US),
            5 => Some(SAMPLEPER_A::_4096US),
            6 => Some(SAMPLEPER_A::_8192US),
            7 => Some(SAMPLEPER_A::_16384US),
            8 => Some(SAMPLEPER_A::_32MS),
            9 => Some(SAMPLEPER_A::_65MS),
            10 => Some(SAMPLEPER_A::_131MS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_128US`"]
    #[inline(always)]
    pub fn is_128us(&self) -> bool {
        *self == SAMPLEPER_A::_128US
    }
    #[doc = "Checks if the value of the field is `_256US`"]
    #[inline(always)]
    pub fn is_256us(&self) -> bool {
        *self == SAMPLEPER_A::_256US
    }
    #[doc = "Checks if the value of the field is `_512US`"]
    #[inline(always)]
    pub fn is_512us(&self) -> bool {
        *self == SAMPLEPER_A::_512US
    }
    #[doc = "Checks if the value of the field is `_1024US`"]
    #[inline(always)]
    pub fn is_1024us(&self) -> bool {
        *self == SAMPLEPER_A::_1024US
    }
    #[doc = "Checks if the value of the field is `_2048US`"]
    #[inline(always)]
    pub fn is_2048us(&self) -> bool {
        *self == SAMPLEPER_A::_2048US
    }
    #[doc = "Checks if the value of the field is `_4096US`"]
    #[inline(always)]
    pub fn is_4096us(&self) -> bool {
        *self == SAMPLEPER_A::_4096US
    }
    #[doc = "Checks if the value of the field is `_8192US`"]
    #[inline(always)]
    pub fn is_8192us(&self) -> bool {
        *self == SAMPLEPER_A::_8192US
    }
    #[doc = "Checks if the value of the field is `_16384US`"]
    #[inline(always)]
    pub fn is_16384us(&self) -> bool {
        *self == SAMPLEPER_A::_16384US
    }
    #[doc = "Checks if the value of the field is `_32MS`"]
    #[inline(always)]
    pub fn is_32ms(&self) -> bool {
        *self == SAMPLEPER_A::_32MS
    }
    #[doc = "Checks if the value of the field is `_65MS`"]
    #[inline(always)]
    pub fn is_65ms(&self) -> bool {
        *self == SAMPLEPER_A::_65MS
    }
    #[doc = "Checks if the value of the field is `_131MS`"]
    #[inline(always)]
    pub fn is_131ms(&self) -> bool {
        *self == SAMPLEPER_A::_131MS
    }
}
#[doc = "Field `SAMPLEPER` writer - Sample period. The SAMPLE register will be updated for every new sample"]
pub type SAMPLEPER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAMPLEPER_SPEC, u8, SAMPLEPER_A, 4, O>;
impl<'a, const O: u8> SAMPLEPER_W<'a, O> {
    #[doc = "128 us"]
    #[inline(always)]
    pub fn _128us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_128US)
    }
    #[doc = "256 us"]
    #[inline(always)]
    pub fn _256us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_256US)
    }
    #[doc = "512 us"]
    #[inline(always)]
    pub fn _512us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_512US)
    }
    #[doc = "1024 us"]
    #[inline(always)]
    pub fn _1024us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_1024US)
    }
    #[doc = "2048 us"]
    #[inline(always)]
    pub fn _2048us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_2048US)
    }
    #[doc = "4096 us"]
    #[inline(always)]
    pub fn _4096us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_4096US)
    }
    #[doc = "8192 us"]
    #[inline(always)]
    pub fn _8192us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_8192US)
    }
    #[doc = "16384 us"]
    #[inline(always)]
    pub fn _16384us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_16384US)
    }
    #[doc = "32768 us"]
    #[inline(always)]
    pub fn _32ms(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_32MS)
    }
    #[doc = "65536 us"]
    #[inline(always)]
    pub fn _65ms(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_65MS)
    }
    #[doc = "131072 us"]
    #[inline(always)]
    pub fn _131ms(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_131MS)
    }
}
impl R {
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    pub fn sampleper(&self) -> SAMPLEPER_R {
        SAMPLEPER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    #[must_use]
    pub fn sampleper(&mut self) -> SAMPLEPER_W<0> {
        SAMPLEPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample period\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampleper](index.html) module"]
pub struct SAMPLEPER_SPEC;
impl crate::RegisterSpec for SAMPLEPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sampleper::R](R) reader structure"]
impl crate::Readable for SAMPLEPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampleper::W](W) writer structure"]
impl crate::Writable for SAMPLEPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMPLEPER to value 0"]
impl crate::Resettable for SAMPLEPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
