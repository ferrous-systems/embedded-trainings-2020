#[doc = "Register `FORCEPROTECT` reader"]
pub struct R(crate::R<FORCEPROTECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FORCEPROTECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FORCEPROTECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FORCEPROTECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FORCEPROTECT` writer"]
pub struct W(crate::W<FORCEPROTECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FORCEPROTECT_SPEC>;
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
impl From<crate::W<FORCEPROTECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FORCEPROTECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCEPROTECT` reader - Write 0x0 to force enable APPROTECT mechanism"]
pub type FORCEPROTECT_R = crate::FieldReader<u8, FORCEPROTECT_A>;
#[doc = "Write 0x0 to force enable APPROTECT mechanism\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORCEPROTECT_A {
    #[doc = "0: Software force enable APPROTECT mechanism"]
    FORCE = 0,
}
impl From<FORCEPROTECT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORCEPROTECT_A) -> Self {
        variant as _
    }
}
impl FORCEPROTECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORCEPROTECT_A> {
        match self.bits {
            0 => Some(FORCEPROTECT_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCEPROTECT_A::FORCE
    }
}
#[doc = "Field `FORCEPROTECT` writer - Write 0x0 to force enable APPROTECT mechanism"]
pub type FORCEPROTECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FORCEPROTECT_SPEC, u8, FORCEPROTECT_A, 8, O>;
impl<'a, const O: u8> FORCEPROTECT_W<'a, O> {
    #[doc = "Software force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEPROTECT_A::FORCE)
    }
}
impl R {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    pub fn forceprotect(&self) -> FORCEPROTECT_R {
        FORCEPROTECT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write 0x0 to force enable APPROTECT mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn forceprotect(&mut self) -> FORCEPROTECT_W<0> {
        FORCEPROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software force enable APPROTECT mechanism until next reset. This register can only be written once.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [forceprotect](index.html) module"]
pub struct FORCEPROTECT_SPEC;
impl crate::RegisterSpec for FORCEPROTECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [forceprotect::R](R) reader structure"]
impl crate::Readable for FORCEPROTECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [forceprotect::W](W) writer structure"]
impl crate::Writable for FORCEPROTECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FORCEPROTECT to value 0xffff_ffff"]
impl crate::Resettable for FORCEPROTECT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
