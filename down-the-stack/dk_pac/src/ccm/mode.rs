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
#[doc = "Field `MODE` reader - The mode of operation to be used"]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "The mode of operation to be used\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: AES CCM packet encryption mode"]
    ENCRYPTION = 0,
    #[doc = "1: AES CCM packet decryption mode"]
    DECRYPTION = 1,
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
            false => MODE_A::ENCRYPTION,
            true => MODE_A::DECRYPTION,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPTION`"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == MODE_A::ENCRYPTION
    }
    #[doc = "Checks if the value of the field is `DECRYPTION`"]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == MODE_A::DECRYPTION
    }
}
#[doc = "Field `MODE` writer - The mode of operation to be used"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "AES CCM packet encryption mode"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut W {
        self.variant(MODE_A::ENCRYPTION)
    }
    #[doc = "AES CCM packet decryption mode"]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut W {
        self.variant(MODE_A::DECRYPTION)
    }
}
#[doc = "Field `DATARATE` reader - Data rate that the CCM shall run in synch with"]
pub type DATARATE_R = crate::BitReader<DATARATE_A>;
#[doc = "Data rate that the CCM shall run in synch with\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DATARATE_A {
    #[doc = "0: In synch with 1 Mbit data rate"]
    _1MBIT = 0,
    #[doc = "1: In synch with 2 Mbit data rate"]
    _2MBIT = 1,
}
impl From<DATARATE_A> for bool {
    #[inline(always)]
    fn from(variant: DATARATE_A) -> Self {
        variant as u8 != 0
    }
}
impl DATARATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATARATE_A {
        match self.bits {
            false => DATARATE_A::_1MBIT,
            true => DATARATE_A::_2MBIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1MBIT`"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == DATARATE_A::_1MBIT
    }
    #[doc = "Checks if the value of the field is `_2MBIT`"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == DATARATE_A::_2MBIT
    }
}
#[doc = "Field `DATARATE` writer - Data rate that the CCM shall run in synch with"]
pub type DATARATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, DATARATE_A, O>;
impl<'a, const O: u8> DATARATE_W<'a, O> {
    #[doc = "In synch with 1 Mbit data rate"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut W {
        self.variant(DATARATE_A::_1MBIT)
    }
    #[doc = "In synch with 2 Mbit data rate"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut W {
        self.variant(DATARATE_A::_2MBIT)
    }
}
#[doc = "Field `LENGTH` reader - Packet length configuration"]
pub type LENGTH_R = crate::BitReader<LENGTH_A>;
#[doc = "Packet length configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LENGTH_A {
    #[doc = "0: Default length. Effective length of LENGTH field is 5-bit"]
    DEFAULT = 0,
    #[doc = "1: Extended length. Effective length of LENGTH field is 8-bit"]
    EXTENDED = 1,
}
impl From<LENGTH_A> for bool {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as u8 != 0
    }
}
impl LENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LENGTH_A {
        match self.bits {
            false => LENGTH_A::DEFAULT,
            true => LENGTH_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LENGTH_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LENGTH_A::EXTENDED
    }
}
#[doc = "Field `LENGTH` writer - Packet length configuration"]
pub type LENGTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, LENGTH_A, O>;
impl<'a, const O: u8> LENGTH_W<'a, O> {
    #[doc = "Default length. Effective length of LENGTH field is 5-bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LENGTH_A::DEFAULT)
    }
    #[doc = "Extended length. Effective length of LENGTH field is 8-bit"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(LENGTH_A::EXTENDED)
    }
}
impl R {
    #[doc = "Bit 0 - The mode of operation to be used"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Data rate that the CCM shall run in synch with"]
    #[inline(always)]
    pub fn datarate(&self) -> DATARATE_R {
        DATARATE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The mode of operation to be used"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 16 - Data rate that the CCM shall run in synch with"]
    #[inline(always)]
    #[must_use]
    pub fn datarate(&mut self) -> DATARATE_W<16> {
        DATARATE_W::new(self)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<24> {
        LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operation mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
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
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
