#[doc = "Register `CONFIG[%s]` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG[%s]` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
    DISABLED = 0,
    #[doc = "1: Event mode"]
    EVENT = 1,
    #[doc = "3: Task mode"]
    TASK = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DISABLED),
            1 => Some(MODE_A::EVENT),
            3 => Some(MODE_A::TASK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MODE_A::EVENT
    }
    #[doc = "Checks if the value of the field is `TASK`"]
    #[inline(always)]
    pub fn is_task(&self) -> bool {
        *self == MODE_A::TASK
    }
}
#[doc = "Field `MODE` writer - Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Disabled. Pin specified by PSEL will not be acquired by the GPIOTE module."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE_A::DISABLED)
    }
    #[doc = "Event mode"]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(MODE_A::EVENT)
    }
    #[doc = "Task mode"]
    #[inline(always)]
    pub fn task(self) -> &'a mut W {
        self.variant(MODE_A::TASK)
    }
}
#[doc = "Field `PSEL` reader - GPIO number associated with SET\\[n\\], CLR\\[n\\]
and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
pub type PSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEL` writer - GPIO number associated with SET\\[n\\], CLR\\[n\\]
and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `POLARITY` reader - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
pub type POLARITY_R = crate::FieldReader<u8, POLARITY_A>;
#[doc = "When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLARITY_A {
    #[doc = "0: Task mode: No effect on pin from OUT\\[n\\]
task. Event mode: no IN\\[n\\]
event generated on pin activity."]
    NONE = 0,
    #[doc = "1: Task mode: Set pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when rising edge on pin."]
    LO_TO_HI = 1,
    #[doc = "2: Task mode: Clear pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when falling edge on pin."]
    HI_TO_LO = 2,
    #[doc = "3: Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\]
when any change on pin."]
    TOGGLE = 3,
}
impl From<POLARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        variant as _
    }
}
impl POLARITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            0 => POLARITY_A::NONE,
            1 => POLARITY_A::LO_TO_HI,
            2 => POLARITY_A::HI_TO_LO,
            3 => POLARITY_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == POLARITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `LO_TO_HI`"]
    #[inline(always)]
    pub fn is_lo_to_hi(&self) -> bool {
        *self == POLARITY_A::LO_TO_HI
    }
    #[doc = "Checks if the value of the field is `HI_TO_LO`"]
    #[inline(always)]
    pub fn is_hi_to_lo(&self) -> bool {
        *self == POLARITY_A::HI_TO_LO
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == POLARITY_A::TOGGLE
    }
}
#[doc = "Field `POLARITY` writer - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
pub type POLARITY_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, POLARITY_A, 2, O>;
impl<'a, const O: u8> POLARITY_W<'a, O> {
    #[doc = "Task mode: No effect on pin from OUT\\[n\\]
task. Event mode: no IN\\[n\\]
event generated on pin activity."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(POLARITY_A::NONE)
    }
    #[doc = "Task mode: Set pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when rising edge on pin."]
    #[inline(always)]
    pub fn lo_to_hi(self) -> &'a mut W {
        self.variant(POLARITY_A::LO_TO_HI)
    }
    #[doc = "Task mode: Clear pin from OUT\\[n\\]
task. Event mode: Generate IN\\[n\\]
event when falling edge on pin."]
    #[inline(always)]
    pub fn hi_to_lo(self) -> &'a mut W {
        self.variant(POLARITY_A::HI_TO_LO)
    }
    #[doc = "Task mode: Toggle pin from OUT\\[n\\]. Event mode: Generate IN\\[n\\]
when any change on pin."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(POLARITY_A::TOGGLE)
    }
}
#[doc = "Field `OUTINIT` reader - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
pub type OUTINIT_R = crate::BitReader<OUTINIT_A>;
#[doc = "When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTINIT_A {
    #[doc = "0: Task mode: Initial value of pin before task triggering is low"]
    LOW = 0,
    #[doc = "1: Task mode: Initial value of pin before task triggering is high"]
    HIGH = 1,
}
impl From<OUTINIT_A> for bool {
    #[inline(always)]
    fn from(variant: OUTINIT_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTINIT_A {
        match self.bits {
            false => OUTINIT_A::LOW,
            true => OUTINIT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTINIT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTINIT_A::HIGH
    }
}
#[doc = "Field `OUTINIT` writer - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
pub type OUTINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, OUTINIT_A, O>;
impl<'a, const O: u8> OUTINIT_W<'a, O> {
    #[doc = "Task mode: Initial value of pin before task triggering is low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUTINIT_A::LOW)
    }
    #[doc = "Task mode: Initial value of pin before task triggering is high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUTINIT_A::HIGH)
    }
}
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:12 - GPIO number associated with SET\\[n\\], CLR\\[n\\]
and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
    #[inline(always)]
    pub fn outinit(&self) -> OUTINIT_R {
        OUTINIT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 8:12 - GPIO number associated with SET\\[n\\], CLR\\[n\\]
and OUT\\[n\\]
tasks and IN\\[n\\]
event"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<8> {
        PSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - When In task mode: Operation to be performed on output when OUT\\[n\\]
task is triggered. When In event mode: Operation on input that shall trigger IN\\[n\\]
event."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> POLARITY_W<16> {
        POLARITY_W::new(self)
    }
    #[doc = "Bit 20 - When in task mode: Initial value of the output when the GPIOTE channel is configured. When in event mode: No effect."]
    #[inline(always)]
    #[must_use]
    pub fn outinit(&mut self) -> OUTINIT_W<20> {
        OUTINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Configuration for OUT\\[n\\], SET\\[n\\]
and CLR\\[n\\]
tasks and IN\\[n\\]
event\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG[%s]
to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
