#[doc = "Register `ANADETECT` reader"]
pub struct R(crate::R<ANADETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANADETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANADETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANADETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANADETECT` writer"]
pub struct W(crate::W<ANADETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANADETECT_SPEC>;
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
impl From<crate::W<ANADETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANADETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANADETECT` reader - Analog detect configuration"]
pub type ANADETECT_R = crate::FieldReader<u8, ANADETECT_A>;
#[doc = "Analog detect configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ANADETECT_A {
    #[doc = "0: Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    CROSS = 0,
    #[doc = "1: Generate ANADETECT on upward crossing only"]
    UP = 1,
    #[doc = "2: Generate ANADETECT on downward crossing only"]
    DOWN = 2,
}
impl From<ANADETECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ANADETECT_A) -> Self {
        variant as _
    }
}
impl ANADETECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ANADETECT_A> {
        match self.bits {
            0 => Some(ANADETECT_A::CROSS),
            1 => Some(ANADETECT_A::UP),
            2 => Some(ANADETECT_A::DOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CROSS`"]
    #[inline(always)]
    pub fn is_cross(&self) -> bool {
        *self == ANADETECT_A::CROSS
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == ANADETECT_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == ANADETECT_A::DOWN
    }
}
#[doc = "Field `ANADETECT` writer - Analog detect configuration"]
pub type ANADETECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANADETECT_SPEC, u8, ANADETECT_A, 2, O>;
impl<'a, const O: u8> ANADETECT_W<'a, O> {
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    #[inline(always)]
    pub fn cross(self) -> &'a mut W {
        self.variant(ANADETECT_A::CROSS)
    }
    #[doc = "Generate ANADETECT on upward crossing only"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(ANADETECT_A::UP)
    }
    #[doc = "Generate ANADETECT on downward crossing only"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(ANADETECT_A::DOWN)
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(&self) -> ANADETECT_R {
        ANADETECT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    #[must_use]
    pub fn anadetect(&mut self) -> ANADETECT_W<0> {
        ANADETECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog detect configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anadetect](index.html) module"]
pub struct ANADETECT_SPEC;
impl crate::RegisterSpec for ANADETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anadetect::R](R) reader structure"]
impl crate::Readable for ANADETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anadetect::W](W) writer structure"]
impl crate::Writable for ANADETECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANADETECT to value 0"]
impl crate::Resettable for ANADETECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
