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
#[doc = "Field `REPORTRDY_READCLRACC` reader - Shortcut between event REPORTRDY and task READCLRACC"]
pub type REPORTRDY_READCLRACC_R = crate::BitReader<REPORTRDY_READCLRACC_A>;
#[doc = "Shortcut between event REPORTRDY and task READCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPORTRDY_READCLRACC_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<REPORTRDY_READCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_READCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
impl REPORTRDY_READCLRACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_READCLRACC_A {
        match self.bits {
            false => REPORTRDY_READCLRACC_A::DISABLED,
            true => REPORTRDY_READCLRACC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDY_READCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_READCLRACC_A::ENABLED
    }
}
#[doc = "Field `REPORTRDY_READCLRACC` writer - Shortcut between event REPORTRDY and task READCLRACC"]
pub type REPORTRDY_READCLRACC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, REPORTRDY_READCLRACC_A, O>;
impl<'a, const O: u8> REPORTRDY_READCLRACC_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::ENABLED)
    }
}
#[doc = "Field `SAMPLERDY_STOP` reader - Shortcut between event SAMPLERDY and task STOP"]
pub type SAMPLERDY_STOP_R = crate::BitReader<SAMPLERDY_STOP_A>;
#[doc = "Shortcut between event SAMPLERDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPLERDY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SAMPLERDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPLERDY_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_STOP_A {
        match self.bits {
            false => SAMPLERDY_STOP_A::DISABLED,
            true => SAMPLERDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAMPLERDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDY_STOP_A::ENABLED
    }
}
#[doc = "Field `SAMPLERDY_STOP` writer - Shortcut between event SAMPLERDY and task STOP"]
pub type SAMPLERDY_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, SAMPLERDY_STOP_A, O>;
impl<'a, const O: u8> SAMPLERDY_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::ENABLED)
    }
}
#[doc = "Field `REPORTRDY_RDCLRACC` reader - Shortcut between event REPORTRDY and task RDCLRACC"]
pub type REPORTRDY_RDCLRACC_R = crate::BitReader<REPORTRDY_RDCLRACC_A>;
#[doc = "Shortcut between event REPORTRDY and task RDCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPORTRDY_RDCLRACC_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<REPORTRDY_RDCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_RDCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
impl REPORTRDY_RDCLRACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_RDCLRACC_A {
        match self.bits {
            false => REPORTRDY_RDCLRACC_A::DISABLED,
            true => REPORTRDY_RDCLRACC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDY_RDCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_RDCLRACC_A::ENABLED
    }
}
#[doc = "Field `REPORTRDY_RDCLRACC` writer - Shortcut between event REPORTRDY and task RDCLRACC"]
pub type REPORTRDY_RDCLRACC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, REPORTRDY_RDCLRACC_A, O>;
impl<'a, const O: u8> REPORTRDY_RDCLRACC_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_RDCLRACC_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_RDCLRACC_A::ENABLED)
    }
}
#[doc = "Field `REPORTRDY_STOP` reader - Shortcut between event REPORTRDY and task STOP"]
pub type REPORTRDY_STOP_R = crate::BitReader<REPORTRDY_STOP_A>;
#[doc = "Shortcut between event REPORTRDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPORTRDY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<REPORTRDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl REPORTRDY_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_STOP_A {
        match self.bits {
            false => REPORTRDY_STOP_A::DISABLED,
            true => REPORTRDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_STOP_A::ENABLED
    }
}
#[doc = "Field `REPORTRDY_STOP` writer - Shortcut between event REPORTRDY and task STOP"]
pub type REPORTRDY_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, REPORTRDY_STOP_A, O>;
impl<'a, const O: u8> REPORTRDY_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_STOP_A::ENABLED)
    }
}
#[doc = "Field `DBLRDY_RDCLRDBL` reader - Shortcut between event DBLRDY and task RDCLRDBL"]
pub type DBLRDY_RDCLRDBL_R = crate::BitReader<DBLRDY_RDCLRDBL_A>;
#[doc = "Shortcut between event DBLRDY and task RDCLRDBL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLRDY_RDCLRDBL_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DBLRDY_RDCLRDBL_A> for bool {
    #[inline(always)]
    fn from(variant: DBLRDY_RDCLRDBL_A) -> Self {
        variant as u8 != 0
    }
}
impl DBLRDY_RDCLRDBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLRDY_RDCLRDBL_A {
        match self.bits {
            false => DBLRDY_RDCLRDBL_A::DISABLED,
            true => DBLRDY_RDCLRDBL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBLRDY_RDCLRDBL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBLRDY_RDCLRDBL_A::ENABLED
    }
}
#[doc = "Field `DBLRDY_RDCLRDBL` writer - Shortcut between event DBLRDY and task RDCLRDBL"]
pub type DBLRDY_RDCLRDBL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, DBLRDY_RDCLRDBL_A, O>;
impl<'a, const O: u8> DBLRDY_RDCLRDBL_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLRDY_RDCLRDBL_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLRDY_RDCLRDBL_A::ENABLED)
    }
}
#[doc = "Field `DBLRDY_STOP` reader - Shortcut between event DBLRDY and task STOP"]
pub type DBLRDY_STOP_R = crate::BitReader<DBLRDY_STOP_A>;
#[doc = "Shortcut between event DBLRDY and task STOP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBLRDY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DBLRDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DBLRDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBLRDY_STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLRDY_STOP_A {
        match self.bits {
            false => DBLRDY_STOP_A::DISABLED,
            true => DBLRDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBLRDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBLRDY_STOP_A::ENABLED
    }
}
#[doc = "Field `DBLRDY_STOP` writer - Shortcut between event DBLRDY and task STOP"]
pub type DBLRDY_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHORTS_SPEC, DBLRDY_STOP_A, O>;
impl<'a, const O: u8> DBLRDY_STOP_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBLRDY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBLRDY_STOP_A::ENABLED)
    }
}
#[doc = "Field `SAMPLERDY_READCLRACC` reader - Shortcut between event SAMPLERDY and task READCLRACC"]
pub type SAMPLERDY_READCLRACC_R = crate::BitReader<SAMPLERDY_READCLRACC_A>;
#[doc = "Shortcut between event SAMPLERDY and task READCLRACC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMPLERDY_READCLRACC_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SAMPLERDY_READCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_READCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMPLERDY_READCLRACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_READCLRACC_A {
        match self.bits {
            false => SAMPLERDY_READCLRACC_A::DISABLED,
            true => SAMPLERDY_READCLRACC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAMPLERDY_READCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDY_READCLRACC_A::ENABLED
    }
}
#[doc = "Field `SAMPLERDY_READCLRACC` writer - Shortcut between event SAMPLERDY and task READCLRACC"]
pub type SAMPLERDY_READCLRACC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SHORTS_SPEC, SAMPLERDY_READCLRACC_A, O>;
impl<'a, const O: u8> SAMPLERDY_READCLRACC_W<'a, O> {
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_READCLRACC_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_READCLRACC_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    pub fn reportrdy_readclracc(&self) -> REPORTRDY_READCLRACC_R {
        REPORTRDY_READCLRACC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    pub fn samplerdy_stop(&self) -> SAMPLERDY_STOP_R {
        SAMPLERDY_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    pub fn reportrdy_rdclracc(&self) -> REPORTRDY_RDCLRACC_R {
        REPORTRDY_RDCLRACC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    pub fn reportrdy_stop(&self) -> REPORTRDY_STOP_R {
        REPORTRDY_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    pub fn dblrdy_rdclrdbl(&self) -> DBLRDY_RDCLRDBL_R {
        DBLRDY_RDCLRDBL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    pub fn dblrdy_stop(&self) -> DBLRDY_STOP_R {
        DBLRDY_STOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    pub fn samplerdy_readclracc(&self) -> SAMPLERDY_READCLRACC_R {
        SAMPLERDY_READCLRACC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between event REPORTRDY and task READCLRACC"]
    #[inline(always)]
    #[must_use]
    pub fn reportrdy_readclracc(&mut self) -> REPORTRDY_READCLRACC_W<0> {
        REPORTRDY_READCLRACC_W::new(self)
    }
    #[doc = "Bit 1 - Shortcut between event SAMPLERDY and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn samplerdy_stop(&mut self) -> SAMPLERDY_STOP_W<1> {
        SAMPLERDY_STOP_W::new(self)
    }
    #[doc = "Bit 2 - Shortcut between event REPORTRDY and task RDCLRACC"]
    #[inline(always)]
    #[must_use]
    pub fn reportrdy_rdclracc(&mut self) -> REPORTRDY_RDCLRACC_W<2> {
        REPORTRDY_RDCLRACC_W::new(self)
    }
    #[doc = "Bit 3 - Shortcut between event REPORTRDY and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn reportrdy_stop(&mut self) -> REPORTRDY_STOP_W<3> {
        REPORTRDY_STOP_W::new(self)
    }
    #[doc = "Bit 4 - Shortcut between event DBLRDY and task RDCLRDBL"]
    #[inline(always)]
    #[must_use]
    pub fn dblrdy_rdclrdbl(&mut self) -> DBLRDY_RDCLRDBL_W<4> {
        DBLRDY_RDCLRDBL_W::new(self)
    }
    #[doc = "Bit 5 - Shortcut between event DBLRDY and task STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dblrdy_stop(&mut self) -> DBLRDY_STOP_W<5> {
        DBLRDY_STOP_W::new(self)
    }
    #[doc = "Bit 6 - Shortcut between event SAMPLERDY and task READCLRACC"]
    #[inline(always)]
    #[must_use]
    pub fn samplerdy_readclracc(&mut self) -> SAMPLERDY_READCLRACC_W<6> {
        SAMPLERDY_READCLRACC_W::new(self)
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
