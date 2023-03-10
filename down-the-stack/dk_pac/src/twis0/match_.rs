#[doc = "Register `MATCH` reader"]
pub struct R(crate::R<MATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MATCH` reader - Which of the addresses in {ADDRESS} matched the incoming address"]
pub type MATCH_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Which of the addresses in {ADDRESS} matched the incoming address"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Status register indicating which address had a match\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [match_](index.html) module"]
pub struct MATCH_SPEC;
impl crate::RegisterSpec for MATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [match_::R](R) reader structure"]
impl crate::Readable for MATCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MATCH to value 0"]
impl crate::Resettable for MATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
