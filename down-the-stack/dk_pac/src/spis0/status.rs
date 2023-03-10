#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERREAD` reader - TX buffer over-read detected, and prevented"]
pub type OVERREAD_R = crate::BitReader<OVERREAD_A>;
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERREAD_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<OVERREAD_A> for bool {
    #[inline(always)]
    fn from(variant: OVERREAD_A) -> Self {
        variant as u8 != 0
    }
}
impl OVERREAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERREAD_A {
        match self.bits {
            false => OVERREAD_A::NOT_PRESENT,
            true => OVERREAD_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == OVERREAD_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == OVERREAD_A::PRESENT
    }
}
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERREAD_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERREAD_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERREAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERREAD` writer - TX buffer over-read detected, and prevented"]
pub type OVERREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, OVERREAD_AW, O>;
impl<'a, const O: u8> OVERREAD_W<'a, O> {
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERREAD_AW::CLEAR)
    }
}
#[doc = "Field `OVERFLOW` reader - RX buffer overflow detected, and prevented"]
pub type OVERFLOW_R = crate::BitReader<OVERFLOW_A>;
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERFLOW_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl OVERFLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_A {
        match self.bits {
            false => OVERFLOW_A::NOT_PRESENT,
            true => OVERFLOW_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == OVERFLOW_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == OVERFLOW_A::PRESENT
    }
}
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERFLOW_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERFLOW` writer - RX buffer overflow detected, and prevented"]
pub type OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, OVERFLOW_AW, O>;
impl<'a, const O: u8> OVERFLOW_W<'a, O> {
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OVERREAD_R {
        OVERREAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    #[must_use]
    pub fn overread(&mut self) -> OVERREAD_W<0> {
        OVERREAD_W::new(self)
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<1> {
        OVERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status from last transaction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
