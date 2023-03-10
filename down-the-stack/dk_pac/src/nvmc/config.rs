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
#[doc = "Field `WEN` reader - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
pub type WEN_R = crate::FieldReader<u8, WEN_A>;
#[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WEN_A {
    #[doc = "0: Read only access"]
    REN = 0,
    #[doc = "1: Write Enabled"]
    WEN = 1,
    #[doc = "2: Erase enabled"]
    EEN = 2,
}
impl From<WEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WEN_A) -> Self {
        variant as _
    }
}
impl WEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WEN_A> {
        match self.bits {
            0 => Some(WEN_A::REN),
            1 => Some(WEN_A::WEN),
            2 => Some(WEN_A::EEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REN`"]
    #[inline(always)]
    pub fn is_ren(&self) -> bool {
        *self == WEN_A::REN
    }
    #[doc = "Checks if the value of the field is `WEN`"]
    #[inline(always)]
    pub fn is_wen(&self) -> bool {
        *self == WEN_A::WEN
    }
    #[doc = "Checks if the value of the field is `EEN`"]
    #[inline(always)]
    pub fn is_een(&self) -> bool {
        *self == WEN_A::EEN
    }
}
#[doc = "Field `WEN` writer - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
pub type WEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, WEN_A, 2, O>;
impl<'a, const O: u8> WEN_W<'a, O> {
    #[doc = "Read only access"]
    #[inline(always)]
    pub fn ren(self) -> &'a mut W {
        self.variant(WEN_A::REN)
    }
    #[doc = "Write Enabled"]
    #[inline(always)]
    pub fn wen(self) -> &'a mut W {
        self.variant(WEN_A::WEN)
    }
    #[doc = "Erase enabled"]
    #[inline(always)]
    pub fn een(self) -> &'a mut W {
        self.variant(WEN_A::EEN)
    }
}
impl R {
    #[doc = "Bits 0:1 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WEN_W<0> {
        WEN_W::new(self)
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
