#[doc = "Register `TRACECONFIG` reader"]
pub struct R(crate::R<TRACECONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRACECONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRACECONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRACECONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRACECONFIG` writer"]
pub struct W(crate::W<TRACECONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRACECONFIG_SPEC>;
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
impl From<crate::W<TRACECONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRACECONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRACEPORTSPEED` reader - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
pub type TRACEPORTSPEED_R = crate::FieldReader<u8, TRACEPORTSPEED_A>;
#[doc = "Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRACEPORTSPEED_A {
    #[doc = "0: 32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    _32MHZ = 0,
    #[doc = "1: 16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    _16MHZ = 1,
    #[doc = "2: 8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    _8MHZ = 2,
    #[doc = "3: 4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    _4MHZ = 3,
}
impl From<TRACEPORTSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEPORTSPEED_A) -> Self {
        variant as _
    }
}
impl TRACEPORTSPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACEPORTSPEED_A {
        match self.bits {
            0 => TRACEPORTSPEED_A::_32MHZ,
            1 => TRACEPORTSPEED_A::_16MHZ,
            2 => TRACEPORTSPEED_A::_8MHZ,
            3 => TRACEPORTSPEED_A::_4MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_32MHZ
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_8MHZ`"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_8MHZ
    }
    #[doc = "Checks if the value of the field is `_4MHZ`"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        *self == TRACEPORTSPEED_A::_4MHZ
    }
}
#[doc = "Field `TRACEPORTSPEED` writer - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
pub type TRACEPORTSPEED_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TRACECONFIG_SPEC, u8, TRACEPORTSPEED_A, 2, O>;
impl<'a, const O: u8> TRACEPORTSPEED_W<'a, O> {
    #[doc = "32 MHz Trace Port clock (TRACECLK = 16 MHz)"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_32MHZ)
    }
    #[doc = "16 MHz Trace Port clock (TRACECLK = 8 MHz)"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_16MHZ)
    }
    #[doc = "8 MHz Trace Port clock (TRACECLK = 4 MHz)"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_8MHZ)
    }
    #[doc = "4 MHz Trace Port clock (TRACECLK = 2 MHz)"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut W {
        self.variant(TRACEPORTSPEED_A::_4MHZ)
    }
}
#[doc = "Field `TRACEMUX` reader - Pin multiplexing of trace signals."]
pub type TRACEMUX_R = crate::FieldReader<u8, TRACEMUX_A>;
#[doc = "Pin multiplexing of trace signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRACEMUX_A {
    #[doc = "0: GPIOs multiplexed onto all trace-pins"]
    GPIO = 0,
    #[doc = "1: SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
    SERIAL = 1,
    #[doc = "2: TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
    PARALLEL = 2,
}
impl From<TRACEMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TRACEMUX_A) -> Self {
        variant as _
    }
}
impl TRACEMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRACEMUX_A> {
        match self.bits {
            0 => Some(TRACEMUX_A::GPIO),
            1 => Some(TRACEMUX_A::SERIAL),
            2 => Some(TRACEMUX_A::PARALLEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO`"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TRACEMUX_A::GPIO
    }
    #[doc = "Checks if the value of the field is `SERIAL`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == TRACEMUX_A::SERIAL
    }
    #[doc = "Checks if the value of the field is `PARALLEL`"]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        *self == TRACEMUX_A::PARALLEL
    }
}
#[doc = "Field `TRACEMUX` writer - Pin multiplexing of trace signals."]
pub type TRACEMUX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRACECONFIG_SPEC, u8, TRACEMUX_A, 2, O>;
impl<'a, const O: u8> TRACEMUX_W<'a, O> {
    #[doc = "GPIOs multiplexed onto all trace-pins"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut W {
        self.variant(TRACEMUX_A::GPIO)
    }
    #[doc = "SWO multiplexed onto P0.18, GPIO multiplexed onto other trace pins"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut W {
        self.variant(TRACEMUX_A::SERIAL)
    }
    #[doc = "TRACECLK and TRACEDATA multiplexed onto P0.20, P0.18, P0.16, P0.15 and P0.14."]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut W {
        self.variant(TRACEMUX_A::PARALLEL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TRACEPORTSPEED_R {
        TRACEPORTSPEED_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals."]
    #[inline(always)]
    pub fn tracemux(&self) -> TRACEMUX_R {
        TRACEMUX_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of Trace Port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    #[must_use]
    pub fn traceportspeed(&mut self) -> TRACEPORTSPEED_W<0> {
        TRACEPORTSPEED_W::new(self)
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals."]
    #[inline(always)]
    #[must_use]
    pub fn tracemux(&mut self) -> TRACEMUX_W<16> {
        TRACEMUX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clocking options for the Trace Port debug interface\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceconfig](index.html) module"]
pub struct TRACECONFIG_SPEC;
impl crate::RegisterSpec for TRACECONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [traceconfig::R](R) reader structure"]
impl crate::Readable for TRACECONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [traceconfig::W](W) writer structure"]
impl crate::Writable for TRACECONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRACECONFIG to value 0"]
impl crate::Resettable for TRACECONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
