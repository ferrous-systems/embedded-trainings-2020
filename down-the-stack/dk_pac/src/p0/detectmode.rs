#[doc = "Register `DETECTMODE` reader"]
pub struct R(crate::R<DETECTMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DETECTMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DETECTMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DETECTMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DETECTMODE` writer"]
pub struct W(crate::W<DETECTMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DETECTMODE_SPEC>;
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
impl From<crate::W<DETECTMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DETECTMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DETECTMODE` reader - Select between default DETECT signal behaviour and LDETECT mode"]
pub type DETECTMODE_R = crate::BitReader<DETECTMODE_A>;
#[doc = "Select between default DETECT signal behaviour and LDETECT mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DETECTMODE_A {
    #[doc = "0: DETECT directly connected to PIN DETECT signals"]
    DEFAULT = 0,
    #[doc = "1: Use the latched LDETECT behaviour"]
    LDETECT = 1,
}
impl From<DETECTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DETECTMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DETECTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DETECTMODE_A {
        match self.bits {
            false => DETECTMODE_A::DEFAULT,
            true => DETECTMODE_A::LDETECT,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == DETECTMODE_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LDETECT`"]
    #[inline(always)]
    pub fn is_ldetect(&self) -> bool {
        *self == DETECTMODE_A::LDETECT
    }
}
#[doc = "Field `DETECTMODE` writer - Select between default DETECT signal behaviour and LDETECT mode"]
pub type DETECTMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DETECTMODE_SPEC, DETECTMODE_A, O>;
impl<'a, const O: u8> DETECTMODE_W<'a, O> {
    #[doc = "DETECT directly connected to PIN DETECT signals"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(DETECTMODE_A::DEFAULT)
    }
    #[doc = "Use the latched LDETECT behaviour"]
    #[inline(always)]
    pub fn ldetect(self) -> &'a mut W {
        self.variant(DETECTMODE_A::LDETECT)
    }
}
impl R {
    #[doc = "Bit 0 - Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    pub fn detectmode(&self) -> DETECTMODE_R {
        DETECTMODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select between default DETECT signal behaviour and LDETECT mode"]
    #[inline(always)]
    #[must_use]
    pub fn detectmode(&mut self) -> DETECTMODE_W<0> {
        DETECTMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Select between default DETECT signal behaviour and LDETECT mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [detectmode](index.html) module"]
pub struct DETECTMODE_SPEC;
impl crate::RegisterSpec for DETECTMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [detectmode::R](R) reader structure"]
impl crate::Readable for DETECTMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [detectmode::W](W) writer structure"]
impl crate::Writable for DETECTMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DETECTMODE to value 0"]
impl crate::Resettable for DETECTMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
