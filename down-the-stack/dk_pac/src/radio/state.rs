#[doc = "Register `STATE` reader"]
pub struct R(crate::R<STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATE` reader - Current radio state"]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
#[doc = "Current radio state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: RADIO is in the Disabled state"]
    DISABLED = 0,
    #[doc = "1: RADIO is in the RXRU state"]
    RX_RU = 1,
    #[doc = "2: RADIO is in the RXIDLE state"]
    RX_IDLE = 2,
    #[doc = "3: RADIO is in the RX state"]
    RX = 3,
    #[doc = "4: RADIO is in the RXDISABLED state"]
    RX_DISABLE = 4,
    #[doc = "9: RADIO is in the TXRU state"]
    TX_RU = 9,
    #[doc = "10: RADIO is in the TXIDLE state"]
    TX_IDLE = 10,
    #[doc = "11: RADIO is in the TX state"]
    TX = 11,
    #[doc = "12: RADIO is in the TXDISABLED state"]
    TX_DISABLE = 12,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::DISABLED),
            1 => Some(STATE_A::RX_RU),
            2 => Some(STATE_A::RX_IDLE),
            3 => Some(STATE_A::RX),
            4 => Some(STATE_A::RX_DISABLE),
            9 => Some(STATE_A::TX_RU),
            10 => Some(STATE_A::TX_IDLE),
            11 => Some(STATE_A::TX),
            12 => Some(STATE_A::TX_DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STATE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RX_RU`"]
    #[inline(always)]
    pub fn is_rx_ru(&self) -> bool {
        *self == STATE_A::RX_RU
    }
    #[doc = "Checks if the value of the field is `RX_IDLE`"]
    #[inline(always)]
    pub fn is_rx_idle(&self) -> bool {
        *self == STATE_A::RX_IDLE
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == STATE_A::RX
    }
    #[doc = "Checks if the value of the field is `RX_DISABLE`"]
    #[inline(always)]
    pub fn is_rx_disable(&self) -> bool {
        *self == STATE_A::RX_DISABLE
    }
    #[doc = "Checks if the value of the field is `TX_RU`"]
    #[inline(always)]
    pub fn is_tx_ru(&self) -> bool {
        *self == STATE_A::TX_RU
    }
    #[doc = "Checks if the value of the field is `TX_IDLE`"]
    #[inline(always)]
    pub fn is_tx_idle(&self) -> bool {
        *self == STATE_A::TX_IDLE
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == STATE_A::TX
    }
    #[doc = "Checks if the value of the field is `TX_DISABLE`"]
    #[inline(always)]
    pub fn is_tx_disable(&self) -> bool {
        *self == STATE_A::TX_DISABLE
    }
}
impl R {
    #[doc = "Bits 0:3 - Current radio state"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Current radio state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state](index.html) module"]
pub struct STATE_SPEC;
impl crate::RegisterSpec for STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state::R](R) reader structure"]
impl crate::Readable for STATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE to value 0"]
impl crate::Resettable for STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
