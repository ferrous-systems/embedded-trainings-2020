#[doc = "Register `SUBSTATRA` reader"]
pub struct R(crate::R<SUBSTATRA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBSTATRA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBSTATRA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBSTATRA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBSTATRA` writer"]
pub struct W(crate::W<SUBSTATRA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBSTATRA_SPEC>;
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
impl From<crate::W<SUBSTATRA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBSTATRA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR0` reader - Subregion 0 in region n (write '1' to clear)"]
pub type SR0_R = crate::BitReader<SR0_A>;
#[doc = "Subregion 0 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR0_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR0_A> for bool {
    #[inline(always)]
    fn from(variant: SR0_A) -> Self {
        variant as u8 != 0
    }
}
impl SR0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR0_A {
        match self.bits {
            false => SR0_A::NO_ACCESS,
            true => SR0_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR0_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR0_A::ACCESS
    }
}
#[doc = "Field `SR0` writer - Subregion 0 in region n (write '1' to clear)"]
pub type SR0_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR0_A, O>;
impl<'a, const O: u8> SR0_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR0_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR0_A::ACCESS)
    }
}
#[doc = "Field `SR1` reader - Subregion 1 in region n (write '1' to clear)"]
pub type SR1_R = crate::BitReader<SR1_A>;
#[doc = "Subregion 1 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR1_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR1_A> for bool {
    #[inline(always)]
    fn from(variant: SR1_A) -> Self {
        variant as u8 != 0
    }
}
impl SR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR1_A {
        match self.bits {
            false => SR1_A::NO_ACCESS,
            true => SR1_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR1_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR1_A::ACCESS
    }
}
#[doc = "Field `SR1` writer - Subregion 1 in region n (write '1' to clear)"]
pub type SR1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR1_A, O>;
impl<'a, const O: u8> SR1_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR1_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR1_A::ACCESS)
    }
}
#[doc = "Field `SR2` reader - Subregion 2 in region n (write '1' to clear)"]
pub type SR2_R = crate::BitReader<SR2_A>;
#[doc = "Subregion 2 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR2_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR2_A> for bool {
    #[inline(always)]
    fn from(variant: SR2_A) -> Self {
        variant as u8 != 0
    }
}
impl SR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR2_A {
        match self.bits {
            false => SR2_A::NO_ACCESS,
            true => SR2_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR2_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR2_A::ACCESS
    }
}
#[doc = "Field `SR2` writer - Subregion 2 in region n (write '1' to clear)"]
pub type SR2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR2_A, O>;
impl<'a, const O: u8> SR2_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR2_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR2_A::ACCESS)
    }
}
#[doc = "Field `SR3` reader - Subregion 3 in region n (write '1' to clear)"]
pub type SR3_R = crate::BitReader<SR3_A>;
#[doc = "Subregion 3 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR3_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR3_A> for bool {
    #[inline(always)]
    fn from(variant: SR3_A) -> Self {
        variant as u8 != 0
    }
}
impl SR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR3_A {
        match self.bits {
            false => SR3_A::NO_ACCESS,
            true => SR3_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR3_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR3_A::ACCESS
    }
}
#[doc = "Field `SR3` writer - Subregion 3 in region n (write '1' to clear)"]
pub type SR3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR3_A, O>;
impl<'a, const O: u8> SR3_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR3_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR3_A::ACCESS)
    }
}
#[doc = "Field `SR4` reader - Subregion 4 in region n (write '1' to clear)"]
pub type SR4_R = crate::BitReader<SR4_A>;
#[doc = "Subregion 4 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR4_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR4_A> for bool {
    #[inline(always)]
    fn from(variant: SR4_A) -> Self {
        variant as u8 != 0
    }
}
impl SR4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR4_A {
        match self.bits {
            false => SR4_A::NO_ACCESS,
            true => SR4_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR4_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR4_A::ACCESS
    }
}
#[doc = "Field `SR4` writer - Subregion 4 in region n (write '1' to clear)"]
pub type SR4_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR4_A, O>;
impl<'a, const O: u8> SR4_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR4_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR4_A::ACCESS)
    }
}
#[doc = "Field `SR5` reader - Subregion 5 in region n (write '1' to clear)"]
pub type SR5_R = crate::BitReader<SR5_A>;
#[doc = "Subregion 5 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR5_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR5_A> for bool {
    #[inline(always)]
    fn from(variant: SR5_A) -> Self {
        variant as u8 != 0
    }
}
impl SR5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR5_A {
        match self.bits {
            false => SR5_A::NO_ACCESS,
            true => SR5_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR5_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR5_A::ACCESS
    }
}
#[doc = "Field `SR5` writer - Subregion 5 in region n (write '1' to clear)"]
pub type SR5_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR5_A, O>;
impl<'a, const O: u8> SR5_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR5_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR5_A::ACCESS)
    }
}
#[doc = "Field `SR6` reader - Subregion 6 in region n (write '1' to clear)"]
pub type SR6_R = crate::BitReader<SR6_A>;
#[doc = "Subregion 6 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR6_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR6_A> for bool {
    #[inline(always)]
    fn from(variant: SR6_A) -> Self {
        variant as u8 != 0
    }
}
impl SR6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR6_A {
        match self.bits {
            false => SR6_A::NO_ACCESS,
            true => SR6_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR6_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR6_A::ACCESS
    }
}
#[doc = "Field `SR6` writer - Subregion 6 in region n (write '1' to clear)"]
pub type SR6_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR6_A, O>;
impl<'a, const O: u8> SR6_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR6_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR6_A::ACCESS)
    }
}
#[doc = "Field `SR7` reader - Subregion 7 in region n (write '1' to clear)"]
pub type SR7_R = crate::BitReader<SR7_A>;
#[doc = "Subregion 7 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR7_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR7_A> for bool {
    #[inline(always)]
    fn from(variant: SR7_A) -> Self {
        variant as u8 != 0
    }
}
impl SR7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR7_A {
        match self.bits {
            false => SR7_A::NO_ACCESS,
            true => SR7_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR7_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR7_A::ACCESS
    }
}
#[doc = "Field `SR7` writer - Subregion 7 in region n (write '1' to clear)"]
pub type SR7_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR7_A, O>;
impl<'a, const O: u8> SR7_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR7_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR7_A::ACCESS)
    }
}
#[doc = "Field `SR8` reader - Subregion 8 in region n (write '1' to clear)"]
pub type SR8_R = crate::BitReader<SR8_A>;
#[doc = "Subregion 8 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR8_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR8_A> for bool {
    #[inline(always)]
    fn from(variant: SR8_A) -> Self {
        variant as u8 != 0
    }
}
impl SR8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR8_A {
        match self.bits {
            false => SR8_A::NO_ACCESS,
            true => SR8_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR8_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR8_A::ACCESS
    }
}
#[doc = "Field `SR8` writer - Subregion 8 in region n (write '1' to clear)"]
pub type SR8_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR8_A, O>;
impl<'a, const O: u8> SR8_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR8_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR8_A::ACCESS)
    }
}
#[doc = "Field `SR9` reader - Subregion 9 in region n (write '1' to clear)"]
pub type SR9_R = crate::BitReader<SR9_A>;
#[doc = "Subregion 9 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR9_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR9_A> for bool {
    #[inline(always)]
    fn from(variant: SR9_A) -> Self {
        variant as u8 != 0
    }
}
impl SR9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR9_A {
        match self.bits {
            false => SR9_A::NO_ACCESS,
            true => SR9_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR9_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR9_A::ACCESS
    }
}
#[doc = "Field `SR9` writer - Subregion 9 in region n (write '1' to clear)"]
pub type SR9_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR9_A, O>;
impl<'a, const O: u8> SR9_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR9_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR9_A::ACCESS)
    }
}
#[doc = "Field `SR10` reader - Subregion 10 in region n (write '1' to clear)"]
pub type SR10_R = crate::BitReader<SR10_A>;
#[doc = "Subregion 10 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR10_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR10_A> for bool {
    #[inline(always)]
    fn from(variant: SR10_A) -> Self {
        variant as u8 != 0
    }
}
impl SR10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR10_A {
        match self.bits {
            false => SR10_A::NO_ACCESS,
            true => SR10_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR10_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR10_A::ACCESS
    }
}
#[doc = "Field `SR10` writer - Subregion 10 in region n (write '1' to clear)"]
pub type SR10_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR10_A, O>;
impl<'a, const O: u8> SR10_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR10_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR10_A::ACCESS)
    }
}
#[doc = "Field `SR11` reader - Subregion 11 in region n (write '1' to clear)"]
pub type SR11_R = crate::BitReader<SR11_A>;
#[doc = "Subregion 11 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR11_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR11_A> for bool {
    #[inline(always)]
    fn from(variant: SR11_A) -> Self {
        variant as u8 != 0
    }
}
impl SR11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR11_A {
        match self.bits {
            false => SR11_A::NO_ACCESS,
            true => SR11_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR11_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR11_A::ACCESS
    }
}
#[doc = "Field `SR11` writer - Subregion 11 in region n (write '1' to clear)"]
pub type SR11_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR11_A, O>;
impl<'a, const O: u8> SR11_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR11_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR11_A::ACCESS)
    }
}
#[doc = "Field `SR12` reader - Subregion 12 in region n (write '1' to clear)"]
pub type SR12_R = crate::BitReader<SR12_A>;
#[doc = "Subregion 12 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR12_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR12_A> for bool {
    #[inline(always)]
    fn from(variant: SR12_A) -> Self {
        variant as u8 != 0
    }
}
impl SR12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR12_A {
        match self.bits {
            false => SR12_A::NO_ACCESS,
            true => SR12_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR12_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR12_A::ACCESS
    }
}
#[doc = "Field `SR12` writer - Subregion 12 in region n (write '1' to clear)"]
pub type SR12_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR12_A, O>;
impl<'a, const O: u8> SR12_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR12_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR12_A::ACCESS)
    }
}
#[doc = "Field `SR13` reader - Subregion 13 in region n (write '1' to clear)"]
pub type SR13_R = crate::BitReader<SR13_A>;
#[doc = "Subregion 13 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR13_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR13_A> for bool {
    #[inline(always)]
    fn from(variant: SR13_A) -> Self {
        variant as u8 != 0
    }
}
impl SR13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR13_A {
        match self.bits {
            false => SR13_A::NO_ACCESS,
            true => SR13_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR13_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR13_A::ACCESS
    }
}
#[doc = "Field `SR13` writer - Subregion 13 in region n (write '1' to clear)"]
pub type SR13_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR13_A, O>;
impl<'a, const O: u8> SR13_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR13_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR13_A::ACCESS)
    }
}
#[doc = "Field `SR14` reader - Subregion 14 in region n (write '1' to clear)"]
pub type SR14_R = crate::BitReader<SR14_A>;
#[doc = "Subregion 14 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR14_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR14_A> for bool {
    #[inline(always)]
    fn from(variant: SR14_A) -> Self {
        variant as u8 != 0
    }
}
impl SR14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR14_A {
        match self.bits {
            false => SR14_A::NO_ACCESS,
            true => SR14_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR14_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR14_A::ACCESS
    }
}
#[doc = "Field `SR14` writer - Subregion 14 in region n (write '1' to clear)"]
pub type SR14_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR14_A, O>;
impl<'a, const O: u8> SR14_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR14_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR14_A::ACCESS)
    }
}
#[doc = "Field `SR15` reader - Subregion 15 in region n (write '1' to clear)"]
pub type SR15_R = crate::BitReader<SR15_A>;
#[doc = "Subregion 15 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR15_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR15_A> for bool {
    #[inline(always)]
    fn from(variant: SR15_A) -> Self {
        variant as u8 != 0
    }
}
impl SR15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR15_A {
        match self.bits {
            false => SR15_A::NO_ACCESS,
            true => SR15_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR15_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR15_A::ACCESS
    }
}
#[doc = "Field `SR15` writer - Subregion 15 in region n (write '1' to clear)"]
pub type SR15_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR15_A, O>;
impl<'a, const O: u8> SR15_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR15_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR15_A::ACCESS)
    }
}
#[doc = "Field `SR16` reader - Subregion 16 in region n (write '1' to clear)"]
pub type SR16_R = crate::BitReader<SR16_A>;
#[doc = "Subregion 16 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR16_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR16_A> for bool {
    #[inline(always)]
    fn from(variant: SR16_A) -> Self {
        variant as u8 != 0
    }
}
impl SR16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR16_A {
        match self.bits {
            false => SR16_A::NO_ACCESS,
            true => SR16_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR16_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR16_A::ACCESS
    }
}
#[doc = "Field `SR16` writer - Subregion 16 in region n (write '1' to clear)"]
pub type SR16_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR16_A, O>;
impl<'a, const O: u8> SR16_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR16_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR16_A::ACCESS)
    }
}
#[doc = "Field `SR17` reader - Subregion 17 in region n (write '1' to clear)"]
pub type SR17_R = crate::BitReader<SR17_A>;
#[doc = "Subregion 17 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR17_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR17_A> for bool {
    #[inline(always)]
    fn from(variant: SR17_A) -> Self {
        variant as u8 != 0
    }
}
impl SR17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR17_A {
        match self.bits {
            false => SR17_A::NO_ACCESS,
            true => SR17_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR17_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR17_A::ACCESS
    }
}
#[doc = "Field `SR17` writer - Subregion 17 in region n (write '1' to clear)"]
pub type SR17_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR17_A, O>;
impl<'a, const O: u8> SR17_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR17_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR17_A::ACCESS)
    }
}
#[doc = "Field `SR18` reader - Subregion 18 in region n (write '1' to clear)"]
pub type SR18_R = crate::BitReader<SR18_A>;
#[doc = "Subregion 18 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR18_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR18_A> for bool {
    #[inline(always)]
    fn from(variant: SR18_A) -> Self {
        variant as u8 != 0
    }
}
impl SR18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR18_A {
        match self.bits {
            false => SR18_A::NO_ACCESS,
            true => SR18_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR18_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR18_A::ACCESS
    }
}
#[doc = "Field `SR18` writer - Subregion 18 in region n (write '1' to clear)"]
pub type SR18_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR18_A, O>;
impl<'a, const O: u8> SR18_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR18_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR18_A::ACCESS)
    }
}
#[doc = "Field `SR19` reader - Subregion 19 in region n (write '1' to clear)"]
pub type SR19_R = crate::BitReader<SR19_A>;
#[doc = "Subregion 19 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR19_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR19_A> for bool {
    #[inline(always)]
    fn from(variant: SR19_A) -> Self {
        variant as u8 != 0
    }
}
impl SR19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR19_A {
        match self.bits {
            false => SR19_A::NO_ACCESS,
            true => SR19_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR19_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR19_A::ACCESS
    }
}
#[doc = "Field `SR19` writer - Subregion 19 in region n (write '1' to clear)"]
pub type SR19_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR19_A, O>;
impl<'a, const O: u8> SR19_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR19_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR19_A::ACCESS)
    }
}
#[doc = "Field `SR20` reader - Subregion 20 in region n (write '1' to clear)"]
pub type SR20_R = crate::BitReader<SR20_A>;
#[doc = "Subregion 20 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR20_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR20_A> for bool {
    #[inline(always)]
    fn from(variant: SR20_A) -> Self {
        variant as u8 != 0
    }
}
impl SR20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR20_A {
        match self.bits {
            false => SR20_A::NO_ACCESS,
            true => SR20_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR20_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR20_A::ACCESS
    }
}
#[doc = "Field `SR20` writer - Subregion 20 in region n (write '1' to clear)"]
pub type SR20_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR20_A, O>;
impl<'a, const O: u8> SR20_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR20_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR20_A::ACCESS)
    }
}
#[doc = "Field `SR21` reader - Subregion 21 in region n (write '1' to clear)"]
pub type SR21_R = crate::BitReader<SR21_A>;
#[doc = "Subregion 21 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR21_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR21_A> for bool {
    #[inline(always)]
    fn from(variant: SR21_A) -> Self {
        variant as u8 != 0
    }
}
impl SR21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR21_A {
        match self.bits {
            false => SR21_A::NO_ACCESS,
            true => SR21_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR21_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR21_A::ACCESS
    }
}
#[doc = "Field `SR21` writer - Subregion 21 in region n (write '1' to clear)"]
pub type SR21_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR21_A, O>;
impl<'a, const O: u8> SR21_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR21_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR21_A::ACCESS)
    }
}
#[doc = "Field `SR22` reader - Subregion 22 in region n (write '1' to clear)"]
pub type SR22_R = crate::BitReader<SR22_A>;
#[doc = "Subregion 22 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR22_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR22_A> for bool {
    #[inline(always)]
    fn from(variant: SR22_A) -> Self {
        variant as u8 != 0
    }
}
impl SR22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR22_A {
        match self.bits {
            false => SR22_A::NO_ACCESS,
            true => SR22_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR22_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR22_A::ACCESS
    }
}
#[doc = "Field `SR22` writer - Subregion 22 in region n (write '1' to clear)"]
pub type SR22_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR22_A, O>;
impl<'a, const O: u8> SR22_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR22_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR22_A::ACCESS)
    }
}
#[doc = "Field `SR23` reader - Subregion 23 in region n (write '1' to clear)"]
pub type SR23_R = crate::BitReader<SR23_A>;
#[doc = "Subregion 23 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR23_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR23_A> for bool {
    #[inline(always)]
    fn from(variant: SR23_A) -> Self {
        variant as u8 != 0
    }
}
impl SR23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR23_A {
        match self.bits {
            false => SR23_A::NO_ACCESS,
            true => SR23_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR23_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR23_A::ACCESS
    }
}
#[doc = "Field `SR23` writer - Subregion 23 in region n (write '1' to clear)"]
pub type SR23_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR23_A, O>;
impl<'a, const O: u8> SR23_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR23_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR23_A::ACCESS)
    }
}
#[doc = "Field `SR24` reader - Subregion 24 in region n (write '1' to clear)"]
pub type SR24_R = crate::BitReader<SR24_A>;
#[doc = "Subregion 24 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR24_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR24_A> for bool {
    #[inline(always)]
    fn from(variant: SR24_A) -> Self {
        variant as u8 != 0
    }
}
impl SR24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR24_A {
        match self.bits {
            false => SR24_A::NO_ACCESS,
            true => SR24_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR24_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR24_A::ACCESS
    }
}
#[doc = "Field `SR24` writer - Subregion 24 in region n (write '1' to clear)"]
pub type SR24_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR24_A, O>;
impl<'a, const O: u8> SR24_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR24_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR24_A::ACCESS)
    }
}
#[doc = "Field `SR25` reader - Subregion 25 in region n (write '1' to clear)"]
pub type SR25_R = crate::BitReader<SR25_A>;
#[doc = "Subregion 25 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR25_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR25_A> for bool {
    #[inline(always)]
    fn from(variant: SR25_A) -> Self {
        variant as u8 != 0
    }
}
impl SR25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR25_A {
        match self.bits {
            false => SR25_A::NO_ACCESS,
            true => SR25_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR25_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR25_A::ACCESS
    }
}
#[doc = "Field `SR25` writer - Subregion 25 in region n (write '1' to clear)"]
pub type SR25_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR25_A, O>;
impl<'a, const O: u8> SR25_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR25_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR25_A::ACCESS)
    }
}
#[doc = "Field `SR26` reader - Subregion 26 in region n (write '1' to clear)"]
pub type SR26_R = crate::BitReader<SR26_A>;
#[doc = "Subregion 26 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR26_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR26_A> for bool {
    #[inline(always)]
    fn from(variant: SR26_A) -> Self {
        variant as u8 != 0
    }
}
impl SR26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR26_A {
        match self.bits {
            false => SR26_A::NO_ACCESS,
            true => SR26_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR26_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR26_A::ACCESS
    }
}
#[doc = "Field `SR26` writer - Subregion 26 in region n (write '1' to clear)"]
pub type SR26_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR26_A, O>;
impl<'a, const O: u8> SR26_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR26_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR26_A::ACCESS)
    }
}
#[doc = "Field `SR27` reader - Subregion 27 in region n (write '1' to clear)"]
pub type SR27_R = crate::BitReader<SR27_A>;
#[doc = "Subregion 27 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR27_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR27_A> for bool {
    #[inline(always)]
    fn from(variant: SR27_A) -> Self {
        variant as u8 != 0
    }
}
impl SR27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR27_A {
        match self.bits {
            false => SR27_A::NO_ACCESS,
            true => SR27_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR27_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR27_A::ACCESS
    }
}
#[doc = "Field `SR27` writer - Subregion 27 in region n (write '1' to clear)"]
pub type SR27_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR27_A, O>;
impl<'a, const O: u8> SR27_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR27_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR27_A::ACCESS)
    }
}
#[doc = "Field `SR28` reader - Subregion 28 in region n (write '1' to clear)"]
pub type SR28_R = crate::BitReader<SR28_A>;
#[doc = "Subregion 28 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR28_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR28_A> for bool {
    #[inline(always)]
    fn from(variant: SR28_A) -> Self {
        variant as u8 != 0
    }
}
impl SR28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR28_A {
        match self.bits {
            false => SR28_A::NO_ACCESS,
            true => SR28_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR28_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR28_A::ACCESS
    }
}
#[doc = "Field `SR28` writer - Subregion 28 in region n (write '1' to clear)"]
pub type SR28_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR28_A, O>;
impl<'a, const O: u8> SR28_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR28_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR28_A::ACCESS)
    }
}
#[doc = "Field `SR29` reader - Subregion 29 in region n (write '1' to clear)"]
pub type SR29_R = crate::BitReader<SR29_A>;
#[doc = "Subregion 29 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR29_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR29_A> for bool {
    #[inline(always)]
    fn from(variant: SR29_A) -> Self {
        variant as u8 != 0
    }
}
impl SR29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR29_A {
        match self.bits {
            false => SR29_A::NO_ACCESS,
            true => SR29_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR29_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR29_A::ACCESS
    }
}
#[doc = "Field `SR29` writer - Subregion 29 in region n (write '1' to clear)"]
pub type SR29_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR29_A, O>;
impl<'a, const O: u8> SR29_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR29_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR29_A::ACCESS)
    }
}
#[doc = "Field `SR30` reader - Subregion 30 in region n (write '1' to clear)"]
pub type SR30_R = crate::BitReader<SR30_A>;
#[doc = "Subregion 30 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR30_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR30_A> for bool {
    #[inline(always)]
    fn from(variant: SR30_A) -> Self {
        variant as u8 != 0
    }
}
impl SR30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR30_A {
        match self.bits {
            false => SR30_A::NO_ACCESS,
            true => SR30_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR30_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR30_A::ACCESS
    }
}
#[doc = "Field `SR30` writer - Subregion 30 in region n (write '1' to clear)"]
pub type SR30_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR30_A, O>;
impl<'a, const O: u8> SR30_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR30_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR30_A::ACCESS)
    }
}
#[doc = "Field `SR31` reader - Subregion 31 in region n (write '1' to clear)"]
pub type SR31_R = crate::BitReader<SR31_A>;
#[doc = "Subregion 31 in region n (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR31_A {
    #[doc = "0: No read access occurred in this subregion"]
    NO_ACCESS = 0,
    #[doc = "1: Read access(es) occurred in this subregion"]
    ACCESS = 1,
}
impl From<SR31_A> for bool {
    #[inline(always)]
    fn from(variant: SR31_A) -> Self {
        variant as u8 != 0
    }
}
impl SR31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR31_A {
        match self.bits {
            false => SR31_A::NO_ACCESS,
            true => SR31_A::ACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ACCESS`"]
    #[inline(always)]
    pub fn is_no_access(&self) -> bool {
        *self == SR31_A::NO_ACCESS
    }
    #[doc = "Checks if the value of the field is `ACCESS`"]
    #[inline(always)]
    pub fn is_access(&self) -> bool {
        *self == SR31_A::ACCESS
    }
}
#[doc = "Field `SR31` writer - Subregion 31 in region n (write '1' to clear)"]
pub type SR31_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SUBSTATRA_SPEC, SR31_A, O>;
impl<'a, const O: u8> SR31_W<'a, O> {
    #[doc = "No read access occurred in this subregion"]
    #[inline(always)]
    pub fn no_access(self) -> &'a mut W {
        self.variant(SR31_A::NO_ACCESS)
    }
    #[doc = "Read access(es) occurred in this subregion"]
    #[inline(always)]
    pub fn access(self) -> &'a mut W {
        self.variant(SR31_A::ACCESS)
    }
}
impl R {
    #[doc = "Bit 0 - Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr0(&self) -> SR0_R {
        SR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Subregion 1 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr1(&self) -> SR1_R {
        SR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Subregion 2 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr2(&self) -> SR2_R {
        SR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Subregion 3 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr3(&self) -> SR3_R {
        SR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Subregion 4 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr4(&self) -> SR4_R {
        SR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Subregion 5 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr5(&self) -> SR5_R {
        SR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Subregion 6 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr6(&self) -> SR6_R {
        SR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Subregion 7 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr7(&self) -> SR7_R {
        SR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Subregion 8 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr8(&self) -> SR8_R {
        SR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Subregion 9 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr9(&self) -> SR9_R {
        SR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Subregion 10 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr10(&self) -> SR10_R {
        SR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Subregion 11 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr11(&self) -> SR11_R {
        SR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Subregion 12 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr12(&self) -> SR12_R {
        SR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Subregion 13 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr13(&self) -> SR13_R {
        SR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Subregion 14 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr14(&self) -> SR14_R {
        SR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Subregion 15 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr15(&self) -> SR15_R {
        SR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Subregion 16 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr16(&self) -> SR16_R {
        SR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Subregion 17 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr17(&self) -> SR17_R {
        SR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Subregion 18 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr18(&self) -> SR18_R {
        SR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Subregion 19 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr19(&self) -> SR19_R {
        SR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Subregion 20 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr20(&self) -> SR20_R {
        SR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Subregion 21 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr21(&self) -> SR21_R {
        SR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Subregion 22 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr22(&self) -> SR22_R {
        SR22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Subregion 23 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr23(&self) -> SR23_R {
        SR23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Subregion 24 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr24(&self) -> SR24_R {
        SR24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Subregion 25 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr25(&self) -> SR25_R {
        SR25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Subregion 26 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr26(&self) -> SR26_R {
        SR26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Subregion 27 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr27(&self) -> SR27_R {
        SR27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Subregion 28 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr28(&self) -> SR28_R {
        SR28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Subregion 29 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr29(&self) -> SR29_R {
        SR29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Subregion 30 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr30(&self) -> SR30_R {
        SR30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Subregion 31 in region n (write '1' to clear)"]
    #[inline(always)]
    pub fn sr31(&self) -> SR31_R {
        SR31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Subregion 0 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr0(&mut self) -> SR0_W<0> {
        SR0_W::new(self)
    }
    #[doc = "Bit 1 - Subregion 1 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr1(&mut self) -> SR1_W<1> {
        SR1_W::new(self)
    }
    #[doc = "Bit 2 - Subregion 2 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr2(&mut self) -> SR2_W<2> {
        SR2_W::new(self)
    }
    #[doc = "Bit 3 - Subregion 3 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr3(&mut self) -> SR3_W<3> {
        SR3_W::new(self)
    }
    #[doc = "Bit 4 - Subregion 4 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr4(&mut self) -> SR4_W<4> {
        SR4_W::new(self)
    }
    #[doc = "Bit 5 - Subregion 5 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr5(&mut self) -> SR5_W<5> {
        SR5_W::new(self)
    }
    #[doc = "Bit 6 - Subregion 6 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr6(&mut self) -> SR6_W<6> {
        SR6_W::new(self)
    }
    #[doc = "Bit 7 - Subregion 7 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr7(&mut self) -> SR7_W<7> {
        SR7_W::new(self)
    }
    #[doc = "Bit 8 - Subregion 8 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr8(&mut self) -> SR8_W<8> {
        SR8_W::new(self)
    }
    #[doc = "Bit 9 - Subregion 9 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr9(&mut self) -> SR9_W<9> {
        SR9_W::new(self)
    }
    #[doc = "Bit 10 - Subregion 10 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr10(&mut self) -> SR10_W<10> {
        SR10_W::new(self)
    }
    #[doc = "Bit 11 - Subregion 11 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr11(&mut self) -> SR11_W<11> {
        SR11_W::new(self)
    }
    #[doc = "Bit 12 - Subregion 12 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr12(&mut self) -> SR12_W<12> {
        SR12_W::new(self)
    }
    #[doc = "Bit 13 - Subregion 13 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr13(&mut self) -> SR13_W<13> {
        SR13_W::new(self)
    }
    #[doc = "Bit 14 - Subregion 14 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr14(&mut self) -> SR14_W<14> {
        SR14_W::new(self)
    }
    #[doc = "Bit 15 - Subregion 15 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr15(&mut self) -> SR15_W<15> {
        SR15_W::new(self)
    }
    #[doc = "Bit 16 - Subregion 16 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr16(&mut self) -> SR16_W<16> {
        SR16_W::new(self)
    }
    #[doc = "Bit 17 - Subregion 17 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr17(&mut self) -> SR17_W<17> {
        SR17_W::new(self)
    }
    #[doc = "Bit 18 - Subregion 18 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr18(&mut self) -> SR18_W<18> {
        SR18_W::new(self)
    }
    #[doc = "Bit 19 - Subregion 19 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr19(&mut self) -> SR19_W<19> {
        SR19_W::new(self)
    }
    #[doc = "Bit 20 - Subregion 20 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr20(&mut self) -> SR20_W<20> {
        SR20_W::new(self)
    }
    #[doc = "Bit 21 - Subregion 21 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr21(&mut self) -> SR21_W<21> {
        SR21_W::new(self)
    }
    #[doc = "Bit 22 - Subregion 22 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr22(&mut self) -> SR22_W<22> {
        SR22_W::new(self)
    }
    #[doc = "Bit 23 - Subregion 23 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr23(&mut self) -> SR23_W<23> {
        SR23_W::new(self)
    }
    #[doc = "Bit 24 - Subregion 24 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr24(&mut self) -> SR24_W<24> {
        SR24_W::new(self)
    }
    #[doc = "Bit 25 - Subregion 25 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr25(&mut self) -> SR25_W<25> {
        SR25_W::new(self)
    }
    #[doc = "Bit 26 - Subregion 26 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr26(&mut self) -> SR26_W<26> {
        SR26_W::new(self)
    }
    #[doc = "Bit 27 - Subregion 27 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr27(&mut self) -> SR27_W<27> {
        SR27_W::new(self)
    }
    #[doc = "Bit 28 - Subregion 28 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr28(&mut self) -> SR28_W<28> {
        SR28_W::new(self)
    }
    #[doc = "Bit 29 - Subregion 29 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr29(&mut self) -> SR29_W<29> {
        SR29_W::new(self)
    }
    #[doc = "Bit 30 - Subregion 30 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr30(&mut self) -> SR30_W<30> {
        SR30_W::new(self)
    }
    #[doc = "Bit 31 - Subregion 31 in region n (write '1' to clear)"]
    #[inline(always)]
    #[must_use]
    pub fn sr31(&mut self) -> SR31_W<31> {
        SR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Source of event/interrupt in region n, read access detected while corresponding subregion was enabled for watching\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [substatra](index.html) module"]
pub struct SUBSTATRA_SPEC;
impl crate::RegisterSpec for SUBSTATRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [substatra::R](R) reader structure"]
impl crate::Readable for SUBSTATRA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [substatra::W](W) writer structure"]
impl crate::Writable for SUBSTATRA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets SUBSTATRA to value 0"]
impl crate::Resettable for SUBSTATRA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
