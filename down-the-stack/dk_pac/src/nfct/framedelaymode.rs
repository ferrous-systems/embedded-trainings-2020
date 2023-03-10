#[doc = "Register `FRAMEDELAYMODE` reader"]
pub struct R(crate::R<FRAMEDELAYMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMEDELAYMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMEDELAYMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMEDELAYMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMEDELAYMODE` writer"]
pub struct W(crate::W<FRAMEDELAYMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMEDELAYMODE_SPEC>;
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
impl From<crate::W<FRAMEDELAYMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMEDELAYMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAMEDELAYMODE` reader - Configuration register for the Frame Delay Timer"]
pub type FRAMEDELAYMODE_R = crate::FieldReader<u8, FRAMEDELAYMODE_A>;
#[doc = "Configuration register for the Frame Delay Timer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRAMEDELAYMODE_A {
    #[doc = "0: Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    FREE_RUN = 0,
    #[doc = "1: Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOW = 1,
    #[doc = "2: Frame is transmitted exactly at FRAMEDELAYMAX"]
    EXACT_VAL = 2,
    #[doc = "3: Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WINDOW_GRID = 3,
}
impl From<FRAMEDELAYMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FRAMEDELAYMODE_A) -> Self {
        variant as _
    }
}
impl FRAMEDELAYMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMEDELAYMODE_A {
        match self.bits {
            0 => FRAMEDELAYMODE_A::FREE_RUN,
            1 => FRAMEDELAYMODE_A::WINDOW,
            2 => FRAMEDELAYMODE_A::EXACT_VAL,
            3 => FRAMEDELAYMODE_A::WINDOW_GRID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FREE_RUN`"]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == FRAMEDELAYMODE_A::FREE_RUN
    }
    #[doc = "Checks if the value of the field is `WINDOW`"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == FRAMEDELAYMODE_A::WINDOW
    }
    #[doc = "Checks if the value of the field is `EXACT_VAL`"]
    #[inline(always)]
    pub fn is_exact_val(&self) -> bool {
        *self == FRAMEDELAYMODE_A::EXACT_VAL
    }
    #[doc = "Checks if the value of the field is `WINDOW_GRID`"]
    #[inline(always)]
    pub fn is_window_grid(&self) -> bool {
        *self == FRAMEDELAYMODE_A::WINDOW_GRID
    }
}
#[doc = "Field `FRAMEDELAYMODE` writer - Configuration register for the Frame Delay Timer"]
pub type FRAMEDELAYMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FRAMEDELAYMODE_SPEC, u8, FRAMEDELAYMODE_A, 2, O>;
impl<'a, const O: u8> FRAMEDELAYMODE_W<'a, O> {
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::FREE_RUN)
    }
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn window(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::WINDOW)
    }
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn exact_val(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::EXACT_VAL)
    }
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn window_grid(self) -> &'a mut W {
        self.variant(FRAMEDELAYMODE_A::WINDOW_GRID)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub fn framedelaymode(&self) -> FRAMEDELAYMODE_R {
        FRAMEDELAYMODE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    #[must_use]
    pub fn framedelaymode(&mut self) -> FRAMEDELAYMODE_W<0> {
        FRAMEDELAYMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for the Frame Delay Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framedelaymode](index.html) module"]
pub struct FRAMEDELAYMODE_SPEC;
impl crate::RegisterSpec for FRAMEDELAYMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framedelaymode::R](R) reader structure"]
impl crate::Readable for FRAMEDELAYMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framedelaymode::W](W) writer structure"]
impl crate::Writable for FRAMEDELAYMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRAMEDELAYMODE to value 0x01"]
impl crate::Resettable for FRAMEDELAYMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
