#[doc = "Register `ERRORSTATUS` reader"]
pub struct R(crate::R<ERRORSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRORSTATUS` writer"]
pub struct W(crate::W<ERRORSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRORSTATUS_SPEC>;
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
impl From<crate::W<ERRORSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRORSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDELAYTIMEOUT` reader - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
pub type FRAMEDELAYTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `FRAMEDELAYTIMEOUT` writer - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
pub type FRAMEDELAYTIMEOUT_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, ERRORSTATUS_SPEC, bool, O>;
#[doc = "Field `NFCFIELDTOOSTRONG` reader - Field level is too high at max load resistance"]
pub type NFCFIELDTOOSTRONG_R = crate::BitReader<bool>;
#[doc = "Field `NFCFIELDTOOSTRONG` writer - Field level is too high at max load resistance"]
pub type NFCFIELDTOOSTRONG_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, ERRORSTATUS_SPEC, bool, O>;
#[doc = "Field `NFCFIELDTOOWEAK` reader - Field level is too low at min load resistance"]
pub type NFCFIELDTOOWEAK_R = crate::BitReader<bool>;
#[doc = "Field `NFCFIELDTOOWEAK` writer - Field level is too low at min load resistance"]
pub type NFCFIELDTOOWEAK_W<'a, const O: u8> =
    crate::BitWriter1C<'a, u32, ERRORSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&self) -> FRAMEDELAYTIMEOUT_R {
        FRAMEDELAYTIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Field level is too high at max load resistance"]
    #[inline(always)]
    pub fn nfcfieldtoostrong(&self) -> NFCFIELDTOOSTRONG_R {
        NFCFIELDTOOSTRONG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Field level is too low at min load resistance"]
    #[inline(always)]
    pub fn nfcfieldtooweak(&self) -> NFCFIELDTOOWEAK_R {
        NFCFIELDTOOWEAK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    #[must_use]
    pub fn framedelaytimeout(&mut self) -> FRAMEDELAYTIMEOUT_W<0> {
        FRAMEDELAYTIMEOUT_W::new(self)
    }
    #[doc = "Bit 2 - Field level is too high at max load resistance"]
    #[inline(always)]
    #[must_use]
    pub fn nfcfieldtoostrong(&mut self) -> NFCFIELDTOOSTRONG_W<2> {
        NFCFIELDTOOSTRONG_W::new(self)
    }
    #[doc = "Bit 3 - Field level is too low at min load resistance"]
    #[inline(always)]
    #[must_use]
    pub fn nfcfieldtooweak(&mut self) -> NFCFIELDTOOWEAK_W<3> {
        NFCFIELDTOOWEAK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NFC Error Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorstatus](index.html) module"]
pub struct ERRORSTATUS_SPEC;
impl crate::RegisterSpec for ERRORSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorstatus::R](R) reader structure"]
impl crate::Readable for ERRORSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errorstatus::W](W) writer structure"]
impl crate::Writable for ERRORSTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0d;
}
#[doc = "`reset()` method sets ERRORSTATUS to value 0"]
impl crate::Resettable for ERRORSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
