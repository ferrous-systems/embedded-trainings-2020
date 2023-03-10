#[doc = "Register `ICACHECNF` reader"]
pub struct R(crate::R<ICACHECNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHECNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHECNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHECNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHECNF` writer"]
pub struct W(crate::W<ICACHECNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHECNF_SPEC>;
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
impl From<crate::W<ICACHECNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHECNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHEEN` reader - Cache enable"]
pub type CACHEEN_R = crate::BitReader<CACHEEN_A>;
#[doc = "Cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHEEN_A {
    #[doc = "0: Disable cache. Invalidates all cache entries."]
    DISABLED = 0,
    #[doc = "1: Enable cache"]
    ENABLED = 1,
}
impl From<CACHEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEEN_A {
        match self.bits {
            false => CACHEEN_A::DISABLED,
            true => CACHEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CACHEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CACHEEN_A::ENABLED
    }
}
#[doc = "Field `CACHEEN` writer - Cache enable"]
pub type CACHEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHECNF_SPEC, CACHEEN_A, O>;
impl<'a, const O: u8> CACHEEN_W<'a, O> {
    #[doc = "Disable cache. Invalidates all cache entries."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::DISABLED)
    }
    #[doc = "Enable cache"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::ENABLED)
    }
}
#[doc = "Field `CACHEPROFEN` reader - Cache profiling enable"]
pub type CACHEPROFEN_R = crate::BitReader<CACHEPROFEN_A>;
#[doc = "Cache profiling enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHEPROFEN_A {
    #[doc = "0: Disable cache profiling"]
    DISABLED = 0,
    #[doc = "1: Enable cache profiling"]
    ENABLED = 1,
}
impl From<CACHEPROFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEPROFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHEPROFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEPROFEN_A {
        match self.bits {
            false => CACHEPROFEN_A::DISABLED,
            true => CACHEPROFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CACHEPROFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CACHEPROFEN_A::ENABLED
    }
}
#[doc = "Field `CACHEPROFEN` writer - Cache profiling enable"]
pub type CACHEPROFEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ICACHECNF_SPEC, CACHEPROFEN_A, O>;
impl<'a, const O: u8> CACHEPROFEN_W<'a, O> {
    #[doc = "Disable cache profiling"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEPROFEN_A::DISABLED)
    }
    #[doc = "Enable cache profiling"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEPROFEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn cacheen(&self) -> CACHEEN_R {
        CACHEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    pub fn cacheprofen(&self) -> CACHEPROFEN_R {
        CACHEPROFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn cacheen(&mut self) -> CACHEEN_W<0> {
        CACHEEN_W::new(self)
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    #[must_use]
    pub fn cacheprofen(&mut self) -> CACHEPROFEN_W<8> {
        CACHEPROFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I-Code cache configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icachecnf](index.html) module"]
pub struct ICACHECNF_SPEC;
impl crate::RegisterSpec for ICACHECNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icachecnf::R](R) reader structure"]
impl crate::Readable for ICACHECNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icachecnf::W](W) writer structure"]
impl crate::Writable for ICACHECNF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHECNF to value 0"]
impl crate::Resettable for ICACHECNF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
