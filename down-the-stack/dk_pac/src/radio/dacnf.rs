#[doc = "Register `DACNF` reader"]
pub struct R(crate::R<DACNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACNF` writer"]
pub struct W(crate::W<DACNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACNF_SPEC>;
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
impl From<crate::W<DACNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA0` reader - Enable or disable device address matching using device address 0"]
pub type ENA0_R = crate::BitReader<ENA0_A>;
#[doc = "Enable or disable device address matching using device address 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA0_A> for bool {
    #[inline(always)]
    fn from(variant: ENA0_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA0_A {
        match self.bits {
            false => ENA0_A::DISABLED,
            true => ENA0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA0_A::ENABLED
    }
}
#[doc = "Field `ENA0` writer - Enable or disable device address matching using device address 0"]
pub type ENA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA0_A, O>;
impl<'a, const O: u8> ENA0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA0_A::ENABLED)
    }
}
#[doc = "Field `ENA1` reader - Enable or disable device address matching using device address 1"]
pub type ENA1_R = crate::BitReader<ENA1_A>;
#[doc = "Enable or disable device address matching using device address 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA1_A> for bool {
    #[inline(always)]
    fn from(variant: ENA1_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA1_A {
        match self.bits {
            false => ENA1_A::DISABLED,
            true => ENA1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA1_A::ENABLED
    }
}
#[doc = "Field `ENA1` writer - Enable or disable device address matching using device address 1"]
pub type ENA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA1_A, O>;
impl<'a, const O: u8> ENA1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA1_A::ENABLED)
    }
}
#[doc = "Field `ENA2` reader - Enable or disable device address matching using device address 2"]
pub type ENA2_R = crate::BitReader<ENA2_A>;
#[doc = "Enable or disable device address matching using device address 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA2_A> for bool {
    #[inline(always)]
    fn from(variant: ENA2_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA2_A {
        match self.bits {
            false => ENA2_A::DISABLED,
            true => ENA2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA2_A::ENABLED
    }
}
#[doc = "Field `ENA2` writer - Enable or disable device address matching using device address 2"]
pub type ENA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA2_A, O>;
impl<'a, const O: u8> ENA2_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA2_A::ENABLED)
    }
}
#[doc = "Field `ENA3` reader - Enable or disable device address matching using device address 3"]
pub type ENA3_R = crate::BitReader<ENA3_A>;
#[doc = "Enable or disable device address matching using device address 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA3_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA3_A> for bool {
    #[inline(always)]
    fn from(variant: ENA3_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA3_A {
        match self.bits {
            false => ENA3_A::DISABLED,
            true => ENA3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA3_A::ENABLED
    }
}
#[doc = "Field `ENA3` writer - Enable or disable device address matching using device address 3"]
pub type ENA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA3_A, O>;
impl<'a, const O: u8> ENA3_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA3_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA3_A::ENABLED)
    }
}
#[doc = "Field `ENA4` reader - Enable or disable device address matching using device address 4"]
pub type ENA4_R = crate::BitReader<ENA4_A>;
#[doc = "Enable or disable device address matching using device address 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA4_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA4_A> for bool {
    #[inline(always)]
    fn from(variant: ENA4_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA4_A {
        match self.bits {
            false => ENA4_A::DISABLED,
            true => ENA4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA4_A::ENABLED
    }
}
#[doc = "Field `ENA4` writer - Enable or disable device address matching using device address 4"]
pub type ENA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA4_A, O>;
impl<'a, const O: u8> ENA4_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA4_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA4_A::ENABLED)
    }
}
#[doc = "Field `ENA5` reader - Enable or disable device address matching using device address 5"]
pub type ENA5_R = crate::BitReader<ENA5_A>;
#[doc = "Enable or disable device address matching using device address 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA5_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA5_A> for bool {
    #[inline(always)]
    fn from(variant: ENA5_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA5_A {
        match self.bits {
            false => ENA5_A::DISABLED,
            true => ENA5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA5_A::ENABLED
    }
}
#[doc = "Field `ENA5` writer - Enable or disable device address matching using device address 5"]
pub type ENA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA5_A, O>;
impl<'a, const O: u8> ENA5_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA5_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA5_A::ENABLED)
    }
}
#[doc = "Field `ENA6` reader - Enable or disable device address matching using device address 6"]
pub type ENA6_R = crate::BitReader<ENA6_A>;
#[doc = "Enable or disable device address matching using device address 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA6_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA6_A> for bool {
    #[inline(always)]
    fn from(variant: ENA6_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA6_A {
        match self.bits {
            false => ENA6_A::DISABLED,
            true => ENA6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA6_A::ENABLED
    }
}
#[doc = "Field `ENA6` writer - Enable or disable device address matching using device address 6"]
pub type ENA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA6_A, O>;
impl<'a, const O: u8> ENA6_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA6_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA6_A::ENABLED)
    }
}
#[doc = "Field `ENA7` reader - Enable or disable device address matching using device address 7"]
pub type ENA7_R = crate::BitReader<ENA7_A>;
#[doc = "Enable or disable device address matching using device address 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA7_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA7_A> for bool {
    #[inline(always)]
    fn from(variant: ENA7_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA7_A {
        match self.bits {
            false => ENA7_A::DISABLED,
            true => ENA7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA7_A::ENABLED
    }
}
#[doc = "Field `ENA7` writer - Enable or disable device address matching using device address 7"]
pub type ENA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, ENA7_A, O>;
impl<'a, const O: u8> ENA7_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA7_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA7_A::ENABLED)
    }
}
#[doc = "Field `TXADD0` reader - TxAdd for device address 0"]
pub type TXADD0_R = crate::BitReader<bool>;
#[doc = "Field `TXADD0` writer - TxAdd for device address 0"]
pub type TXADD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
#[doc = "Field `TXADD1` reader - TxAdd for device address 1"]
pub type TXADD1_R = crate::BitReader<bool>;
#[doc = "Field `TXADD1` writer - TxAdd for device address 1"]
pub type TXADD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
#[doc = "Field `TXADD2` reader - TxAdd for device address 2"]
pub type TXADD2_R = crate::BitReader<bool>;
#[doc = "Field `TXADD2` writer - TxAdd for device address 2"]
pub type TXADD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
#[doc = "Field `TXADD3` reader - TxAdd for device address 3"]
pub type TXADD3_R = crate::BitReader<bool>;
#[doc = "Field `TXADD3` writer - TxAdd for device address 3"]
pub type TXADD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
#[doc = "Field `TXADD4` reader - TxAdd for device address 4"]
pub type TXADD4_R = crate::BitReader<bool>;
#[doc = "Field `TXADD4` writer - TxAdd for device address 4"]
pub type TXADD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
#[doc = "Field `TXADD5` reader - TxAdd for device address 5"]
pub type TXADD5_R = crate::BitReader<bool>;
#[doc = "Field `TXADD5` writer - TxAdd for device address 5"]
pub type TXADD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
#[doc = "Field `TXADD6` reader - TxAdd for device address 6"]
pub type TXADD6_R = crate::BitReader<bool>;
#[doc = "Field `TXADD6` writer - TxAdd for device address 6"]
pub type TXADD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
#[doc = "Field `TXADD7` reader - TxAdd for device address 7"]
pub type TXADD7_R = crate::BitReader<bool>;
#[doc = "Field `TXADD7` writer - TxAdd for device address 7"]
pub type TXADD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACNF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn ena0(&self) -> ENA0_R {
        ENA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn ena1(&self) -> ENA1_R {
        ENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn ena2(&self) -> ENA2_R {
        ENA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn ena3(&self) -> ENA3_R {
        ENA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn ena4(&self) -> ENA4_R {
        ENA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn ena5(&self) -> ENA5_R {
        ENA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn ena6(&self) -> ENA6_R {
        ENA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn ena7(&self) -> ENA7_R {
        ENA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    pub fn txadd0(&self) -> TXADD0_R {
        TXADD0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    pub fn txadd1(&self) -> TXADD1_R {
        TXADD1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    pub fn txadd2(&self) -> TXADD2_R {
        TXADD2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    pub fn txadd3(&self) -> TXADD3_R {
        TXADD3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    pub fn txadd4(&self) -> TXADD4_R {
        TXADD4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    pub fn txadd5(&self) -> TXADD5_R {
        TXADD5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    pub fn txadd6(&self) -> TXADD6_R {
        TXADD6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    pub fn txadd7(&self) -> TXADD7_R {
        TXADD7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    #[must_use]
    pub fn ena0(&mut self) -> ENA0_W<0> {
        ENA0_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    #[must_use]
    pub fn ena1(&mut self) -> ENA1_W<1> {
        ENA1_W::new(self)
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    #[must_use]
    pub fn ena2(&mut self) -> ENA2_W<2> {
        ENA2_W::new(self)
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    #[must_use]
    pub fn ena3(&mut self) -> ENA3_W<3> {
        ENA3_W::new(self)
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    #[must_use]
    pub fn ena4(&mut self) -> ENA4_W<4> {
        ENA4_W::new(self)
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    #[must_use]
    pub fn ena5(&mut self) -> ENA5_W<5> {
        ENA5_W::new(self)
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    #[must_use]
    pub fn ena6(&mut self) -> ENA6_W<6> {
        ENA6_W::new(self)
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    #[must_use]
    pub fn ena7(&mut self) -> ENA7_W<7> {
        ENA7_W::new(self)
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    #[must_use]
    pub fn txadd0(&mut self) -> TXADD0_W<8> {
        TXADD0_W::new(self)
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    #[must_use]
    pub fn txadd1(&mut self) -> TXADD1_W<9> {
        TXADD1_W::new(self)
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    #[must_use]
    pub fn txadd2(&mut self) -> TXADD2_W<10> {
        TXADD2_W::new(self)
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    #[must_use]
    pub fn txadd3(&mut self) -> TXADD3_W<11> {
        TXADD3_W::new(self)
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    #[must_use]
    pub fn txadd4(&mut self) -> TXADD4_W<12> {
        TXADD4_W::new(self)
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    #[must_use]
    pub fn txadd5(&mut self) -> TXADD5_W<13> {
        TXADD5_W::new(self)
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    #[must_use]
    pub fn txadd6(&mut self) -> TXADD6_W<14> {
        TXADD6_W::new(self)
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    #[must_use]
    pub fn txadd7(&mut self) -> TXADD7_W<15> {
        TXADD7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device address match configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacnf](index.html) module"]
pub struct DACNF_SPEC;
impl crate::RegisterSpec for DACNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacnf::R](R) reader structure"]
impl crate::Readable for DACNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacnf::W](W) writer structure"]
impl crate::Writable for DACNF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACNF to value 0"]
impl crate::Resettable for DACNF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
