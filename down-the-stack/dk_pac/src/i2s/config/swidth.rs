#[doc = "Register `SWIDTH` reader"]
pub struct R(crate::R<SWIDTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIDTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIDTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIDTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIDTH` writer"]
pub struct W(crate::W<SWIDTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIDTH_SPEC>;
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
impl From<crate::W<SWIDTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIDTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWIDTH` reader - Sample width."]
pub type SWIDTH_R = crate::FieldReader<u8, SWIDTH_A>;
#[doc = "Sample width.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWIDTH_A {
    #[doc = "0: 8 bit."]
    _8BIT = 0,
    #[doc = "1: 16 bit."]
    _16BIT = 1,
    #[doc = "2: 24 bit."]
    _24BIT = 2,
}
impl From<SWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SWIDTH_A) -> Self {
        variant as _
    }
}
impl SWIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWIDTH_A> {
        match self.bits {
            0 => Some(SWIDTH_A::_8BIT),
            1 => Some(SWIDTH_A::_16BIT),
            2 => Some(SWIDTH_A::_24BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == SWIDTH_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == SWIDTH_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == SWIDTH_A::_24BIT
    }
}
#[doc = "Field `SWIDTH` writer - Sample width."]
pub type SWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWIDTH_SPEC, u8, SWIDTH_A, 2, O>;
impl<'a, const O: u8> SWIDTH_W<'a, O> {
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_8BIT)
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_16BIT)
    }
    #[doc = "24 bit."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_24BIT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sample width."]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample width."]
    #[inline(always)]
    #[must_use]
    pub fn swidth(&mut self) -> SWIDTH_W<0> {
        SWIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample width.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swidth](index.html) module"]
pub struct SWIDTH_SPEC;
impl crate::RegisterSpec for SWIDTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swidth::R](R) reader structure"]
impl crate::Readable for SWIDTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swidth::W](W) writer structure"]
impl crate::Writable for SWIDTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWIDTH to value 0x01"]
impl crate::Resettable for SWIDTH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
