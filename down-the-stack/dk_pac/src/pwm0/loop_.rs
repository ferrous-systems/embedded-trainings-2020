#[doc = "Register `LOOP` reader"]
pub struct R(crate::R<LOOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOOP` writer"]
pub struct W(crate::W<LOOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOOP_SPEC>;
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
impl From<crate::W<LOOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - Amount of playback of pattern cycles"]
pub type CNT_R = crate::FieldReader<u16, CNT_A>;
#[doc = "Amount of playback of pattern cycles\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CNT_A {
    #[doc = "0: Looping disabled (stop at the end of the sequence)"]
    DISABLED = 0,
}
impl From<CNT_A> for u16 {
    #[inline(always)]
    fn from(variant: CNT_A) -> Self {
        variant as _
    }
}
impl CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CNT_A> {
        match self.bits {
            0 => Some(CNT_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CNT_A::DISABLED
    }
}
#[doc = "Field `CNT` writer - Amount of playback of pattern cycles"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOOP_SPEC, u16, CNT_A, 16, O>;
impl<'a, const O: u8> CNT_W<'a, O> {
    #[doc = "Looping disabled (stop at the end of the sequence)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNT_A::DISABLED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Amount of playback of pattern cycles"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Amount of playback of pattern cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amount of playback of a loop\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loop_](index.html) module"]
pub struct LOOP_SPEC;
impl crate::RegisterSpec for LOOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loop_::R](R) reader structure"]
impl crate::Readable for LOOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loop_::W](W) writer structure"]
impl crate::Writable for LOOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOOP to value 0"]
impl crate::Resettable for LOOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
