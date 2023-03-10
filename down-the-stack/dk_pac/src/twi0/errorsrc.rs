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
    #[doc = "0: Read: no overrun occured"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: overrun occured"]
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
#[doc = "Overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERRUN_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERRUN_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVERRUN` writer - Overrun error"]
pub type OVERRUN_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, OVERRUN_AW, O>;
impl<'a, const O: u8> OVERRUN_W<'a, O> {
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERRUN_AW::CLEAR)
    }
}
#[doc = "Field `ANACK` reader - NACK received after sending the address (write '1' to clear)"]
pub type ANACK_R = crate::BitReader<ANACK_A>;
#[doc = "NACK received after sending the address (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANACK_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<ANACK_A> for bool {
    #[inline(always)]
    fn from(variant: ANACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ANACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACK_A {
        match self.bits {
            false => ANACK_A::NOT_PRESENT,
            true => ANACK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == ANACK_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == ANACK_A::PRESENT
    }
}
#[doc = "NACK received after sending the address (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANACK_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<ANACK_AW> for bool {
    #[inline(always)]
    fn from(variant: ANACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` writer - NACK received after sending the address (write '1' to clear)"]
pub type ANACK_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, ANACK_AW, O>;
impl<'a, const O: u8> ANACK_W<'a, O> {
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ANACK_AW::CLEAR)
    }
}
#[doc = "Field `DNACK` reader - NACK received after sending a data byte (write '1' to clear)"]
pub type DNACK_R = crate::BitReader<DNACK_A>;
#[doc = "NACK received after sending a data byte (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNACK_A {
    #[doc = "0: Read: error not present"]
    NOT_PRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => DNACK_A::NOT_PRESENT,
            true => DNACK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == DNACK_A::NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == DNACK_A::PRESENT
    }
}
#[doc = "NACK received after sending a data byte (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DNACK_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<DNACK_AW> for bool {
    #[inline(always)]
    fn from(variant: DNACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DNACK` writer - NACK received after sending a data byte (write '1' to clear)"]
pub type DNACK_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ERRORSRC_SPEC, DNACK_AW, O>;
impl<'a, const O: u8> DNACK_W<'a, O> {
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DNACK_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    #[must_use]
    pub fn overrun(&mut self) -> OVERRUN_W<0> {
        OVERRUN_W::new(self)
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn anack(&mut self) -> ANACK_W<1> {
        ANACK_W::new(self)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn dnack(&mut self) -> DNACK_W<2> {
        DNACK_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x07;
}
#[doc = "`reset()` method sets ERRORSRC to value 0"]
impl crate::Resettable for ERRORSRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
