#[doc = "Register `BAUDRATE` reader"]
pub struct R(crate::R<BAUDRATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUDRATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUDRATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUDRATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUDRATE` writer"]
pub struct W(crate::W<BAUDRATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUDRATE_SPEC>;
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
impl From<crate::W<BAUDRATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUDRATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUDRATE` reader - Baud rate"]
pub type BAUDRATE_R = crate::FieldReader<u32, BAUDRATE_A>;
#[doc = "Baud rate\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BAUDRATE_A {
    #[doc = "323584: 1200 baud (actual rate: 1205)"]
    BAUD1200 = 323584,
    #[doc = "643072: 2400 baud (actual rate: 2396)"]
    BAUD2400 = 643072,
    #[doc = "1290240: 4800 baud (actual rate: 4808)"]
    BAUD4800 = 1290240,
    #[doc = "2576384: 9600 baud (actual rate: 9598)"]
    BAUD9600 = 2576384,
    #[doc = "3862528: 14400 baud (actual rate: 14401)"]
    BAUD14400 = 3862528,
    #[doc = "5152768: 19200 baud (actual rate: 19208)"]
    BAUD19200 = 5152768,
    #[doc = "7716864: 28800 baud (actual rate: 28777)"]
    BAUD28800 = 7716864,
    #[doc = "8388608: 31250 baud"]
    BAUD31250 = 8388608,
    #[doc = "10289152: 38400 baud (actual rate: 38369)"]
    BAUD38400 = 10289152,
    #[doc = "15007744: 56000 baud (actual rate: 55944)"]
    BAUD56000 = 15007744,
    #[doc = "15400960: 57600 baud (actual rate: 57554)"]
    BAUD57600 = 15400960,
    #[doc = "20615168: 76800 baud (actual rate: 76923)"]
    BAUD76800 = 20615168,
    #[doc = "30801920: 115200 baud (actual rate: 115108)"]
    BAUD115200 = 30801920,
    #[doc = "61865984: 230400 baud (actual rate: 231884)"]
    BAUD230400 = 61865984,
    #[doc = "67108864: 250000 baud"]
    BAUD250000 = 67108864,
    #[doc = "121634816: 460800 baud (actual rate: 457143)"]
    BAUD460800 = 121634816,
    #[doc = "251658240: 921600 baud (actual rate: 941176)"]
    BAUD921600 = 251658240,
    #[doc = "268435456: 1Mega baud"]
    BAUD1M = 268435456,
}
impl From<BAUDRATE_A> for u32 {
    #[inline(always)]
    fn from(variant: BAUDRATE_A) -> Self {
        variant as _
    }
}
impl BAUDRATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAUDRATE_A> {
        match self.bits {
            323584 => Some(BAUDRATE_A::BAUD1200),
            643072 => Some(BAUDRATE_A::BAUD2400),
            1290240 => Some(BAUDRATE_A::BAUD4800),
            2576384 => Some(BAUDRATE_A::BAUD9600),
            3862528 => Some(BAUDRATE_A::BAUD14400),
            5152768 => Some(BAUDRATE_A::BAUD19200),
            7716864 => Some(BAUDRATE_A::BAUD28800),
            8388608 => Some(BAUDRATE_A::BAUD31250),
            10289152 => Some(BAUDRATE_A::BAUD38400),
            15007744 => Some(BAUDRATE_A::BAUD56000),
            15400960 => Some(BAUDRATE_A::BAUD57600),
            20615168 => Some(BAUDRATE_A::BAUD76800),
            30801920 => Some(BAUDRATE_A::BAUD115200),
            61865984 => Some(BAUDRATE_A::BAUD230400),
            67108864 => Some(BAUDRATE_A::BAUD250000),
            121634816 => Some(BAUDRATE_A::BAUD460800),
            251658240 => Some(BAUDRATE_A::BAUD921600),
            268435456 => Some(BAUDRATE_A::BAUD1M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BAUD1200`"]
    #[inline(always)]
    pub fn is_baud1200(&self) -> bool {
        *self == BAUDRATE_A::BAUD1200
    }
    #[doc = "Checks if the value of the field is `BAUD2400`"]
    #[inline(always)]
    pub fn is_baud2400(&self) -> bool {
        *self == BAUDRATE_A::BAUD2400
    }
    #[doc = "Checks if the value of the field is `BAUD4800`"]
    #[inline(always)]
    pub fn is_baud4800(&self) -> bool {
        *self == BAUDRATE_A::BAUD4800
    }
    #[doc = "Checks if the value of the field is `BAUD9600`"]
    #[inline(always)]
    pub fn is_baud9600(&self) -> bool {
        *self == BAUDRATE_A::BAUD9600
    }
    #[doc = "Checks if the value of the field is `BAUD14400`"]
    #[inline(always)]
    pub fn is_baud14400(&self) -> bool {
        *self == BAUDRATE_A::BAUD14400
    }
    #[doc = "Checks if the value of the field is `BAUD19200`"]
    #[inline(always)]
    pub fn is_baud19200(&self) -> bool {
        *self == BAUDRATE_A::BAUD19200
    }
    #[doc = "Checks if the value of the field is `BAUD28800`"]
    #[inline(always)]
    pub fn is_baud28800(&self) -> bool {
        *self == BAUDRATE_A::BAUD28800
    }
    #[doc = "Checks if the value of the field is `BAUD31250`"]
    #[inline(always)]
    pub fn is_baud31250(&self) -> bool {
        *self == BAUDRATE_A::BAUD31250
    }
    #[doc = "Checks if the value of the field is `BAUD38400`"]
    #[inline(always)]
    pub fn is_baud38400(&self) -> bool {
        *self == BAUDRATE_A::BAUD38400
    }
    #[doc = "Checks if the value of the field is `BAUD56000`"]
    #[inline(always)]
    pub fn is_baud56000(&self) -> bool {
        *self == BAUDRATE_A::BAUD56000
    }
    #[doc = "Checks if the value of the field is `BAUD57600`"]
    #[inline(always)]
    pub fn is_baud57600(&self) -> bool {
        *self == BAUDRATE_A::BAUD57600
    }
    #[doc = "Checks if the value of the field is `BAUD76800`"]
    #[inline(always)]
    pub fn is_baud76800(&self) -> bool {
        *self == BAUDRATE_A::BAUD76800
    }
    #[doc = "Checks if the value of the field is `BAUD115200`"]
    #[inline(always)]
    pub fn is_baud115200(&self) -> bool {
        *self == BAUDRATE_A::BAUD115200
    }
    #[doc = "Checks if the value of the field is `BAUD230400`"]
    #[inline(always)]
    pub fn is_baud230400(&self) -> bool {
        *self == BAUDRATE_A::BAUD230400
    }
    #[doc = "Checks if the value of the field is `BAUD250000`"]
    #[inline(always)]
    pub fn is_baud250000(&self) -> bool {
        *self == BAUDRATE_A::BAUD250000
    }
    #[doc = "Checks if the value of the field is `BAUD460800`"]
    #[inline(always)]
    pub fn is_baud460800(&self) -> bool {
        *self == BAUDRATE_A::BAUD460800
    }
    #[doc = "Checks if the value of the field is `BAUD921600`"]
    #[inline(always)]
    pub fn is_baud921600(&self) -> bool {
        *self == BAUDRATE_A::BAUD921600
    }
    #[doc = "Checks if the value of the field is `BAUD1M`"]
    #[inline(always)]
    pub fn is_baud1m(&self) -> bool {
        *self == BAUDRATE_A::BAUD1M
    }
}
#[doc = "Field `BAUDRATE` writer - Baud rate"]
pub type BAUDRATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAUDRATE_SPEC, u32, BAUDRATE_A, 32, O>;
impl<'a, const O: u8> BAUDRATE_W<'a, O> {
    #[doc = "1200 baud (actual rate: 1205)"]
    #[inline(always)]
    pub fn baud1200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD1200)
    }
    #[doc = "2400 baud (actual rate: 2396)"]
    #[inline(always)]
    pub fn baud2400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD2400)
    }
    #[doc = "4800 baud (actual rate: 4808)"]
    #[inline(always)]
    pub fn baud4800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD4800)
    }
    #[doc = "9600 baud (actual rate: 9598)"]
    #[inline(always)]
    pub fn baud9600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD9600)
    }
    #[doc = "14400 baud (actual rate: 14401)"]
    #[inline(always)]
    pub fn baud14400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD14400)
    }
    #[doc = "19200 baud (actual rate: 19208)"]
    #[inline(always)]
    pub fn baud19200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD19200)
    }
    #[doc = "28800 baud (actual rate: 28777)"]
    #[inline(always)]
    pub fn baud28800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD28800)
    }
    #[doc = "31250 baud"]
    #[inline(always)]
    pub fn baud31250(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD31250)
    }
    #[doc = "38400 baud (actual rate: 38369)"]
    #[inline(always)]
    pub fn baud38400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD38400)
    }
    #[doc = "56000 baud (actual rate: 55944)"]
    #[inline(always)]
    pub fn baud56000(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD56000)
    }
    #[doc = "57600 baud (actual rate: 57554)"]
    #[inline(always)]
    pub fn baud57600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD57600)
    }
    #[doc = "76800 baud (actual rate: 76923)"]
    #[inline(always)]
    pub fn baud76800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD76800)
    }
    #[doc = "115200 baud (actual rate: 115108)"]
    #[inline(always)]
    pub fn baud115200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD115200)
    }
    #[doc = "230400 baud (actual rate: 231884)"]
    #[inline(always)]
    pub fn baud230400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD230400)
    }
    #[doc = "250000 baud"]
    #[inline(always)]
    pub fn baud250000(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD250000)
    }
    #[doc = "460800 baud (actual rate: 457143)"]
    #[inline(always)]
    pub fn baud460800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD460800)
    }
    #[doc = "921600 baud (actual rate: 941176)"]
    #[inline(always)]
    pub fn baud921600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD921600)
    }
    #[doc = "1Mega baud"]
    #[inline(always)]
    pub fn baud1m(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD1M)
    }
}
impl R {
    #[doc = "Bits 0:31 - Baud rate"]
    #[inline(always)]
    pub fn baudrate(&self) -> BAUDRATE_R {
        BAUDRATE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Baud rate"]
    #[inline(always)]
    #[must_use]
    pub fn baudrate(&mut self) -> BAUDRATE_W<0> {
        BAUDRATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud rate. Accuracy depends on the HFCLK source selected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baudrate](index.html) module"]
pub struct BAUDRATE_SPEC;
impl crate::RegisterSpec for BAUDRATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baudrate::R](R) reader structure"]
impl crate::Readable for BAUDRATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baudrate::W](W) writer structure"]
impl crate::Writable for BAUDRATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAUDRATE to value 0x0400_0000"]
impl crate::Resettable for BAUDRATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0000;
}
