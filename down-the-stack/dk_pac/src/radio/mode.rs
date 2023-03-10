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
#[doc = "Field `MODE` reader - Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 1 Mbit/s Nordic proprietary radio mode"]
    NRF_1MBIT = 0,
    #[doc = "1: 2 Mbit/s Nordic proprietary radio mode"]
    NRF_2MBIT = 1,
    #[doc = "2: Deprecated enumerator - 250 kbit/s Nordic proprietary radio mode"]
    NRF_250KBIT = 2,
    #[doc = "3: 1 Mbit/s Bluetooth Low Energy"]
    BLE_1MBIT = 3,
    #[doc = "4: 2 Mbit/s Bluetooth Low Energy"]
    BLE_2MBIT = 4,
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
            0 => Some(MODE_A::NRF_1MBIT),
            1 => Some(MODE_A::NRF_2MBIT),
            2 => Some(MODE_A::NRF_250KBIT),
            3 => Some(MODE_A::BLE_1MBIT),
            4 => Some(MODE_A::BLE_2MBIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NRF_1MBIT`"]
    #[inline(always)]
    pub fn is_nrf_1mbit(&self) -> bool {
        *self == MODE_A::NRF_1MBIT
    }
    #[doc = "Checks if the value of the field is `NRF_2MBIT`"]
    #[inline(always)]
    pub fn is_nrf_2mbit(&self) -> bool {
        *self == MODE_A::NRF_2MBIT
    }
    #[doc = "Checks if the value of the field is `NRF_250KBIT`"]
    #[inline(always)]
    pub fn is_nrf_250kbit(&self) -> bool {
        *self == MODE_A::NRF_250KBIT
    }
    #[doc = "Checks if the value of the field is `BLE_1MBIT`"]
    #[inline(always)]
    pub fn is_ble_1mbit(&self) -> bool {
        *self == MODE_A::BLE_1MBIT
    }
    #[doc = "Checks if the value of the field is `BLE_2MBIT`"]
    #[inline(always)]
    pub fn is_ble_2mbit(&self) -> bool {
        *self == MODE_A::BLE_2MBIT
    }
}
#[doc = "Field `MODE` writer - Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, MODE_A, 4, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "1 Mbit/s Nordic proprietary radio mode"]
    #[inline(always)]
    pub fn nrf_1mbit(self) -> &'a mut W {
        self.variant(MODE_A::NRF_1MBIT)
    }
    #[doc = "2 Mbit/s Nordic proprietary radio mode"]
    #[inline(always)]
    pub fn nrf_2mbit(self) -> &'a mut W {
        self.variant(MODE_A::NRF_2MBIT)
    }
    #[doc = "Deprecated enumerator - 250 kbit/s Nordic proprietary radio mode"]
    #[inline(always)]
    pub fn nrf_250kbit(self) -> &'a mut W {
        self.variant(MODE_A::NRF_250KBIT)
    }
    #[doc = "1 Mbit/s Bluetooth Low Energy"]
    #[inline(always)]
    pub fn ble_1mbit(self) -> &'a mut W {
        self.variant(MODE_A::BLE_1MBIT)
    }
    #[doc = "2 Mbit/s Bluetooth Low Energy"]
    #[inline(always)]
    pub fn ble_2mbit(self) -> &'a mut W {
        self.variant(MODE_A::BLE_2MBIT)
    }
}
impl R {
    #[doc = "Bits 0:3 - Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Radio data rate and modulation setting. The radio supports Frequency-shift Keying (FSK) modulation."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data rate and modulation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
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
