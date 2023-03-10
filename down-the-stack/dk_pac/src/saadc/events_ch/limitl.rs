#[doc = "Register `LIMITL` reader"]
pub struct R(crate::R<LIMITL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMITL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMITL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMITL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMITL` writer"]
pub struct W(crate::W<LIMITL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMITL_SPEC>;
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
impl From<crate::W<LIMITL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMITL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIMITL` reader - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
pub type LIMITL_R = crate::BitReader<LIMITL_A>;
#[doc = "Last results is equal or below CH\\[n\\].LIMIT.LOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LIMITL_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
impl LIMITL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LIMITL_A {
        match self.bits {
            false => LIMITL_A::NOT_GENERATED,
            true => LIMITL_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == LIMITL_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == LIMITL_A::GENERATED
    }
}
#[doc = "Field `LIMITL` writer - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
pub type LIMITL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LIMITL_SPEC, LIMITL_A, O>;
impl<'a, const O: u8> LIMITL_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(LIMITL_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(LIMITL_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    pub fn limitl(&self) -> LIMITL_R {
        LIMITL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Last results is equal or below CH\\[n\\].LIMIT.LOW"]
    #[inline(always)]
    #[must_use]
    pub fn limitl(&mut self) -> LIMITL_W<0> {
        LIMITL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Last results is equal or below CH\\[n\\].LIMIT.LOW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limitl](index.html) module"]
pub struct LIMITL_SPEC;
impl crate::RegisterSpec for LIMITL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limitl::R](R) reader structure"]
impl crate::Readable for LIMITL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limitl::W](W) writer structure"]
impl crate::Writable for LIMITL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LIMITL to value 0"]
impl crate::Resettable for LIMITL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
