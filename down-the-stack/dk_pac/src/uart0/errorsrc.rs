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
#[doc = "Field `OVERRUN` reader - Overrun error"]
pub type OVERRUN_R = crate::BitReader<OVERRUN_A>;
#[doc = "Overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERRUN_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => OVERRUN_A::NOT_PRESENT,
            true => OVERRUN_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == OVERRUN_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == OVERRUN_A::PRESENT
    }
}
#[doc = "Field `OVERRUN` writer - Overrun error"]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, OVERRUN_A, O>;
impl<'a, const O: u8> OVERRUN_W<'a, O> {
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(OVERRUN_A::NOT_PRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(OVERRUN_A::PRESENT)
    }
}
#[doc = "Field `PARITY` reader - Parity error"]
pub type PARITY_R = crate::BitReader<PARITY_A>;
#[doc = "Parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITY_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<PARITY_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_A {
        match self.bits {
            false => PARITY_A::NOT_PRESENT,
            true => PARITY_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == PARITY_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PARITY_A::PRESENT
    }
}
#[doc = "Field `PARITY` writer - Parity error"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, PARITY_A, O>;
impl<'a, const O: u8> PARITY_W<'a, O> {
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(PARITY_A::NOT_PRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(PARITY_A::PRESENT)
    }
}
#[doc = "Field `FRAMING` reader - Framing error occurred"]
pub type FRAMING_R = crate::BitReader<FRAMING_A>;
#[doc = "Framing error occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAMING_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<FRAMING_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMING_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAMING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMING_A {
        match self.bits {
            false => FRAMING_A::NOT_PRESENT,
            true => FRAMING_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == FRAMING_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == FRAMING_A::PRESENT
    }
}
#[doc = "Field `FRAMING` writer - Framing error occurred"]
pub type FRAMING_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, FRAMING_A, O>;
impl<'a, const O: u8> FRAMING_W<'a, O> {
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(FRAMING_A::NOT_PRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(FRAMING_A::PRESENT)
    }
}
#[doc = "Field `BREAK` reader - Break condition"]
pub type BREAK_R = crate::BitReader<BREAK_A>;
#[doc = "Break condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREAK_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<BREAK_A> for bool {
    #[inline(always)]
    fn from(variant: BREAK_A) -> Self {
        variant as u8 != 0
    }
}
impl BREAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREAK_A {
        match self.bits {
            false => BREAK_A::NOT_PRESENT,
            true => BREAK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == BREAK_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == BREAK_A::PRESENT
    }
}
#[doc = "Field `BREAK` writer - Break condition"]
pub type BREAK_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, BREAK_A, O>;
impl<'a, const O: u8> BREAK_W<'a, O> {
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(BREAK_A::NOT_PRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(BREAK_A::PRESENT)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline(always)]
    pub fn framing(&self) -> FRAMING_R {
        FRAMING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<0> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<1> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline(always)]
    #[must_use]
    pub fn framing(&mut self) -> FRAMING_W<2> {
        FRAMING_W::new(self)
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline(always)]
    #[must_use]
    pub fn break_(&mut self) -> BREAK_W<3> {
        BREAK_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ERRORSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
