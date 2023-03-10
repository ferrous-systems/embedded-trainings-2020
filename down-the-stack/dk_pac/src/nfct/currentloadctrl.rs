#[doc = "Register `CURRENTLOADCTRL` reader"]
pub struct R(crate::R<CURRENTLOADCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CURRENTLOADCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CURRENTLOADCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CURRENTLOADCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CURRENTLOADCTRL` reader - Current value driven to the NFC Load Control"]
pub type CURRENTLOADCTRL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Current value driven to the NFC Load Control"]
    #[inline(always)]
    pub fn currentloadctrl(&self) -> CURRENTLOADCTRL_R {
        CURRENTLOADCTRL_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Current value driven to the NFC Load Control\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [currentloadctrl](index.html) module"]
pub struct CURRENTLOADCTRL_SPEC;
impl crate::RegisterSpec for CURRENTLOADCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [currentloadctrl::R](R) reader structure"]
impl crate::Readable for CURRENTLOADCTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CURRENTLOADCTRL to value 0"]
impl crate::Resettable for CURRENTLOADCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
