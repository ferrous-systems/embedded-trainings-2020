#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READY_START` reader - Shortcut between event READY and task START"]
pub type READY_START_R = crate::BitReader<READY_START_A>;
#[doc = "Shortcut between event READY and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_START_A> for bool {
    #[inline(always)]
    fn from(variant: READY_START_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_START_A {
        match self.bits {
            false => READY_START_A::DISABLED,
            true => READY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_START_A::ENABLED
    }
}
#[doc = "Field `READY_START` writer - Shortcut between event READY and task START"]
pub type READY_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, READY_START_A, O>;
impl<'a, const O: u8> READY_START_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_START_A::ENABLED)
    }
}
#[doc = "Field `END_DISABLE` reader - Shortcut between event END and task DISABLE"]
pub type END_DISABLE_R = crate::BitReader<END_DISABLE_A>;
#[doc = "Shortcut between event END and task DISABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum END_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: END_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl END_DISABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_DISABLE_A {
        match self.bits {
            false => END_DISABLE_A::DISABLED,
            true => END_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_DISABLE_A::ENABLED
    }
}
#[doc = "Field `END_DISABLE` writer - Shortcut between event END and task DISABLE"]
pub type END_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, END_DISABLE_A, O>;
impl<'a, const O: u8> END_DISABLE_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::ENABLED)
    }
}
#[doc = "Field `DISABLED_TXEN` reader - Shortcut between event DISABLED and task TXEN"]
pub type DISABLED_TXEN_R = crate::BitReader<DISABLED_TXEN_A>;
#[doc = "Shortcut between event DISABLED and task TXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLED_TXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_TXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLED_TXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_TXEN_A {
        match self.bits {
            false => DISABLED_TXEN_A::DISABLED,
            true => DISABLED_TXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_TXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_TXEN_A::ENABLED
    }
}
#[doc = "Field `DISABLED_TXEN` writer - Shortcut between event DISABLED and task TXEN"]
pub type DISABLED_TXEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, DISABLED_TXEN_A, O>;
impl<'a, const O: u8> DISABLED_TXEN_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::ENABLED)
    }
}
#[doc = "Field `DISABLED_RXEN` reader - Shortcut between event DISABLED and task RXEN"]
pub type DISABLED_RXEN_R = crate::BitReader<DISABLED_RXEN_A>;
#[doc = "Shortcut between event DISABLED and task RXEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLED_RXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLED_RXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RXEN_A {
        match self.bits {
            false => DISABLED_RXEN_A::DISABLED,
            true => DISABLED_RXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RXEN_A::ENABLED
    }
}
#[doc = "Field `DISABLED_RXEN` writer - Shortcut between event DISABLED and task RXEN"]
pub type DISABLED_RXEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, DISABLED_RXEN_A, O>;
impl<'a, const O: u8> DISABLED_RXEN_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::ENABLED)
    }
}
#[doc = "Field `ADDRESS_RSSISTART` reader - Shortcut between event ADDRESS and task RSSISTART"]
pub type ADDRESS_RSSISTART_R = crate::BitReader<ADDRESS_RSSISTART_A>;
#[doc = "Shortcut between event ADDRESS and task RSSISTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRESS_RSSISTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_RSSISTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_RSSISTART_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRESS_RSSISTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_RSSISTART_A {
        match self.bits {
            false => ADDRESS_RSSISTART_A::DISABLED,
            true => ADDRESS_RSSISTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_RSSISTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_RSSISTART_A::ENABLED
    }
}
#[doc = "Field `ADDRESS_RSSISTART` writer - Shortcut between event ADDRESS and task RSSISTART"]
pub type ADDRESS_RSSISTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, ADDRESS_RSSISTART_A, O>;
impl<'a, const O: u8> ADDRESS_RSSISTART_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::ENABLED)
    }
}
#[doc = "Field `END_START` reader - Shortcut between event END and task START"]
pub type END_START_R = crate::BitReader<END_START_A>;
#[doc = "Shortcut between event END and task START\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum END_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_START_A> for bool {
    #[inline(always)]
    fn from(variant: END_START_A) -> Self {
        variant as u8 != 0
    }
}
impl END_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_START_A {
        match self.bits {
            false => END_START_A::DISABLED,
            true => END_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_START_A::ENABLED
    }
}
#[doc = "Field `END_START` writer - Shortcut between event END and task START"]
pub type END_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, END_START_A, O>;
impl<'a, const O: u8> END_START_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_START_A::ENABLED)
    }
}
#[doc = "Field `ADDRESS_BCSTART` reader - Shortcut between event ADDRESS and task BCSTART"]
pub type ADDRESS_BCSTART_R = crate::BitReader<ADDRESS_BCSTART_A>;
#[doc = "Shortcut between event ADDRESS and task BCSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRESS_BCSTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_BCSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_BCSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRESS_BCSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_BCSTART_A {
        match self.bits {
            false => ADDRESS_BCSTART_A::DISABLED,
            true => ADDRESS_BCSTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_BCSTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_BCSTART_A::ENABLED
    }
}
#[doc = "Field `ADDRESS_BCSTART` writer - Shortcut between event ADDRESS and task BCSTART"]
pub type ADDRESS_BCSTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, ADDRESS_BCSTART_A, O>;
impl<'a, const O: u8> ADDRESS_BCSTART_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::ENABLED)
    }
}
#[doc = "Field `DISABLED_RSSISTOP` reader - Shortcut between event DISABLED and task RSSISTOP"]
pub type DISABLED_RSSISTOP_R = crate::BitReader<DISABLED_RSSISTOP_A>;
#[doc = "Shortcut between event DISABLED and task RSSISTOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLED_RSSISTOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RSSISTOP_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RSSISTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLED_RSSISTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RSSISTOP_A {
        match self.bits {
            false => DISABLED_RSSISTOP_A::DISABLED,
            true => DISABLED_RSSISTOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RSSISTOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RSSISTOP_A::ENABLED
    }
}
#[doc = "Field `DISABLED_RSSISTOP` writer - Shortcut between event DISABLED and task RSSISTOP"]
pub type DISABLED_RSSISTOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, DISABLED_RSSISTOP_A, O>;
impl<'a, const O: u8> DISABLED_RSSISTOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event READY and task START"]
    #[inline(always)]
    pub fn ready_start(&self) -> READY_START_R {
        READY_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event END and task DISABLE"]
    #[inline(always)]
    pub fn end_disable(&self) -> END_DISABLE_R {
        END_DISABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    pub fn disabled_txen(&self) -> DISABLED_TXEN_R {
        DISABLED_TXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    pub fn disabled_rxen(&self) -> DISABLED_RXEN_R {
        DISABLED_RXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    pub fn address_rssistart(&self) -> ADDRESS_RSSISTART_R {
        ADDRESS_RSSISTART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event END and task START"]
    #[inline(always)]
    pub fn end_start(&self) -> END_START_R {
        END_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    pub fn address_bcstart(&self) -> ADDRESS_BCSTART_R {
        ADDRESS_BCSTART_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    pub fn disabled_rssistop(&self) -> DISABLED_RSSISTOP_R {
        DISABLED_RSSISTOP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event READY and task START"]
    #[inline(always)]
    #[must_use]
    pub fn ready_start(&mut self) -> READY_START_W<0> {
        READY_START_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event END and task DISABLE"]
    #[inline(always)]
    #[must_use]
    pub fn end_disable(&mut self) -> END_DISABLE_W<1> {
        END_DISABLE_W::new(self)
    }
    #[doc = "Bit 2 - Shortcut between event DISABLED and task TXEN"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_txen(&mut self) -> DISABLED_TXEN_W<2> {
        DISABLED_TXEN_W::new(self)
    }
    #[doc = "Bit 3 - Shortcut between event DISABLED and task RXEN"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_rxen(&mut self) -> DISABLED_RXEN_W<3> {
        DISABLED_RXEN_W::new(self)
    }
    #[doc = "Bit 4 - Shortcut between event ADDRESS and task RSSISTART"]
    #[inline(always)]
    #[must_use]
    pub fn address_rssistart(&mut self) -> ADDRESS_RSSISTART_W<4> {
        ADDRESS_RSSISTART_W::new(self)
    }
    #[doc = "Bit 5 - Shortcut between event END and task START"]
    #[inline(always)]
    #[must_use]
    pub fn end_start(&mut self) -> END_START_W<5> {
        END_START_W::new(self)
    }
    #[doc = "Bit 6 - Shortcut between event ADDRESS and task BCSTART"]
    #[inline(always)]
    #[must_use]
    pub fn address_bcstart(&mut self) -> ADDRESS_BCSTART_W<6> {
        ADDRESS_BCSTART_W::new(self)
    }
    #[doc = "Bit 8 - Shortcut between event DISABLED and task RSSISTOP"]
    #[inline(always)]
    #[must_use]
    pub fn disabled_rssistop(&mut self) -> DISABLED_RSSISTOP_W<8> {
        DISABLED_RSSISTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
