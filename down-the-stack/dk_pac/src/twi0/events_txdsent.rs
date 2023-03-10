#[doc = "Register `EVENTS_TXDSENT` reader"]
pub struct R(crate::R<EVENTS_TXDSENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_TXDSENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_TXDSENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_TXDSENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_TXDSENT` writer"]
pub struct W(crate::W<EVENTS_TXDSENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_TXDSENT_SPEC>;
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
impl From<crate::W<EVENTS_TXDSENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_TXDSENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_TXDSENT` reader - TWI TXD byte sent"]
pub type EVENTS_TXDSENT_R = crate::BitReader<EVENTS_TXDSENT_A>;
#[doc = "TWI TXD byte sent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_TXDSENT_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_TXDSENT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_TXDSENT_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_TXDSENT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_TXDSENT_A {
        match self.bits {
            false => EVENTS_TXDSENT_A::NOT_GENERATED,
            true => EVENTS_TXDSENT_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TXDSENT_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TXDSENT_A::GENERATED
    }
}
#[doc = "Field `EVENTS_TXDSENT` writer - TWI TXD byte sent"]
pub type EVENTS_TXDSENT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_TXDSENT_SPEC, EVENTS_TXDSENT_A, O>;
impl<'a, const O: u8> EVENTS_TXDSENT_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TXDSENT_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TXDSENT_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - TWI TXD byte sent"]
    #[inline(always)]
    pub fn events_txdsent(&self) -> EVENTS_TXDSENT_R {
        EVENTS_TXDSENT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TWI TXD byte sent"]
    #[inline(always)]
    #[must_use]
    pub fn events_txdsent(&mut self) -> EVENTS_TXDSENT_W<0> {
        EVENTS_TXDSENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI TXD byte sent\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txdsent](index.html) module"]
pub struct EVENTS_TXDSENT_SPEC;
impl crate::RegisterSpec for EVENTS_TXDSENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_txdsent::R](R) reader structure"]
impl crate::Readable for EVENTS_TXDSENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_txdsent::W](W) writer structure"]
impl crate::Writable for EVENTS_TXDSENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_TXDSENT to value 0"]
impl crate::Resettable for EVENTS_TXDSENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
