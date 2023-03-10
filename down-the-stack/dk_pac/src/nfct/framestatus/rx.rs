#[doc = "Register `RX` reader"]
pub struct R(crate::R<RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX` writer"]
pub struct W(crate::W<RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_SPEC>;
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
impl From<crate::W<RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCERROR` reader - No valid End of Frame detected"]
pub type CRCERROR_R = crate::BitReader<CRCERROR_A>;
#[doc = "No valid End of Frame detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERROR_A {
    #[doc = "0: Valid CRC detected"]
    CRCCORRECT = 0,
    #[doc = "1: CRC received does not match local check"]
    CRCERROR = 1,
}
impl From<CRCERROR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERROR_A {
        match self.bits {
            false => CRCERROR_A::CRCCORRECT,
            true => CRCERROR_A::CRCERROR,
        }
    }
    #[doc = "Checks if the value of the field is `CRCCORRECT`"]
    #[inline(always)]
    pub fn is_crccorrect(&self) -> bool {
        *self == CRCERROR_A::CRCCORRECT
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline(always)]
    pub fn is_crcerror(&self) -> bool {
        *self == CRCERROR_A::CRCERROR
    }
}
#[doc = "Field `CRCERROR` writer - No valid End of Frame detected"]
pub type CRCERROR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, RX_SPEC, CRCERROR_A, O>;
impl<'a, const O: u8> CRCERROR_W<'a, O> {
    #[doc = "Valid CRC detected"]
    #[inline(always)]
    pub fn crccorrect(self) -> &'a mut W {
        self.variant(CRCERROR_A::CRCCORRECT)
    }
    #[doc = "CRC received does not match local check"]
    #[inline(always)]
    pub fn crcerror(self) -> &'a mut W {
        self.variant(CRCERROR_A::CRCERROR)
    }
}
#[doc = "Field `PARITYSTATUS` reader - Parity status of received frame"]
pub type PARITYSTATUS_R = crate::BitReader<PARITYSTATUS_A>;
#[doc = "Parity status of received frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITYSTATUS_A {
    #[doc = "0: Frame received with parity OK"]
    PARITY_OK = 0,
    #[doc = "1: Frame received with parity error"]
    PARITY_ERROR = 1,
}
impl From<PARITYSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITYSTATUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYSTATUS_A {
        match self.bits {
            false => PARITYSTATUS_A::PARITY_OK,
            true => PARITYSTATUS_A::PARITY_ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `PARITY_OK`"]
    #[inline(always)]
    pub fn is_parity_ok(&self) -> bool {
        *self == PARITYSTATUS_A::PARITY_OK
    }
    #[doc = "Checks if the value of the field is `PARITY_ERROR`"]
    #[inline(always)]
    pub fn is_parity_error(&self) -> bool {
        *self == PARITYSTATUS_A::PARITY_ERROR
    }
}
#[doc = "Field `PARITYSTATUS` writer - Parity status of received frame"]
pub type PARITYSTATUS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, RX_SPEC, PARITYSTATUS_A, O>;
impl<'a, const O: u8> PARITYSTATUS_W<'a, O> {
    #[doc = "Frame received with parity OK"]
    #[inline(always)]
    pub fn parity_ok(self) -> &'a mut W {
        self.variant(PARITYSTATUS_A::PARITY_OK)
    }
    #[doc = "Frame received with parity error"]
    #[inline(always)]
    pub fn parity_error(self) -> &'a mut W {
        self.variant(PARITYSTATUS_A::PARITY_ERROR)
    }
}
#[doc = "Field `OVERRUN` reader - Overrun detected"]
pub type OVERRUN_R = crate::BitReader<OVERRUN_A>;
#[doc = "Overrun detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERRUN_A {
    #[doc = "0: No overrun detected"]
    NO_OVERRUN = 0,
    #[doc = "1: Overrun error"]
    OVERRUN = 1,
}
impl From<OVERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVERRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERRUN_A {
        match self.bits {
            false => OVERRUN_A::NO_OVERRUN,
            true => OVERRUN_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVERRUN_A::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVERRUN_A::OVERRUN
    }
}
#[doc = "Field `OVERRUN` writer - Overrun detected"]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, RX_SPEC, OVERRUN_A, O>;
impl<'a, const O: u8> OVERRUN_W<'a, O> {
    #[doc = "No overrun detected"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVERRUN_A::NO_OVERRUN)
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVERRUN_A::OVERRUN)
    }
}
impl R {
    #[doc = "Bit 0 - No valid End of Frame detected"]
    #[inline(always)]
    pub fn crcerror(&self) -> CRCERROR_R {
        CRCERROR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline(always)]
    pub fn paritystatus(&self) -> PARITYSTATUS_R {
        PARITYSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No valid End of Frame detected"]
    #[inline(always)]
    #[must_use]
    pub fn crcerror(&mut self) -> CRCERROR_W<0> {
        CRCERROR_W::new(self)
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline(always)]
    #[must_use]
    pub fn paritystatus(&mut self) -> PARITYSTATUS_W<2> {
        PARITYSTATUS_W::new(self)
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<3> {
        OVERRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result of last incoming frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx](index.html) module"]
pub struct RX_SPEC;
impl crate::RegisterSpec for RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx::R](R) reader structure"]
impl crate::Readable for RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx::W](W) writer structure"]
impl crate::Writable for RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0d;
}
#[doc = "`reset()` method sets RX to value 0"]
impl crate::Resettable for RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
