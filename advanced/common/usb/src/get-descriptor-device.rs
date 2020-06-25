//! Some USB 2.0 data types
// NOTE this is a solution to exercise `usb-2`

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

#[cfg(TODO)]
use core::num::NonZeroU8;

/// Standard USB request
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Request {
    /// GET_DESCRIPTOR
    // see section 9.4.3 of the USB specification
    GetDescriptor {
        /// Requested descriptor
        descriptor: Descriptor,
        /// How many bytes of data to return
        length: u16,
    },

    /// SET_ADDRESS
    // see section 9.4.6 of the USB specification
    #[cfg(TODO)]
    SetAddress {
        /// New device address, in the range `1..=127`
        address: Option<NonZeroU8>,
    },

    /// SET_CONFIGURATION
    // see section 9.4.7 of the USB specification
    #[cfg(TODO)]
    SetConfiguration {
        /// bConfigurationValue to change the device to
        value: Option<NonZeroU8>,
    },
    // there are even more standard requests but we don't need to support them
}

impl Request {
    /// Parses SETUP packet data into a USB request
    ///
    /// Returns `Err` if the SETUP data doesn't match a supported standard request
    // see section 9.4 of the USB specification; in particular tables 9-3, 9-4 and 9-5
    pub fn parse(
        bmrequesttype: u8,
        brequest: u8,
        wvalue: u16,
        windex: u16,
        wlength: u16,
    ) -> Result<Self, ()> {
        // see table 9-4
        const GET_DESCRIPTOR: u8 = 6;

        if bmrequesttype == 0b10000000 && brequest == GET_DESCRIPTOR {
            // see table 9-5
            const DEVICE: u8 = 1;

            let desc_ty = (wvalue >> 8) as u8;
            let desc_index = wvalue as u8;
            let langid = windex;

            if desc_ty == DEVICE && desc_index == 0 && langid == 0 {
                Ok(Request::GetDescriptor {
                    descriptor: Descriptor::Device,
                    length: wlength,
                })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

/// Descriptor types that appear in GET_DESCRIPTOR requests
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Descriptor {
    /// Device descriptor
    Device,

    /// Configuration descriptor
    #[cfg(TODO)]
    Configuration {
        /// Index of the descriptor
        index: u8,
    },
    // there are even more descriptor types but we don't need to support them
}

#[cfg(test)]
mod tests {
    #[cfg(TODO)]
    use core::num::NonZeroU8;

    use crate::{Descriptor, Request};

    #[test]
    fn get_descriptor_device() {
        // OK: GET_DESCRIPTOR Device [length=18]
        assert_eq!(
            Request::parse(0b1000_0000, 0x06, 0x0100, 0, 18),
            Ok(Request::GetDescriptor {
                descriptor: Descriptor::Device,
                length: 18
            })
        );

        // wrong descriptor index
        assert!(Request::parse(0b1000_0000, 0x06, 0x01_01, 0, 18).is_err());
        //                                             ^^

        // has language ID but shouldn't
        assert!(Request::parse(0b1000_0000, 0x06, 0x01_00, 1033, 18).is_err());
        //                                                 ^^^^
    }

    #[cfg(TODO)]
    #[test]
    fn get_descriptor_configuration() {
        // OK: GET_DESCRIPTOR Configuration 0 [length=9]
        assert_eq!(
            Request::parse(0b1000_0000, 0x06, 0x02_00, 0, 9),
            Ok(Request::GetDescriptor {
                descriptor: Descriptor::Configuration { index: 0 },
                length: 9
            })
        );

        // has language ID but shouldn't
        assert!(Request::parse(0b1000_0000, 0x06, 0x02_00, 1033, 9).is_err());
        //                                                 ^^^^
    }

    #[cfg(TODO)]
    #[test]
    fn set_address() {
        // OK: SET_ADDRESS 16
        assert_eq!(
            Request::parse(0b0000_0000, 0x05, 0x00_10, 0, 0),
            Ok(Request::SetAddress {
                address: NonZeroU8::new(0x10)
            })
        );

        // OK: SET_ADDRESS 0
        assert_eq!(
            Request::parse(0b0000_0000, 0x05, 0x00_00, 0, 0),
            Ok(Request::SetAddress { address: None })
        );

        // address is outside the valid range
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_ff, 1033, 0).is_err());
        //                                             ^^

        // has language id but shouldn't
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_10, 1033, 0).is_err());
        //                                                 ^^^^

        // length should be zero
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_10, 0, 1).is_err());
        //                                                    ^
    }

    #[cfg(TODO)]
    #[test]
    fn set_configuration() {
        // OK: SET_CONFIGURATION 1
        assert_eq!(
            Request::parse(0b0000_0000, 0x09, 0x00_01, 0, 0),
            Ok(Request::SetConfiguration {
                value: NonZeroU8::new(1)
            })
        );

        // OK: SET_CONFIGURATION 0
        assert_eq!(
            Request::parse(0b0000_0000, 0x09, 0x00_00, 0, 0),
            Ok(Request::SetConfiguration { value: None })
        );

        // has language id but shouldn't
        assert!(Request::parse(0b0000_0000, 0x09, 0x00_01, 1033, 0).is_err());
        //                                                 ^^^^

        // length should be zero
        assert!(Request::parse(0b0000_0000, 0x09, 0x00_01, 0, 1).is_err());
        //                                                    ^
    }
}
