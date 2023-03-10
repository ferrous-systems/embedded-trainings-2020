#[doc = "Register `ALIGN` reader"]
pub struct R(crate::R<ALIGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALIGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALIGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALIGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALIGN` writer"]
pub struct W(crate::W<ALIGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALIGN_SPEC>;
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
impl From<crate::W<ALIGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALIGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALIGN` reader - Alignment of sample within a frame."]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
#[doc = "Alignment of sample within a frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    #[doc = "0: Left-aligned."]
    LEFT = 0,
    #[doc = "1: Right-aligned."]
    RIGHT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::LEFT,
            true => ALIGN_A::RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::RIGHT
    }
}
#[doc = "Field `ALIGN` writer - Alignment of sample within a frame."]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALIGN_SPEC, ALIGN_A, O>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    #[doc = "Left-aligned."]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
    }
    #[doc = "Right-aligned."]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
    }
}
impl R {
    #[doc = "Bit 0 - Alignment of sample within a frame."]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alignment of sample within a frame."]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<0> {
        ALIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Alignment of sample within a frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [align](index.html) module"]
pub struct ALIGN_SPEC;
impl crate::RegisterSpec for ALIGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [align::R](R) reader structure"]
impl crate::Readable for ALIGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [align::W](W) writer structure"]
impl crate::Writable for ALIGN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALIGN to value 0"]
impl crate::Resettable for ALIGN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
