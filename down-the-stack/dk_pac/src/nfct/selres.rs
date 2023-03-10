#[doc = "Register `SELRES` reader"]
pub struct R(crate::R<SELRES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SELRES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SELRES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SELRES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SELRES` writer"]
pub struct W(crate::W<SELRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SELRES_SPEC>;
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
impl From<crate::W<SELRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SELRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFU10` reader - Reserved for future use. Shall be 0."]
pub type RFU10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFU10` writer - Reserved for future use. Shall be 0."]
pub type RFU10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SELRES_SPEC, u8, u8, 2, O>;
#[doc = "Field `CASCADE` reader - Cascade bit (controlled by hardware, write has no effect)"]
pub type CASCADE_R = crate::BitReader<CASCADE_A>;
#[doc = "Cascade bit (controlled by hardware, write has no effect)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CASCADE_A {
    #[doc = "0: NFCID1 complete"]
    COMPLETE = 0,
    #[doc = "1: NFCID1 not complete"]
    NOT_COMPLETE = 1,
}
impl From<CASCADE_A> for bool {
    #[inline(always)]
    fn from(variant: CASCADE_A) -> Self {
        variant as u8 != 0
    }
}
impl CASCADE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASCADE_A {
        match self.bits {
            false => CASCADE_A::COMPLETE,
            true => CASCADE_A::NOT_COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CASCADE_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CASCADE_A::NOT_COMPLETE
    }
}
#[doc = "Field `CASCADE` writer - Cascade bit (controlled by hardware, write has no effect)"]
pub type CASCADE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SELRES_SPEC, CASCADE_A, O>;
impl<'a, const O: u8> CASCADE_W<'a, O> {
    #[doc = "NFCID1 complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(CASCADE_A::COMPLETE)
    }
    #[doc = "NFCID1 not complete"]
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(CASCADE_A::NOT_COMPLETE)
    }
}
#[doc = "Field `RFU43` reader - Reserved for future use. Shall be 0."]
pub type RFU43_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFU43` writer - Reserved for future use. Shall be 0."]
pub type RFU43_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SELRES_SPEC, u8, u8, 2, O>;
#[doc = "Field `PROTOCOL` reader - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type PROTOCOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROTOCOL` writer - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
pub type PROTOCOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SELRES_SPEC, u8, u8, 2, O>;
#[doc = "Field `RFU7` reader - Reserved for future use. Shall be 0."]
pub type RFU7_R = crate::BitReader<bool>;
#[doc = "Field `RFU7` writer - Reserved for future use. Shall be 0."]
pub type RFU7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SELRES_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu10(&self) -> RFU10_R {
        RFU10_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline(always)]
    pub fn cascade(&self) -> CASCADE_R {
        CASCADE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu43(&self) -> RFU43_R {
        RFU43_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu7(&self) -> RFU7_R {
        RFU7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    #[must_use]
    pub fn rfu10(&mut self) -> RFU10_W<0> {
        RFU10_W::new(self)
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline(always)]
    #[must_use]
    pub fn cascade(&mut self) -> CASCADE_W<2> {
        CASCADE_W::new(self)
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    #[must_use]
    pub fn rfu43(&mut self) -> RFU43_W<3> {
        RFU43_W::new(self)
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    #[must_use]
    pub fn protocol(&mut self) -> PROTOCOL_W<5> {
        PROTOCOL_W::new(self)
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    #[must_use]
    pub fn rfu7(&mut self) -> RFU7_W<7> {
        RFU7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NFC-A SEL_RES auto-response settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [selres](index.html) module"]
pub struct SELRES_SPEC;
impl crate::RegisterSpec for SELRES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [selres::R](R) reader structure"]
impl crate::Readable for SELRES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [selres::W](W) writer structure"]
impl crate::Writable for SELRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SELRES to value 0"]
impl crate::Resettable for SELRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
