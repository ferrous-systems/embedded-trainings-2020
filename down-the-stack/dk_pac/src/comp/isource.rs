#[doc = "Register `ISOURCE` reader"]
pub struct R(crate::R<ISOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISOURCE` writer"]
pub struct W(crate::W<ISOURCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOURCE_SPEC>;
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
impl From<crate::W<ISOURCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOURCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISOURCE` reader - Comparator hysteresis"]
pub type ISOURCE_R = crate::FieldReader<u8, ISOURCE_A>;
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ISOURCE_A {
    #[doc = "0: Current source disabled"]
    OFF = 0,
    #[doc = "1: Current source enabled (+/- 2.5 uA)"]
    IEN2M_A5 = 1,
    #[doc = "2: Current source enabled (+/- 5 uA)"]
    IEN5M_A = 2,
    #[doc = "3: Current source enabled (+/- 10 uA)"]
    IEN10M_A = 3,
}
impl From<ISOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: ISOURCE_A) -> Self {
        variant as _
    }
}
impl ISOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOURCE_A {
        match self.bits {
            0 => ISOURCE_A::OFF,
            1 => ISOURCE_A::IEN2M_A5,
            2 => ISOURCE_A::IEN5M_A,
            3 => ISOURCE_A::IEN10M_A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ISOURCE_A::OFF
    }
    #[doc = "Checks if the value of the field is `IEN2M_A5`"]
    #[inline(always)]
    pub fn is_ien2m_a5(&self) -> bool {
        *self == ISOURCE_A::IEN2M_A5
    }
    #[doc = "Checks if the value of the field is `IEN5M_A`"]
    #[inline(always)]
    pub fn is_ien5m_a(&self) -> bool {
        *self == ISOURCE_A::IEN5M_A
    }
    #[doc = "Checks if the value of the field is `IEN10M_A`"]
    #[inline(always)]
    pub fn is_ien10m_a(&self) -> bool {
        *self == ISOURCE_A::IEN10M_A
    }
}
#[doc = "Field `ISOURCE` writer - Comparator hysteresis"]
pub type ISOURCE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ISOURCE_SPEC, u8, ISOURCE_A, 2, O>;
impl<'a, const O: u8> ISOURCE_W<'a, O> {
    #[doc = "Current source disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ISOURCE_A::OFF)
    }
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    #[inline(always)]
    pub fn ien2m_a5(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN2M_A5)
    }
    #[doc = "Current source enabled (+/- 5 uA)"]
    #[inline(always)]
    pub fn ien5m_a(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN5M_A)
    }
    #[doc = "Current source enabled (+/- 10 uA)"]
    #[inline(always)]
    pub fn ien10m_a(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN10M_A)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline(always)]
    pub fn isource(&self) -> ISOURCE_R {
        ISOURCE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn isource(&mut self) -> ISOURCE_W<0> {
        ISOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current source select on analog input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isource](index.html) module"]
pub struct ISOURCE_SPEC;
impl crate::RegisterSpec for ISOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isource::R](R) reader structure"]
impl crate::Readable for ISOURCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isource::W](W) writer structure"]
impl crate::Writable for ISOURCE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISOURCE to value 0"]
impl crate::Resettable for ISOURCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
