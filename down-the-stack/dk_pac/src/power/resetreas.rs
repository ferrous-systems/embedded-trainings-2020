#[doc = "Register `RESETREAS` reader"]
pub struct R(crate::R<RESETREAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESETREAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESETREAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESETREAS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESETREAS` writer"]
pub struct W(crate::W<RESETREAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESETREAS_SPEC>;
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
impl From<crate::W<RESETREAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESETREAS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESETPIN` reader - Reset from pin-reset detected"]
pub type RESETPIN_R = crate::BitReader<RESETPIN_A>;
#[doc = "Reset from pin-reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETPIN_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<RESETPIN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETPIN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESETPIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETPIN_A {
        match self.bits {
            false => RESETPIN_A::NOT_DETECTED,
            true => RESETPIN_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == RESETPIN_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == RESETPIN_A::DETECTED
    }
}
#[doc = "Field `RESETPIN` writer - Reset from pin-reset detected"]
pub type RESETPIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, RESETPIN_A, O>;
impl<'a, const O: u8> RESETPIN_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESETPIN_A::DETECTED)
    }
}
#[doc = "Field `DOG` reader - Reset from watchdog detected"]
pub type DOG_R = crate::BitReader<DOG_A>;
#[doc = "Reset from watchdog detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOG_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DOG_A> for bool {
    #[inline(always)]
    fn from(variant: DOG_A) -> Self {
        variant as u8 != 0
    }
}
impl DOG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOG_A {
        match self.bits {
            false => DOG_A::NOT_DETECTED,
            true => DOG_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DOG_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DOG_A::DETECTED
    }
}
#[doc = "Field `DOG` writer - Reset from watchdog detected"]
pub type DOG_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, DOG_A, O>;
impl<'a, const O: u8> DOG_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOG_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOG_A::DETECTED)
    }
}
#[doc = "Field `SREQ` reader - Reset from soft reset detected"]
pub type SREQ_R = crate::BitReader<SREQ_A>;
#[doc = "Reset from soft reset detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SREQ_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<SREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl SREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SREQ_A {
        match self.bits {
            false => SREQ_A::NOT_DETECTED,
            true => SREQ_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == SREQ_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == SREQ_A::DETECTED
    }
}
#[doc = "Field `SREQ` writer - Reset from soft reset detected"]
pub type SREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, SREQ_A, O>;
impl<'a, const O: u8> SREQ_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SREQ_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(SREQ_A::DETECTED)
    }
}
#[doc = "Field `LOCKUP` reader - Reset from CPU lock-up detected"]
pub type LOCKUP_R = crate::BitReader<LOCKUP_A>;
#[doc = "Reset from CPU lock-up detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::NOT_DETECTED,
            true => LOCKUP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LOCKUP_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LOCKUP_A::DETECTED
    }
}
#[doc = "Field `LOCKUP` writer - Reset from CPU lock-up detected"]
pub type LOCKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, LOCKUP_A, O>;
impl<'a, const O: u8> LOCKUP_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LOCKUP_A::DETECTED)
    }
}
#[doc = "Field `OFF` reader - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
pub type OFF_R = crate::BitReader<OFF_A>;
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFF_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<OFF_A> for bool {
    #[inline(always)]
    fn from(variant: OFF_A) -> Self {
        variant as u8 != 0
    }
}
impl OFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFF_A {
        match self.bits {
            false => OFF_A::NOT_DETECTED,
            true => OFF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OFF_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OFF_A::DETECTED
    }
}
#[doc = "Field `OFF` writer - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
pub type OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, OFF_A, O>;
impl<'a, const O: u8> OFF_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OFF_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OFF_A::DETECTED)
    }
}
#[doc = "Field `LPCOMP` reader - Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
pub type LPCOMP_R = crate::BitReader<LPCOMP_A>;
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPCOMP_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<LPCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: LPCOMP_A) -> Self {
        variant as u8 != 0
    }
}
impl LPCOMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCOMP_A {
        match self.bits {
            false => LPCOMP_A::NOT_DETECTED,
            true => LPCOMP_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == LPCOMP_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == LPCOMP_A::DETECTED
    }
}
#[doc = "Field `LPCOMP` writer - Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
pub type LPCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, LPCOMP_A, O>;
impl<'a, const O: u8> LPCOMP_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(LPCOMP_A::DETECTED)
    }
}
#[doc = "Field `DIF` reader - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
pub type DIF_R = crate::BitReader<DIF_A>;
#[doc = "Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIF_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<DIF_A> for bool {
    #[inline(always)]
    fn from(variant: DIF_A) -> Self {
        variant as u8 != 0
    }
}
impl DIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIF_A {
        match self.bits {
            false => DIF_A::NOT_DETECTED,
            true => DIF_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == DIF_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == DIF_A::DETECTED
    }
}
#[doc = "Field `DIF` writer - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
pub type DIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, DIF_A, O>;
impl<'a, const O: u8> DIF_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DIF_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(DIF_A::DETECTED)
    }
}
#[doc = "Field `NFC` reader - Reset due to wake up from System OFF mode by NFC field detect"]
pub type NFC_R = crate::BitReader<NFC_A>;
#[doc = "Reset due to wake up from System OFF mode by NFC field detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NFC_A {
    #[doc = "0: Not detected"]
    NOT_DETECTED = 0,
    #[doc = "1: Detected"]
    DETECTED = 1,
}
impl From<NFC_A> for bool {
    #[inline(always)]
    fn from(variant: NFC_A) -> Self {
        variant as u8 != 0
    }
}
impl NFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NFC_A {
        match self.bits {
            false => NFC_A::NOT_DETECTED,
            true => NFC_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == NFC_A::NOT_DETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == NFC_A::DETECTED
    }
}
#[doc = "Field `NFC` writer - Reset due to wake up from System OFF mode by NFC field detect"]
pub type NFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RESETREAS_SPEC, NFC_A, O>;
impl<'a, const O: u8> NFC_W<'a, O> {
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(NFC_A::NOT_DETECTED)
    }
    #[doc = "Detected"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(NFC_A::DETECTED)
    }
}
impl R {
    #[doc = "Bit 0 - Reset from pin-reset detected"]
    #[inline(always)]
    pub fn resetpin(&self) -> RESETPIN_R {
        RESETPIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset from watchdog detected"]
    #[inline(always)]
    pub fn dog(&self) -> DOG_R {
        DOG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset from soft reset detected"]
    #[inline(always)]
    pub fn sreq(&self) -> SREQ_R {
        SREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
    #[inline(always)]
    pub fn lpcomp(&self) -> LPCOMP_R {
        LPCOMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset due to wake up from System OFF mode by NFC field detect"]
    #[inline(always)]
    pub fn nfc(&self) -> NFC_R {
        NFC_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset from pin-reset detected"]
    #[inline(always)]
    #[must_use]
    pub fn resetpin(&mut self) -> RESETPIN_W<0> {
        RESETPIN_W::new(self)
    }
    #[doc = "Bit 1 - Reset from watchdog detected"]
    #[inline(always)]
    #[must_use]
    pub fn dog(&mut self) -> DOG_W<1> {
        DOG_W::new(self)
    }
    #[doc = "Bit 2 - Reset from soft reset detected"]
    #[inline(always)]
    #[must_use]
    pub fn sreq(&mut self) -> SREQ_W<2> {
        SREQ_W::new(self)
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected"]
    #[inline(always)]
    #[must_use]
    pub fn lockup(&mut self) -> LOCKUP_W<3> {
        LOCKUP_W::new(self)
    }
    #[doc = "Bit 16 - Reset due to wake up from System OFF mode when wakeup is triggered from DETECT signal from GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OFF_W<16> {
        OFF_W::new(self)
    }
    #[doc = "Bit 17 - Reset due to wake up from System OFF mode when wakeup is triggered from ANADETECT signal from LPCOMP"]
    #[inline(always)]
    #[must_use]
    pub fn lpcomp(&mut self) -> LPCOMP_W<17> {
        LPCOMP_W::new(self)
    }
    #[doc = "Bit 18 - Reset due to wake up from System OFF mode when wakeup is triggered from entering into debug interface mode"]
    #[inline(always)]
    #[must_use]
    pub fn dif(&mut self) -> DIF_W<18> {
        DIF_W::new(self)
    }
    #[doc = "Bit 19 - Reset due to wake up from System OFF mode by NFC field detect"]
    #[inline(always)]
    #[must_use]
    pub fn nfc(&mut self) -> NFC_W<19> {
        NFC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset reason\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetreas](index.html) module"]
pub struct RESETREAS_SPEC;
impl crate::RegisterSpec for RESETREAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resetreas::R](R) reader structure"]
impl crate::Readable for RESETREAS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resetreas::W](W) writer structure"]
impl crate::Writable for RESETREAS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RESETREAS to value 0"]
impl crate::Resettable for RESETREAS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
