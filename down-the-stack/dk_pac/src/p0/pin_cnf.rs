#[doc = "Register `PIN_CNF[%s]` reader"]
pub struct R(crate::R<PIN_CNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_CNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_CNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_CNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN_CNF[%s]` writer"]
pub struct W(crate::W<PIN_CNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_CNF_SPEC>;
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
impl From<crate::W<PIN_CNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_CNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Pin direction. Same physical register as DIR register"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Pin direction. Same physical register as DIR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Configure pin as an input pin"]
    INPUT = 0,
    #[doc = "1: Configure pin as an output pin"]
    OUTPUT = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::INPUT,
            true => DIR_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR_A::OUTPUT
    }
}
#[doc = "Field `DIR` writer - Pin direction. Same physical register as DIR register"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_CNF_SPEC, DIR_A, O>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Configure pin as an input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR_A::INPUT)
    }
    #[doc = "Configure pin as an output pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR_A::OUTPUT)
    }
}
#[doc = "Field `INPUT` reader - Connect or disconnect input buffer"]
pub type INPUT_R = crate::BitReader<INPUT_A>;
#[doc = "Connect or disconnect input buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INPUT_A {
    #[doc = "0: Connect input buffer"]
    CONNECT = 0,
    #[doc = "1: Disconnect input buffer"]
    DISCONNECT = 1,
}
impl From<INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_A) -> Self {
        variant as u8 != 0
    }
}
impl INPUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_A {
        match self.bits {
            false => INPUT_A::CONNECT,
            true => INPUT_A::DISCONNECT,
        }
    }
    #[doc = "Checks if the value of the field is `CONNECT`"]
    #[inline(always)]
    pub fn is_connect(&self) -> bool {
        *self == INPUT_A::CONNECT
    }
    #[doc = "Checks if the value of the field is `DISCONNECT`"]
    #[inline(always)]
    pub fn is_disconnect(&self) -> bool {
        *self == INPUT_A::DISCONNECT
    }
}
#[doc = "Field `INPUT` writer - Connect or disconnect input buffer"]
pub type INPUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN_CNF_SPEC, INPUT_A, O>;
impl<'a, const O: u8> INPUT_W<'a, O> {
    #[doc = "Connect input buffer"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(INPUT_A::CONNECT)
    }
    #[doc = "Disconnect input buffer"]
    #[inline(always)]
    pub fn disconnect(self) -> &'a mut W {
        self.variant(INPUT_A::DISCONNECT)
    }
}
#[doc = "Field `PULL` reader - Pull configuration"]
pub type PULL_R = crate::FieldReader<u8, PULL_A>;
#[doc = "Pull configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PULL_A {
    #[doc = "0: No pull"]
    DISABLED = 0,
    #[doc = "1: Pull down on pin"]
    PULLDOWN = 1,
    #[doc = "3: Pull up on pin"]
    PULLUP = 3,
}
impl From<PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PULL_A) -> Self {
        variant as _
    }
}
impl PULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PULL_A> {
        match self.bits {
            0 => Some(PULL_A::DISABLED),
            1 => Some(PULL_A::PULLDOWN),
            3 => Some(PULL_A::PULLUP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PULL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == PULL_A::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == PULL_A::PULLUP
    }
}
#[doc = "Field `PULL` writer - Pull configuration"]
pub type PULL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIN_CNF_SPEC, u8, PULL_A, 2, O>;
impl<'a, const O: u8> PULL_W<'a, O> {
    #[doc = "No pull"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PULL_A::DISABLED)
    }
    #[doc = "Pull down on pin"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(PULL_A::PULLDOWN)
    }
    #[doc = "Pull up on pin"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(PULL_A::PULLUP)
    }
}
#[doc = "Field `DRIVE` reader - Drive configuration"]
pub type DRIVE_R = crate::FieldReader<u8, DRIVE_A>;
#[doc = "Drive configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DRIVE_A {
    #[doc = "0: Standard '0', standard '1'"]
    S0S1 = 0,
    #[doc = "1: High drive '0', standard '1'"]
    H0S1 = 1,
    #[doc = "2: Standard '0', high drive '1'"]
    S0H1 = 2,
    #[doc = "3: High drive '0', high 'drive '1''"]
    H0H1 = 3,
    #[doc = "4: Disconnect '0' standard '1' (normally used for wired-or connections)"]
    D0S1 = 4,
    #[doc = "5: Disconnect '0', high drive '1' (normally used for wired-or connections)"]
    D0H1 = 5,
    #[doc = "6: Standard '0'. disconnect '1' (normally used for wired-and connections)"]
    S0D1 = 6,
    #[doc = "7: High drive '0', disconnect '1' (normally used for wired-and connections)"]
    H0D1 = 7,
}
impl From<DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_A) -> Self {
        variant as _
    }
}
impl DRIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_A {
        match self.bits {
            0 => DRIVE_A::S0S1,
            1 => DRIVE_A::H0S1,
            2 => DRIVE_A::S0H1,
            3 => DRIVE_A::H0H1,
            4 => DRIVE_A::D0S1,
            5 => DRIVE_A::D0H1,
            6 => DRIVE_A::S0D1,
            7 => DRIVE_A::H0D1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S0S1`"]
    #[inline(always)]
    pub fn is_s0s1(&self) -> bool {
        *self == DRIVE_A::S0S1
    }
    #[doc = "Checks if the value of the field is `H0S1`"]
    #[inline(always)]
    pub fn is_h0s1(&self) -> bool {
        *self == DRIVE_A::H0S1
    }
    #[doc = "Checks if the value of the field is `S0H1`"]
    #[inline(always)]
    pub fn is_s0h1(&self) -> bool {
        *self == DRIVE_A::S0H1
    }
    #[doc = "Checks if the value of the field is `H0H1`"]
    #[inline(always)]
    pub fn is_h0h1(&self) -> bool {
        *self == DRIVE_A::H0H1
    }
    #[doc = "Checks if the value of the field is `D0S1`"]
    #[inline(always)]
    pub fn is_d0s1(&self) -> bool {
        *self == DRIVE_A::D0S1
    }
    #[doc = "Checks if the value of the field is `D0H1`"]
    #[inline(always)]
    pub fn is_d0h1(&self) -> bool {
        *self == DRIVE_A::D0H1
    }
    #[doc = "Checks if the value of the field is `S0D1`"]
    #[inline(always)]
    pub fn is_s0d1(&self) -> bool {
        *self == DRIVE_A::S0D1
    }
    #[doc = "Checks if the value of the field is `H0D1`"]
    #[inline(always)]
    pub fn is_h0d1(&self) -> bool {
        *self == DRIVE_A::H0D1
    }
}
#[doc = "Field `DRIVE` writer - Drive configuration"]
pub type DRIVE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PIN_CNF_SPEC, u8, DRIVE_A, 3, O>;
impl<'a, const O: u8> DRIVE_W<'a, O> {
    #[doc = "Standard '0', standard '1'"]
    #[inline(always)]
    pub fn s0s1(self) -> &'a mut W {
        self.variant(DRIVE_A::S0S1)
    }
    #[doc = "High drive '0', standard '1'"]
    #[inline(always)]
    pub fn h0s1(self) -> &'a mut W {
        self.variant(DRIVE_A::H0S1)
    }
    #[doc = "Standard '0', high drive '1'"]
    #[inline(always)]
    pub fn s0h1(self) -> &'a mut W {
        self.variant(DRIVE_A::S0H1)
    }
    #[doc = "High drive '0', high 'drive '1''"]
    #[inline(always)]
    pub fn h0h1(self) -> &'a mut W {
        self.variant(DRIVE_A::H0H1)
    }
    #[doc = "Disconnect '0' standard '1' (normally used for wired-or connections)"]
    #[inline(always)]
    pub fn d0s1(self) -> &'a mut W {
        self.variant(DRIVE_A::D0S1)
    }
    #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
    #[inline(always)]
    pub fn d0h1(self) -> &'a mut W {
        self.variant(DRIVE_A::D0H1)
    }
    #[doc = "Standard '0'. disconnect '1' (normally used for wired-and connections)"]
    #[inline(always)]
    pub fn s0d1(self) -> &'a mut W {
        self.variant(DRIVE_A::S0D1)
    }
    #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
    #[inline(always)]
    pub fn h0d1(self) -> &'a mut W {
        self.variant(DRIVE_A::H0D1)
    }
}
#[doc = "Field `SENSE` reader - Pin sensing mechanism"]
pub type SENSE_R = crate::FieldReader<u8, SENSE_A>;
#[doc = "Pin sensing mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SENSE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Sense for high level"]
    HIGH = 2,
    #[doc = "3: Sense for low level"]
    LOW = 3,
}
impl From<SENSE_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE_A) -> Self {
        variant as _
    }
}
impl SENSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SENSE_A> {
        match self.bits {
            0 => Some(SENSE_A::DISABLED),
            2 => Some(SENSE_A::HIGH),
            3 => Some(SENSE_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SENSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE_A::LOW
    }
}
#[doc = "Field `SENSE` writer - Pin sensing mechanism"]
pub type SENSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIN_CNF_SPEC, u8, SENSE_A, 2, O>;
impl<'a, const O: u8> SENSE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SENSE_A::DISABLED)
    }
    #[doc = "Sense for high level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE_A::HIGH)
    }
    #[doc = "Sense for low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE_A::LOW)
    }
}
impl R {
    #[doc = "Bit 0 - Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect or disconnect input buffer"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pull configuration"]
    #[inline(always)]
    pub fn pull(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Drive configuration"]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism"]
    #[inline(always)]
    pub fn sense(&self) -> SENSE_R {
        SENSE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bit 1 - Connect or disconnect input buffer"]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> INPUT_W<1> {
        INPUT_W::new(self)
    }
    #[doc = "Bits 2:3 - Pull configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pull(&mut self) -> PULL_W<2> {
        PULL_W::new(self)
    }
    #[doc = "Bits 8:10 - Drive configuration"]
    #[inline(always)]
    #[must_use]
    pub fn drive(&mut self) -> DRIVE_W<8> {
        DRIVE_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism"]
    #[inline(always)]
    #[must_use]
    pub fn sense(&mut self) -> SENSE_W<16> {
        SENSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Configuration of GPIO pins\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin_cnf](index.html) module"]
pub struct PIN_CNF_SPEC;
impl crate::RegisterSpec for PIN_CNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin_cnf::R](R) reader structure"]
impl crate::Readable for PIN_CNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin_cnf::W](W) writer structure"]
impl crate::Writable for PIN_CNF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN_CNF[%s]
to value 0x02"]
impl crate::Resettable for PIN_CNF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
