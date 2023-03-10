#[doc = "Register `FORMAT` reader"]
pub struct R(crate::R<FORMAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORMAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORMAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORMAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FORMAT` writer"]
pub struct W(crate::W<FORMAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORMAT_SPEC>;
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
impl From<crate::W<FORMAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORMAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - Frame format."]
pub type FORMAT_R = crate::BitReader<FORMAT_A>;
#[doc = "Frame format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORMAT_A {
    #[doc = "0: Original I2S format."]
    I2S = 0,
    #[doc = "1: Alternate (left- or right-aligned) format."]
    ALIGNED = 1,
}
impl From<FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            false => FORMAT_A::I2S,
            true => FORMAT_A::ALIGNED,
        }
    }
    #[doc = "Checks if the value of the field is `I2S`"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == FORMAT_A::I2S
    }
    #[doc = "Checks if the value of the field is `ALIGNED`"]
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        *self == FORMAT_A::ALIGNED
    }
}
#[doc = "Field `FORMAT` writer - Frame format."]
pub type FORMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FORMAT_SPEC, FORMAT_A, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "Original I2S format."]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut W {
        self.variant(FORMAT_A::I2S)
    }
    #[doc = "Alternate (left- or right-aligned) format."]
    #[inline(always)]
    pub fn aligned(self) -> &'a mut W {
        self.variant(FORMAT_A::ALIGNED)
    }
}
impl R {
    #[doc = "Bit 0 - Frame format."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame format."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<0> {
        FORMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame format.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [format](index.html) module"]
pub struct FORMAT_SPEC;
impl crate::RegisterSpec for FORMAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [format::R](R) reader structure"]
impl crate::Readable for FORMAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [format::W](W) writer structure"]
impl crate::Writable for FORMAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORMAT to value 0"]
impl crate::Resettable for FORMAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
