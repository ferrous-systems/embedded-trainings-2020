#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ORDER` reader - Bit order"]
pub type ORDER_R = crate::BitReader<ORDER_A>;
#[doc = "Bit order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORDER_A {
    #[doc = "0: Most significant bit shifted out first"]
    MSB_FIRST = 0,
    #[doc = "1: Least significant bit shifted out first"]
    LSB_FIRST = 1,
}
impl From<ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: ORDER_A) -> Self {
        variant as u8 != 0
    }
}
impl ORDER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORDER_A {
        match self.bits {
            false => ORDER_A::MSB_FIRST,
            true => ORDER_A::LSB_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSB_FIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == ORDER_A::MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `LSB_FIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == ORDER_A::LSB_FIRST
    }
}
#[doc = "Field `ORDER` writer - Bit order"]
pub type ORDER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, ORDER_A, O>;
impl<'a, const O: u8> ORDER_W<'a, O> {
    #[doc = "Most significant bit shifted out first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(ORDER_A::MSB_FIRST)
    }
    #[doc = "Least significant bit shifted out first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(ORDER_A::LSB_FIRST)
    }
}
#[doc = "Field `CPHA` reader - Serial clock (SCK) phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Serial clock (SCK) phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING = 0,
    #[doc = "1: Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::LEADING,
            true => CPHA_A::TRAILING,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING`"]
    #[inline(always)]
    pub fn is_leading(&self) -> bool {
        *self == CPHA_A::LEADING
    }
    #[doc = "Checks if the value of the field is `TRAILING`"]
    #[inline(always)]
    pub fn is_trailing(&self) -> bool {
        *self == CPHA_A::TRAILING
    }
}
#[doc = "Field `CPHA` writer - Serial clock (SCK) phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    #[inline(always)]
    pub fn leading(self) -> &'a mut W {
        self.variant(CPHA_A::LEADING)
    }
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    #[inline(always)]
    pub fn trailing(self) -> &'a mut W {
        self.variant(CPHA_A::TRAILING)
    }
}
#[doc = "Field `CPOL` reader - Serial clock (SCK) polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Serial clock (SCK) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: Active high"]
    ACTIVE_HIGH = 0,
    #[doc = "1: Active low"]
    ACTIVE_LOW = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::ACTIVE_HIGH,
            true => CPOL_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == CPOL_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == CPOL_A::ACTIVE_LOW
    }
}
#[doc = "Field `CPOL` writer - Serial clock (SCK) polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CPOL_A::ACTIVE_HIGH)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CPOL_A::ACTIVE_LOW)
    }
}
impl R {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    pub fn order(&self) -> ORDER_R {
        ORDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    #[must_use]
    pub fn order(&mut self) -> ORDER_W<0> {
        ORDER_W::new(self)
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<1> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<2> {
        CPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
