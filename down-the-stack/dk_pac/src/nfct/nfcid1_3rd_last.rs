#[doc = "Register `NFCID1_3RD_LAST` reader"]
pub struct R(crate::R<NFCID1_3RD_LAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCID1_3RD_LAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCID1_3RD_LAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCID1_3RD_LAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCID1_3RD_LAST` writer"]
pub struct W(crate::W<NFCID1_3RD_LAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCID1_3RD_LAST_SPEC>;
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
impl From<crate::W<NFCID1_3RD_LAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NFCID1_3RD_LAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NFCID1_S` reader - NFCID1 byte S"]
pub type NFCID1_S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFCID1_S` writer - NFCID1 byte S"]
pub type NFCID1_S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NFCID1_3RD_LAST_SPEC, u8, u8, 8, O>;
#[doc = "Field `NFCID1_R` reader - NFCID1 byte R"]
pub type NFCID1_R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFCID1_R` writer - NFCID1 byte R"]
pub type NFCID1_R_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NFCID1_3RD_LAST_SPEC, u8, u8, 8, O>;
#[doc = "Field `NFCID1_Q` reader - NFCID1 byte Q"]
pub type NFCID1_Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFCID1_Q` writer - NFCID1 byte Q"]
pub type NFCID1_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NFCID1_3RD_LAST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    pub fn nfcid1_s(&self) -> NFCID1_S_R {
        NFCID1_S_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    pub fn nfcid1_r(&self) -> NFCID1_R_R {
        NFCID1_R_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    pub fn nfcid1_q(&self) -> NFCID1_Q_R {
        NFCID1_Q_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte S"]
    #[inline(always)]
    #[must_use]
    pub fn nfcid1_s(&mut self) -> NFCID1_S_W<0> {
        NFCID1_S_W::new(self)
    }
    #[doc = "Bits 8:15 - NFCID1 byte R"]
    #[inline(always)]
    #[must_use]
    pub fn nfcid1_r(&mut self) -> NFCID1_R_W<8> {
        NFCID1_R_W::new(self)
    }
    #[doc = "Bits 16:23 - NFCID1 byte Q"]
    #[inline(always)]
    #[must_use]
    pub fn nfcid1_q(&mut self) -> NFCID1_Q_W<16> {
        NFCID1_Q_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Third last NFCID1 part (10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_3rd_last](index.html) module"]
pub struct NFCID1_3RD_LAST_SPEC;
impl crate::RegisterSpec for NFCID1_3RD_LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcid1_3rd_last::R](R) reader structure"]
impl crate::Readable for NFCID1_3RD_LAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcid1_3rd_last::W](W) writer structure"]
impl crate::Writable for NFCID1_3RD_LAST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NFCID1_3RD_LAST to value 0"]
impl crate::Resettable for NFCID1_3RD_LAST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
