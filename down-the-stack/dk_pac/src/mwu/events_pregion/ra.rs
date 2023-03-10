#[doc = "Register `RA` reader"]
pub struct R(crate::R<RA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RA` writer"]
pub struct W(crate::W<RA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RA_SPEC>;
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
impl From<crate::W<RA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Read access to peripheral region n detected"]
pub type RA_R = crate::BitReader<RA_A>;
#[doc = "Read access to peripheral region n detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RA_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<RA_A> for bool {
    #[inline(always)]
    fn from(variant: RA_A) -> Self {
        variant as u8 != 0
    }
}
impl RA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RA_A {
        match self.bits {
            false => RA_A::NOT_GENERATED,
            true => RA_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == RA_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == RA_A::GENERATED
    }
}
#[doc = "Field `RA` writer - Read access to peripheral region n detected"]
pub type RA_W<'a, const O: u8> = crate::BitWriter<'a, u32, RA_SPEC, RA_A, O>;
impl<'a, const O: u8> RA_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(RA_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(RA_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Read access to peripheral region n detected"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read access to peripheral region n detected"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<0> {
        RA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Read access to peripheral region n detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra](index.html) module"]
pub struct RA_SPEC;
impl crate::RegisterSpec for RA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ra::R](R) reader structure"]
impl crate::Readable for RA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ra::W](W) writer structure"]
impl crate::Writable for RA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RA to value 0"]
impl crate::Resettable for RA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
