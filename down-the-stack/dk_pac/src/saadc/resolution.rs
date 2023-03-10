#[doc = "Register `RESOLUTION` reader"]
pub struct R(crate::R<RESOLUTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESOLUTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESOLUTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESOLUTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESOLUTION` writer"]
pub struct W(crate::W<RESOLUTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESOLUTION_SPEC>;
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
impl From<crate::W<RESOLUTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESOLUTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - Set the resolution"]
pub type VAL_R = crate::FieldReader<u8, VAL_A>;
#[doc = "Set the resolution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VAL_A {
    #[doc = "0: 8 bit"]
    _8BIT = 0,
    #[doc = "1: 10 bit"]
    _10BIT = 1,
    #[doc = "2: 12 bit"]
    _12BIT = 2,
    #[doc = "3: 14 bit"]
    _14BIT = 3,
}
impl From<VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        variant as _
    }
}
impl VAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VAL_A> {
        match self.bits {
            0 => Some(VAL_A::_8BIT),
            1 => Some(VAL_A::_10BIT),
            2 => Some(VAL_A::_12BIT),
            3 => Some(VAL_A::_14BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == VAL_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == VAL_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == VAL_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_14BIT`"]
    #[inline(always)]
    pub fn is_14bit(&self) -> bool {
        *self == VAL_A::_14BIT
    }
}
#[doc = "Field `VAL` writer - Set the resolution"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RESOLUTION_SPEC, u8, VAL_A, 3, O>;
impl<'a, const O: u8> VAL_W<'a, O> {
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(VAL_A::_8BIT)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(VAL_A::_10BIT)
    }
    #[doc = "12 bit"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(VAL_A::_12BIT)
    }
    #[doc = "14 bit"]
    #[inline(always)]
    pub fn _14bit(self) -> &'a mut W {
        self.variant(VAL_A::_14BIT)
    }
}
impl R {
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Resolution configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resolution](index.html) module"]
pub struct RESOLUTION_SPEC;
impl crate::RegisterSpec for RESOLUTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resolution::R](R) reader structure"]
impl crate::Readable for RESOLUTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resolution::W](W) writer structure"]
impl crate::Writable for RESOLUTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESOLUTION to value 0x01"]
impl crate::Resettable for RESOLUTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
