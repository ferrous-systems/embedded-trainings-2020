#[doc = "Register `EVENTS_TXPTRUPD` reader"]
pub struct R(crate::R<EVENTS_TXPTRUPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_TXPTRUPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_TXPTRUPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_TXPTRUPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_TXPTRUPD` writer"]
pub struct W(crate::W<EVENTS_TXPTRUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_TXPTRUPD_SPEC>;
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
impl From<crate::W<EVENTS_TXPTRUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_TXPTRUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_TXPTRUPD` reader - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub type EVENTS_TXPTRUPD_R = crate::BitReader<EVENTS_TXPTRUPD_A>;
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EVENTS_TXPTRUPD_A {
    #[doc = "0: Event not generated"]
    NOT_GENERATED = 0,
    #[doc = "1: Event generated"]
    GENERATED = 1,
}
impl From<EVENTS_TXPTRUPD_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_TXPTRUPD_A) -> Self {
        variant as u8 != 0
    }
}
impl EVENTS_TXPTRUPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_TXPTRUPD_A {
        match self.bits {
            false => EVENTS_TXPTRUPD_A::NOT_GENERATED,
            true => EVENTS_TXPTRUPD_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TXPTRUPD_A::NOT_GENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TXPTRUPD_A::GENERATED
    }
}
#[doc = "Field `EVENTS_TXPTRUPD` writer - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
pub type EVENTS_TXPTRUPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVENTS_TXPTRUPD_SPEC, EVENTS_TXPTRUPD_A, O>;
impl<'a, const O: u8> EVENTS_TXPTRUPD_W<'a, O> {
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TXPTRUPD_A::NOT_GENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TXPTRUPD_A::GENERATED)
    }
}
impl R {
    #[doc = "Bit 0 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    pub fn events_txptrupd(&self) -> EVENTS_TXPTRUPD_R {
        EVENTS_TXPTRUPD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin."]
    #[inline(always)]
    #[must_use]
    pub fn events_txptrupd(&mut self) -> EVENTS_TXPTRUPD_W<0> {
        EVENTS_TXPTRUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The TDX.PTR register has been copied to internal double-buffers. When the I2S module is started and TX is enabled, this event will be generated for every RXTXD.MAXCNT words that are sent on the SDOUT pin.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txptrupd](index.html) module"]
pub struct EVENTS_TXPTRUPD_SPEC;
impl crate::RegisterSpec for EVENTS_TXPTRUPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_txptrupd::R](R) reader structure"]
impl crate::Readable for EVENTS_TXPTRUPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_txptrupd::W](W) writer structure"]
impl crate::Writable for EVENTS_TXPTRUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVENTS_TXPTRUPD to value 0"]
impl crate::Resettable for EVENTS_TXPTRUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
