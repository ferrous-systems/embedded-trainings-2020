#[doc = "Register `WA` reader"]
pub struct R(crate::R<WA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WA` writer"]
pub struct W(crate::W<WA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WA_SPEC>;
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
impl From<crate::W<WA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WA` reader - Write access to peripheral region n detected"]
pub type WA_R = crate::BitReader<WA_A>;
#[doc = "Write access to peripheral region n detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WA_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<WA_A> for bool {
    #[inline(always)]
    fn from(variant: WA_A) -> Self {
        variant as u8 != 0
    }
}
impl WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WA_A {
        match self.bits {
            false => WA_A::NOT_GENERATED,
            true => WA_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == WA_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == WA_A::GENERATED
    }
}
#[doc = "Field `WA` writer - Write access to peripheral region n detected"]
pub type WA_W<'a, const O: u8> = crate::BitWriter<'a, u32, WA_SPEC, WA_A, O>;
impl<'a, const O: u8> WA_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(WA_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(WA_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - Write access to peripheral region n detected"]
    #[inline(always)]
    pub fn wa(&self) -> WA_R {
        WA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write access to peripheral region n detected"]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WA_W<0> {
        WA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description cluster: Write access to peripheral region n detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wa](index.html) module"]
pub struct WA_SPEC;
impl crate::RegisterSpec for WA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wa::R](R) reader structure"]
impl crate::Readable for WA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wa::W](W) writer structure"]
impl crate::Writable for WA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WA to value 0"]
impl crate::Resettable for WA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
