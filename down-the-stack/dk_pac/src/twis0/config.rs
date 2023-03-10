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
#[doc = "Field `ADDRESS0` reader - Enable or disable address matching on ADDRESS\\[0\\]"]
pub type ADDRESS0_R = crate::BitReader<ADDRESS0_A>;
#[doc = "Enable or disable address matching on ADDRESS\\[0\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRESS0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADDRESS0_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRESS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS0_A {
        match self.bits {
            false => ADDRESS0_A::DISABLED,
            true => ADDRESS0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS0_A::ENABLED
    }
}
#[doc = "Field `ADDRESS0` writer - Enable or disable address matching on ADDRESS\\[0\\]"]
pub type ADDRESS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, ADDRESS0_A, O>;
impl<'a, const O: u8> ADDRESS0_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS0_A::ENABLED)
    }
}
#[doc = "Field `ADDRESS1` reader - Enable or disable address matching on ADDRESS\\[1\\]"]
pub type ADDRESS1_R = crate::BitReader<ADDRESS1_A>;
#[doc = "Enable or disable address matching on ADDRESS\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRESS1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADDRESS1_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRESS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS1_A {
        match self.bits {
            false => ADDRESS1_A::DISABLED,
            true => ADDRESS1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS1_A::ENABLED
    }
}
#[doc = "Field `ADDRESS1` writer - Enable or disable address matching on ADDRESS\\[1\\]"]
pub type ADDRESS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, ADDRESS1_A, O>;
impl<'a, const O: u8> ADDRESS1_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS1_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    pub fn address0(&self) -> ADDRESS0_R {
        ADDRESS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    pub fn address1(&self) -> ADDRESS1_R {
        ADDRESS1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn address0(&mut self) -> ADDRESS0_W<0> {
        ADDRESS0_W::new(self)
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn address1(&mut self) -> ADDRESS1_W<1> {
        ADDRESS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for the address match mechanism\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
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
#[doc = "`reset()` method sets CONFIG to value 0x01"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
