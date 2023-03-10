#[doc = "Register `IR[%s]` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IR` reader - Identity Root, word n"]
pub type IR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Identity Root, word n"]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new(self.bits)
    }
}
#[doc = "Description collection: Identity Root, word n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IR[%s]
to value 0xffff_ffff"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
