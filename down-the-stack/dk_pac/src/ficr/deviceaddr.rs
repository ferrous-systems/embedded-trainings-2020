#[doc = "Register `DEVICEADDR[%s]` reader"]
pub struct R(crate::R<DEVICEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICEADDR` reader - 48 bit device address"]
pub type DEVICEADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 48 bit device address"]
    #[inline(always)]
    pub fn deviceaddr(&self) -> DEVICEADDR_R {
        DEVICEADDR_R::new(self.bits)
    }
}
#[doc = "Description collection: Device address n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceaddr](index.html) module"]
pub struct DEVICEADDR_SPEC;
impl crate::RegisterSpec for DEVICEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deviceaddr::R](R) reader structure"]
impl crate::Readable for DEVICEADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEADDR[%s]
to value 0xffff_ffff"]
impl crate::Resettable for DEVICEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
