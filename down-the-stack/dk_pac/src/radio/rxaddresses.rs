#[doc = "Register `RXADDRESSES` reader"]
pub struct R(crate::R<RXADDRESSES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXADDRESSES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXADDRESSES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXADDRESSES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXADDRESSES` writer"]
pub struct W(crate::W<RXADDRESSES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXADDRESSES_SPEC>;
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
impl From<crate::W<RXADDRESSES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXADDRESSES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0` reader - Enable or disable reception on logical address 0."]
pub type ADDR0_R = crate::BitReader<ADDR0_A>;
#[doc = "Enable or disable reception on logical address 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR0_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR0_A {
        match self.bits {
            false => ADDR0_A::DISABLED,
            true => ADDR0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR0_A::ENABLED
    }
}
#[doc = "Field `ADDR0` writer - Enable or disable reception on logical address 0."]
pub type ADDR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR0_A, O>;
impl<'a, const O: u8> ADDR0_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR0_A::ENABLED)
    }
}
#[doc = "Field `ADDR1` reader - Enable or disable reception on logical address 1."]
pub type ADDR1_R = crate::BitReader<ADDR1_A>;
#[doc = "Enable or disable reception on logical address 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR1_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR1_A {
        match self.bits {
            false => ADDR1_A::DISABLED,
            true => ADDR1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR1_A::ENABLED
    }
}
#[doc = "Field `ADDR1` writer - Enable or disable reception on logical address 1."]
pub type ADDR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR1_A, O>;
impl<'a, const O: u8> ADDR1_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR1_A::ENABLED)
    }
}
#[doc = "Field `ADDR2` reader - Enable or disable reception on logical address 2."]
pub type ADDR2_R = crate::BitReader<ADDR2_A>;
#[doc = "Enable or disable reception on logical address 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR2_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR2_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR2_A {
        match self.bits {
            false => ADDR2_A::DISABLED,
            true => ADDR2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR2_A::ENABLED
    }
}
#[doc = "Field `ADDR2` writer - Enable or disable reception on logical address 2."]
pub type ADDR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR2_A, O>;
impl<'a, const O: u8> ADDR2_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR2_A::ENABLED)
    }
}
#[doc = "Field `ADDR3` reader - Enable or disable reception on logical address 3."]
pub type ADDR3_R = crate::BitReader<ADDR3_A>;
#[doc = "Enable or disable reception on logical address 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR3_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR3_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR3_A {
        match self.bits {
            false => ADDR3_A::DISABLED,
            true => ADDR3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR3_A::ENABLED
    }
}
#[doc = "Field `ADDR3` writer - Enable or disable reception on logical address 3."]
pub type ADDR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR3_A, O>;
impl<'a, const O: u8> ADDR3_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR3_A::ENABLED)
    }
}
#[doc = "Field `ADDR4` reader - Enable or disable reception on logical address 4."]
pub type ADDR4_R = crate::BitReader<ADDR4_A>;
#[doc = "Enable or disable reception on logical address 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR4_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR4_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR4_A {
        match self.bits {
            false => ADDR4_A::DISABLED,
            true => ADDR4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR4_A::ENABLED
    }
}
#[doc = "Field `ADDR4` writer - Enable or disable reception on logical address 4."]
pub type ADDR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR4_A, O>;
impl<'a, const O: u8> ADDR4_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR4_A::ENABLED)
    }
}
#[doc = "Field `ADDR5` reader - Enable or disable reception on logical address 5."]
pub type ADDR5_R = crate::BitReader<ADDR5_A>;
#[doc = "Enable or disable reception on logical address 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR5_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR5_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR5_A {
        match self.bits {
            false => ADDR5_A::DISABLED,
            true => ADDR5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR5_A::ENABLED
    }
}
#[doc = "Field `ADDR5` writer - Enable or disable reception on logical address 5."]
pub type ADDR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR5_A, O>;
impl<'a, const O: u8> ADDR5_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR5_A::ENABLED)
    }
}
#[doc = "Field `ADDR6` reader - Enable or disable reception on logical address 6."]
pub type ADDR6_R = crate::BitReader<ADDR6_A>;
#[doc = "Enable or disable reception on logical address 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR6_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR6_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR6_A {
        match self.bits {
            false => ADDR6_A::DISABLED,
            true => ADDR6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR6_A::ENABLED
    }
}
#[doc = "Field `ADDR6` writer - Enable or disable reception on logical address 6."]
pub type ADDR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR6_A, O>;
impl<'a, const O: u8> ADDR6_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR6_A::ENABLED)
    }
}
#[doc = "Field `ADDR7` reader - Enable or disable reception on logical address 7."]
pub type ADDR7_R = crate::BitReader<ADDR7_A>;
#[doc = "Enable or disable reception on logical address 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR7_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR7_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR7_A {
        match self.bits {
            false => ADDR7_A::DISABLED,
            true => ADDR7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR7_A::ENABLED
    }
}
#[doc = "Field `ADDR7` writer - Enable or disable reception on logical address 7."]
pub type ADDR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADDRESSES_SPEC, ADDR7_A, O>;
impl<'a, const O: u8> ADDR7_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR7_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub fn addr3(&self) -> ADDR3_R {
        ADDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub fn addr4(&self) -> ADDR4_R {
        ADDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub fn addr5(&self) -> ADDR5_R {
        ADDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub fn addr6(&self) -> ADDR6_R {
        ADDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub fn addr7(&self) -> ADDR7_R {
        ADDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable reception on logical address 0."]
    #[inline(always)]
    #[must_use]
    pub fn addr0(&mut self) -> ADDR0_W<0> {
        ADDR0_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable reception on logical address 1."]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> ADDR1_W<1> {
        ADDR1_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable reception on logical address 2."]
    #[inline(always)]
    #[must_use]
    pub fn addr2(&mut self) -> ADDR2_W<2> {
        ADDR2_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable reception on logical address 3."]
    #[inline(always)]
    #[must_use]
    pub fn addr3(&mut self) -> ADDR3_W<3> {
        ADDR3_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable reception on logical address 4."]
    #[inline(always)]
    #[must_use]
    pub fn addr4(&mut self) -> ADDR4_W<4> {
        ADDR4_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable reception on logical address 5."]
    #[inline(always)]
    #[must_use]
    pub fn addr5(&mut self) -> ADDR5_W<5> {
        ADDR5_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable reception on logical address 6."]
    #[inline(always)]
    #[must_use]
    pub fn addr6(&mut self) -> ADDR6_W<6> {
        ADDR6_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable reception on logical address 7."]
    #[inline(always)]
    #[must_use]
    pub fn addr7(&mut self) -> ADDR7_W<7> {
        ADDR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive address select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxaddresses](index.html) module"]
pub struct RXADDRESSES_SPEC;
impl crate::RegisterSpec for RXADDRESSES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxaddresses::R](R) reader structure"]
impl crate::Readable for RXADDRESSES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxaddresses::W](W) writer structure"]
impl crate::Writable for RXADDRESSES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXADDRESSES to value 0"]
impl crate::Resettable for RXADDRESSES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
