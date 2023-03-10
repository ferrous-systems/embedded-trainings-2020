#[doc = "Register `CHANNELS` reader"]
pub struct R(crate::R<CHANNELS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNELS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNELS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNELS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNELS` writer"]
pub struct W(crate::W<CHANNELS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNELS_SPEC>;
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
impl From<crate::W<CHANNELS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNELS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHANNELS` reader - Enable channels."]
pub type CHANNELS_R = crate::FieldReader<u8, CHANNELS_A>;
#[doc = "Enable channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHANNELS_A {
    #[doc = "0: Stereo."]
    STEREO = 0,
    #[doc = "1: Left only."]
    LEFT = 1,
    #[doc = "2: Right only."]
    RIGHT = 2,
}
impl From<CHANNELS_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNELS_A) -> Self {
        variant as _
    }
}
impl CHANNELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHANNELS_A> {
        match self.bits {
            0 => Some(CHANNELS_A::STEREO),
            1 => Some(CHANNELS_A::LEFT),
            2 => Some(CHANNELS_A::RIGHT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == CHANNELS_A::STEREO
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == CHANNELS_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == CHANNELS_A::RIGHT
    }
}
#[doc = "Field `CHANNELS` writer - Enable channels."]
pub type CHANNELS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNELS_SPEC, u8, CHANNELS_A, 2, O>;
impl<'a, const O: u8> CHANNELS_W<'a, O> {
    #[doc = "Stereo."]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(CHANNELS_A::STEREO)
    }
    #[doc = "Left only."]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(CHANNELS_A::LEFT)
    }
    #[doc = "Right only."]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(CHANNELS_A::RIGHT)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline(always)]
    #[must_use]
    pub fn channels(&mut self) -> CHANNELS_W<0> {
        CHANNELS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channels](index.html) module"]
pub struct CHANNELS_SPEC;
impl crate::RegisterSpec for CHANNELS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channels::R](R) reader structure"]
impl crate::Readable for CHANNELS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channels::W](W) writer structure"]
impl crate::Writable for CHANNELS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNELS to value 0"]
impl crate::Resettable for CHANNELS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
