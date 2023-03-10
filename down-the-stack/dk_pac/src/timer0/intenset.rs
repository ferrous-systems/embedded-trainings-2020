#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARE0` reader - Write '1' to enable interrupt for event COMPARE\\[0\\]"]
pub type COMPARE0_R = crate::BitReader<COMPARE0_A>;
#[doc = "Write '1' to enable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE0_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_A {
        match self.bits {
            false => COMPARE0_A::DISABLED,
            true => COMPARE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event COMPARE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE0_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE0_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` writer - Write '1' to enable interrupt for event COMPARE\\[0\\]"]
pub type COMPARE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, COMPARE0_AW, O>;
impl<'a, const O: u8> COMPARE0_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE0_AW::SET)
    }
}
#[doc = "Field `COMPARE1` reader - Write '1' to enable interrupt for event COMPARE\\[1\\]"]
pub type COMPARE1_R = crate::BitReader<COMPARE1_A>;
#[doc = "Write '1' to enable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE1_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_A {
        match self.bits {
            false => COMPARE1_A::DISABLED,
            true => COMPARE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event COMPARE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE1_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE1_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` writer - Write '1' to enable interrupt for event COMPARE\\[1\\]"]
pub type COMPARE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, COMPARE1_AW, O>;
impl<'a, const O: u8> COMPARE1_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE1_AW::SET)
    }
}
#[doc = "Field `COMPARE2` reader - Write '1' to enable interrupt for event COMPARE\\[2\\]"]
pub type COMPARE2_R = crate::BitReader<COMPARE2_A>;
#[doc = "Write '1' to enable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE2_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_A {
        match self.bits {
            false => COMPARE2_A::DISABLED,
            true => COMPARE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event COMPARE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE2_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE2_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` writer - Write '1' to enable interrupt for event COMPARE\\[2\\]"]
pub type COMPARE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, COMPARE2_AW, O>;
impl<'a, const O: u8> COMPARE2_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE2_AW::SET)
    }
}
#[doc = "Field `COMPARE3` reader - Write '1' to enable interrupt for event COMPARE\\[3\\]"]
pub type COMPARE3_R = crate::BitReader<COMPARE3_A>;
#[doc = "Write '1' to enable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE3_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_A {
        match self.bits {
            false => COMPARE3_A::DISABLED,
            true => COMPARE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event COMPARE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE3_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE3_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` writer - Write '1' to enable interrupt for event COMPARE\\[3\\]"]
pub type COMPARE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, COMPARE3_AW, O>;
impl<'a, const O: u8> COMPARE3_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE3_AW::SET)
    }
}
#[doc = "Field `COMPARE4` reader - Write '1' to enable interrupt for event COMPARE\\[4\\]"]
pub type COMPARE4_R = crate::BitReader<COMPARE4_A>;
#[doc = "Write '1' to enable interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE4_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE4_A {
        match self.bits {
            false => COMPARE4_A::DISABLED,
            true => COMPARE4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE4_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event COMPARE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE4_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE4_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE4` writer - Write '1' to enable interrupt for event COMPARE\\[4\\]"]
pub type COMPARE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, COMPARE4_AW, O>;
impl<'a, const O: u8> COMPARE4_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE4_AW::SET)
    }
}
#[doc = "Field `COMPARE5` reader - Write '1' to enable interrupt for event COMPARE\\[5\\]"]
pub type COMPARE5_R = crate::BitReader<COMPARE5_A>;
#[doc = "Write '1' to enable interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE5_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_A) -> Self {
        variant as u8 != 0
    }
}
impl COMPARE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE5_A {
        match self.bits {
            false => COMPARE5_A::DISABLED,
            true => COMPARE5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE5_A::ENABLED
    }
}
#[doc = "Write '1' to enable interrupt for event COMPARE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMPARE5_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE5_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE5` writer - Write '1' to enable interrupt for event COMPARE\\[5\\]"]
pub type COMPARE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTENSET_SPEC, COMPARE5_AW, O>;
impl<'a, const O: u8> COMPARE5_W<'a, O> {
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE5_AW::SET)
    }
}
impl R {
    #[doc = "Bit 16 - Write '1' to enable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    pub fn compare0(&self) -> COMPARE0_R {
        COMPARE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    pub fn compare1(&self) -> COMPARE1_R {
        COMPARE1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    pub fn compare2(&self) -> COMPARE2_R {
        COMPARE2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    pub fn compare3(&self) -> COMPARE3_R {
        COMPARE3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    pub fn compare4(&self) -> COMPARE4_R {
        COMPARE4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    pub fn compare5(&self) -> COMPARE5_R {
        COMPARE5_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Write '1' to enable interrupt for event COMPARE\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare0(&mut self) -> COMPARE0_W<16> {
        COMPARE0_W::new(self)
    }
    #[doc = "Bit 17 - Write '1' to enable interrupt for event COMPARE\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare1(&mut self) -> COMPARE1_W<17> {
        COMPARE1_W::new(self)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event COMPARE\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare2(&mut self) -> COMPARE2_W<18> {
        COMPARE2_W::new(self)
    }
    #[doc = "Bit 19 - Write '1' to enable interrupt for event COMPARE\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare3(&mut self) -> COMPARE3_W<19> {
        COMPARE3_W::new(self)
    }
    #[doc = "Bit 20 - Write '1' to enable interrupt for event COMPARE\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare4(&mut self) -> COMPARE4_W<20> {
        COMPARE4_W::new(self)
    }
    #[doc = "Bit 21 - Write '1' to enable interrupt for event COMPARE\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn compare5(&mut self) -> COMPARE5_W<21> {
        COMPARE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
