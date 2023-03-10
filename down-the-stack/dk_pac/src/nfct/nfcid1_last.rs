#[doc = "Register `NFCID1_LAST` reader"]
pub struct R(crate::R<NFCID1_LAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NFCID1_LAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NFCID1_LAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NFCID1_LAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NFCID1_LAST` writer"]
pub struct W(crate::W<NFCID1_LAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NFCID1_LAST_SPEC>;
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
impl From<crate::W<NFCID1_LAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NFCID1_LAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NFCID1_Z` reader - NFCID1 byte Z (very last byte sent)"]
pub type NFCID1_Z_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFCID1_Z` writer - NFCID1 byte Z (very last byte sent)"]
pub type NFCID1_Z_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NFCID1_LAST_SPEC, u8, u8, 8, O>;
#[doc = "Field `NFCID1_Y` reader - NFCID1 byte Y"]
pub type NFCID1_Y_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFCID1_Y` writer - NFCID1 byte Y"]
pub type NFCID1_Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NFCID1_LAST_SPEC, u8, u8, 8, O>;
#[doc = "Field `NFCID1_X` reader - NFCID1 byte X"]
pub type NFCID1_X_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFCID1_X` writer - NFCID1 byte X"]
pub type NFCID1_X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NFCID1_LAST_SPEC, u8, u8, 8, O>;
#[doc = "Field `NFCID1_W` reader - NFCID1 byte W"]
pub type NFCID1_W_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFCID1_W` writer - NFCID1 byte W"]
pub type NFCID1_W_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NFCID1_LAST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    pub fn nfcid1_z(&self) -> NFCID1_Z_R {
        NFCID1_Z_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    pub fn nfcid1_y(&self) -> NFCID1_Y_R {
        NFCID1_Y_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    pub fn nfcid1_x(&self) -> NFCID1_X_R {
        NFCID1_X_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    pub fn nfcid1_w(&self) -> NFCID1_W_R {
        NFCID1_W_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline(always)]
    #[must_use]
    pub fn nfcid1_z(&mut self) -> NFCID1_Z_W<0> {
        NFCID1_Z_W::new(self)
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline(always)]
    #[must_use]
    pub fn nfcid1_y(&mut self) -> NFCID1_Y_W<8> {
        NFCID1_Y_W::new(self)
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline(always)]
    #[must_use]
    pub fn nfcid1_x(&mut self) -> NFCID1_X_W<16> {
        NFCID1_X_W::new(self)
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline(always)]
    #[must_use]
    pub fn nfcid1_w(&mut self) -> NFCID1_W_W<24> {
        NFCID1_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last NFCID1 part (4, 7 or 10 bytes ID)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcid1_last](index.html) module"]
pub struct NFCID1_LAST_SPEC;
impl crate::RegisterSpec for NFCID1_LAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nfcid1_last::R](R) reader structure"]
impl crate::Readable for NFCID1_LAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nfcid1_last::W](W) writer structure"]
impl crate::Writable for NFCID1_LAST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NFCID1_LAST to value 0x6363"]
impl crate::Resettable for NFCID1_LAST_SPEC {
    const RESET_VALUE: Self::Ux = 0x6363;
}
