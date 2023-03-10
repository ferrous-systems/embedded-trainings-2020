#[doc = "Register `DATAWHITEIV` reader"]
pub struct R(crate::R<DATAWHITEIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAWHITEIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAWHITEIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAWHITEIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAWHITEIV` writer"]
pub struct W(crate::W<DATAWHITEIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAWHITEIV_SPEC>;
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
impl From<crate::W<DATAWHITEIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAWHITEIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAWHITEIV` reader - Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
pub type DATAWHITEIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAWHITEIV` writer - Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
pub type DATAWHITEIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATAWHITEIV_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    pub fn datawhiteiv(&self) -> DATAWHITEIV_R {
        DATAWHITEIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Data whitening initial value. Bit 6 is hard-wired to '1', writing '0' to it has no effect, and it will always be read back and used by the device as '1'."]
    #[inline(always)]
    #[must_use]
    pub fn datawhiteiv(&mut self) -> DATAWHITEIV_W<0> {
        DATAWHITEIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data whitening initial value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datawhiteiv](index.html) module"]
pub struct DATAWHITEIV_SPEC;
impl crate::RegisterSpec for DATAWHITEIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datawhiteiv::R](R) reader structure"]
impl crate::Readable for DATAWHITEIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datawhiteiv::W](W) writer structure"]
impl crate::Writable for DATAWHITEIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAWHITEIV to value 0x40"]
impl crate::Resettable for DATAWHITEIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
