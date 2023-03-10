#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDOWN` reader - Selects up or up and down as wave counter mode"]
pub type UPDOWN_R = crate::BitReader<UPDOWN_A>;
#[doc = "Selects up or up and down as wave counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDOWN_A {
    #[doc = "0: Up counter - edge aligned PWM duty-cycle"]
    UP = 0,
    #[doc = "1: Up and down counter - center aligned PWM duty cycle"]
    UP_AND_DOWN = 1,
}
impl From<UPDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: UPDOWN_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDOWN_A {
        match self.bits {
            false => UPDOWN_A::UP,
            true => UPDOWN_A::UP_AND_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == UPDOWN_A::UP
    }
    #[doc = "Checks if the value of the field is `UP_AND_DOWN`"]
    #[inline(always)]
    pub fn is_up_and_down(&self) -> bool {
        *self == UPDOWN_A::UP_AND_DOWN
    }
}
#[doc = "Field `UPDOWN` writer - Selects up or up and down as wave counter mode"]
pub type UPDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, UPDOWN_A, O>;
impl<'a, const O: u8> UPDOWN_W<'a, O> {
    #[doc = "Up counter - edge aligned PWM duty-cycle"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(UPDOWN_A::UP)
    }
    #[doc = "Up and down counter - center aligned PWM duty cycle"]
    #[inline(always)]
    pub fn up_and_down(self) -> &'a mut W {
        self.variant(UPDOWN_A::UP_AND_DOWN)
    }
}
impl R {
    #[doc = "Bit 0 - Selects up or up and down as wave counter mode"]
    #[inline(always)]
    pub fn updown(&self) -> UPDOWN_R {
        UPDOWN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects up or up and down as wave counter mode"]
    #[inline(always)]
    #[must_use]
    pub fn updown(&mut self) -> UPDOWN_W<0> {
        UPDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects operating mode of the wave counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
