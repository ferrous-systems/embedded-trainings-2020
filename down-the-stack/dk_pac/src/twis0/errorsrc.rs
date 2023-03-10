#[doc = "Register `ERRORSRC` reader"]
pub struct R(crate::R<ERRORSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORSRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRORSRC` writer"]
pub struct W(crate::W<ERRORSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRORSRC_SPEC>;
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
impl From<crate::W<ERRORSRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRORSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVERFLOW` reader - RX buffer overflow detected, and prevented"]
pub type OVERFLOW_R = crate::BitReader<OVERFLOW_A>;
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERFLOW_A {
    #[doc = "0: Error did not occur"]
    NOT_DETECTED = 0,
    #[doc = "1: Error occurred"]
    DETECTED = 1,
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
            false => OVERFLOW_A::NOT_DETECTED,
            true => OVERFLOW_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OVERFLOW_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OVERFLOW_A::DETECTED
    }
}
#[doc = "Field `OVERFLOW` writer - RX buffer overflow detected, and prevented"]
pub type OVERFLOW_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, OVERFLOW_A, O>;
impl<'a, const O: u8> OVERFLOW_W<'a, O> {
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERFLOW_A::NOT_DETECTED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERFLOW_A::DETECTED)
    }
}
#[doc = "Field `DNACK` reader - NACK sent after receiving a data byte"]
pub type DNACK_R = crate::BitReader<DNACK_A>;
#[doc = "NACK sent after receiving a data byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNACK_A {
    #[doc = "0: Error did not occur"]
    NOT_RECEIVED = 0,
    #[doc = "1: Error occurred"]
    RECEIVED = 1,
}
impl From<DNACK_A> for bool {
    #[inline(always)]
    fn from(variant: DNACK_A) -> Self {
        variant as u8 != 0
    }
}
impl DNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNACK_A {
        match self.bits {
            false => DNACK_A::NOT_RECEIVED,
            true => DNACK_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == DNACK_A::NOT_RECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == DNACK_A::RECEIVED
    }
}
#[doc = "Field `DNACK` writer - NACK sent after receiving a data byte"]
pub type DNACK_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, DNACK_A, O>;
impl<'a, const O: u8> DNACK_W<'a, O> {
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut W {
        self.variant(DNACK_A::NOT_RECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut W {
        self.variant(DNACK_A::RECEIVED)
    }
}
#[doc = "Field `OVERREAD` reader - TX buffer over-read detected, and prevented"]
pub type OVERREAD_R = crate::BitReader<OVERREAD_A>;
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERREAD_A {
    #[doc = "0: Error did not occur"]
    NOT_DETECTED = 0,
    #[doc = "1: Error occurred"]
    DETECTED = 1,
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
            false => OVERREAD_A::NOT_DETECTED,
            true => OVERREAD_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OVERREAD_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OVERREAD_A::DETECTED
    }
}
#[doc = "Field `OVERREAD` writer - TX buffer over-read detected, and prevented"]
pub type OVERREAD_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, OVERREAD_A, O>;
impl<'a, const O: u8> OVERREAD_W<'a, O> {
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERREAD_A::NOT_DETECTED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERREAD_A::DETECTED)
    }
}
impl R {
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OVERREAD_R {
        OVERREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OVERFLOW_W<0> {
        OVERFLOW_W::new(self)
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    #[must_use]
    pub fn dnack(&mut self) -> DNACK_W<2> {
        DNACK_W::new(self)
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    #[must_use]
    pub fn overread(&mut self) -> OVERREAD_W<3> {
        OVERREAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error source\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorsrc](index.html) module"]
pub struct ERRORSRC_SPEC;
impl crate::RegisterSpec for ERRORSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorsrc::R](R) reader structure"]
impl crate::Readable for ERRORSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errorsrc::W](W) writer structure"]
impl crate::Writable for ERRORSRC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0d;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ERRORSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
