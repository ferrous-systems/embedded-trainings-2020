#[doc = r"Register block"]
#[repr(C)]
pub struct CONFIG {
    #[doc = "0x00 - I2S mode."]
    pub mode: MODE,
    #[doc = "0x04 - Reception (RX) enable."]
    pub rxen: RXEN,
    #[doc = "0x08 - Transmission (TX) enable."]
    pub txen: TXEN,
    #[doc = "0x0c - Master clock generator enable."]
    pub mcken: MCKEN,
    #[doc = "0x10 - Master clock generator frequency."]
    pub mckfreq: MCKFREQ,
    #[doc = "0x14 - MCK / LRCK ratio."]
    pub ratio: RATIO,
    #[doc = "0x18 - Sample width."]
    pub swidth: SWIDTH,
    #[doc = "0x1c - Alignment of sample within a frame."]
    pub align: ALIGN,
    #[doc = "0x20 - Frame format."]
    pub format: FORMAT,
    #[doc = "0x24 - Enable channels."]
    pub channels: CHANNELS,
}
#[doc = "MODE (rw) register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "I2S mode."]
pub mod mode;
#[doc = "RXEN (rw) register accessor: an alias for `Reg<RXEN_SPEC>`"]
pub type RXEN = crate::Reg<rxen::RXEN_SPEC>;
#[doc = "Reception (RX) enable."]
pub mod rxen;
#[doc = "TXEN (rw) register accessor: an alias for `Reg<TXEN_SPEC>`"]
pub type TXEN = crate::Reg<txen::TXEN_SPEC>;
#[doc = "Transmission (TX) enable."]
pub mod txen;
#[doc = "MCKEN (rw) register accessor: an alias for `Reg<MCKEN_SPEC>`"]
pub type MCKEN = crate::Reg<mcken::MCKEN_SPEC>;
#[doc = "Master clock generator enable."]
pub mod mcken;
#[doc = "MCKFREQ (rw) register accessor: an alias for `Reg<MCKFREQ_SPEC>`"]
pub type MCKFREQ = crate::Reg<mckfreq::MCKFREQ_SPEC>;
#[doc = "Master clock generator frequency."]
pub mod mckfreq;
#[doc = "RATIO (rw) register accessor: an alias for `Reg<RATIO_SPEC>`"]
pub type RATIO = crate::Reg<ratio::RATIO_SPEC>;
#[doc = "MCK / LRCK ratio."]
pub mod ratio;
#[doc = "SWIDTH (rw) register accessor: an alias for `Reg<SWIDTH_SPEC>`"]
pub type SWIDTH = crate::Reg<swidth::SWIDTH_SPEC>;
#[doc = "Sample width."]
pub mod swidth;
#[doc = "ALIGN (rw) register accessor: an alias for `Reg<ALIGN_SPEC>`"]
pub type ALIGN = crate::Reg<align::ALIGN_SPEC>;
#[doc = "Alignment of sample within a frame."]
pub mod align;
#[doc = "FORMAT (rw) register accessor: an alias for `Reg<FORMAT_SPEC>`"]
pub type FORMAT = crate::Reg<format::FORMAT_SPEC>;
#[doc = "Frame format."]
pub mod format;
#[doc = "CHANNELS (rw) register accessor: an alias for `Reg<CHANNELS_SPEC>`"]
pub type CHANNELS = crate::Reg<channels::CHANNELS_SPEC>;
#[doc = "Enable channels."]
pub mod channels;
