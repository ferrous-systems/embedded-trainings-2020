#[doc = "Register `FREQUENCY` reader"]
pub struct R(crate::R<FREQUENCY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQUENCY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQUENCY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQUENCY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQUENCY` writer"]
pub struct W(crate::W<FREQUENCY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQUENCY_SPEC>;
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
impl From<crate::W<FREQUENCY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQUENCY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FREQUENCY` reader - TWI master clock frequency"]
pub type FREQUENCY_R = crate::FieldReader<u32, FREQUENCY_A>;
#[doc = "TWI master clock frequency\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum FREQUENCY_A {
    #[doc = "26738688: 100 kbps"]
    K100 = 26738688,
    #[doc = "67108864: 250 kbps"]
    K250 = 67108864,
    #[doc = "104857600: 400 kbps"]
    K400 = 104857600,
}
impl From<FREQUENCY_A> for u32 {
    #[inline(always)]
    fn from(variant: FREQUENCY_A) -> Self {
        variant as _
    }
}
impl FREQUENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FREQUENCY_A> {
        match self.bits {
            26738688 => Some(FREQUENCY_A::K100),
            67108864 => Some(FREQUENCY_A::K250),
            104857600 => Some(FREQUENCY_A::K400),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `K100`"]
    #[inline(always)]
    pub fn is_k100(&self) -> bool {
        *self == FREQUENCY_A::K100
    }
    #[doc = "Checks if the value of the field is `K250`"]
    #[inline(always)]
    pub fn is_k250(&self) -> bool {
        *self == FREQUENCY_A::K250
    }
    #[doc = "Checks if the value of the field is `K400`"]
    #[inline(always)]
    pub fn is_k400(&self) -> bool {
        *self == FREQUENCY_A::K400
    }
}
#[doc = "Field `FREQUENCY` writer - TWI master clock frequency"]
pub type FREQUENCY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FREQUENCY_SPEC, u32, FREQUENCY_A, 32, O>;
impl<'a, const O: u8> FREQUENCY_W<'a, O> {
    #[doc = "100 kbps"]
    #[inline(always)]
    pub fn k100(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K100)
    }
    #[doc = "250 kbps"]
    #[inline(always)]
    pub fn k250(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K250)
    }
    #[doc = "400 kbps"]
    #[inline(always)]
    pub fn k400(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K400)
    }
}
impl R {
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TWI master clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn frequency(&mut self) -> FREQUENCY_W<0> {
        FREQUENCY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI frequency\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](index.html) module"]
pub struct FREQUENCY_SPEC;
impl crate::RegisterSpec for FREQUENCY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frequency::R](R) reader structure"]
impl crate::Readable for FREQUENCY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frequency::W](W) writer structure"]
impl crate::Writable for FREQUENCY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQUENCY to value 0x0400_0000"]
impl crate::Resettable for FREQUENCY_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0000;
}
