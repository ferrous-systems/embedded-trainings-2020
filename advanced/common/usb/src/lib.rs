//! Some USB 2.0 data types

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

use core::num::NonZeroU8;

/// Standard USB request
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Request {
    /// GET_DESCRIPTOR
    GetDescriptor {
        /// Requested descriptor
        descriptor: Descriptor,
        /// How many bytes of data to return
        length: u16,
    },

    /// SET_CONFIGURATION
    SetConfiguration {
        /// bConfigurationValue to change the device to
        value: Option<NonZeroU8>,
    },

    /// SET_ADDRESS
    SetAddress {
        /// New device address, in the range `1..=127`
        address: Option<NonZeroU8>,
    },
}

impl Request {
    /// Parses SETUP packet data into a USB request
    pub fn parse(
        _bmrequesttype: u8,
        _brequest: u8,
        _wvalue: u16,
        _windex: u16,
        _wlength: u16,
    ) -> Result<Self, ()> {
        // TODO
        Err(())
    }
}

/// Descriptor types that appear in GET_DESCRIPTOR requests
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Descriptor {
    /// Device descriptor
    Device,

    /// Configuration descriptor
    Configuration {
        /// Index of the descriptor
        index: u8,
    },
}

#[cfg(test)]
mod tests {
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

        // has language ID but shouldn't
        assert!(Request::parse(0b1000_0000, 0x06, 0x01_00, 1033, 18).is_err());
    }

    #[ignore]
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
    }

    #[ignore]
    #[test]
    fn set_address() {
        // OK: SET_ADDRESS 16
        assert_eq!(
            Request::parse(0b0000_0000, 0x05, 0x00_10, 0, 0),
            Ok(Request::SetAddress {
                address: NonZeroU8::new(0x10)
            })
        );

        // has language id but shouldn't
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_10, 1033, 0).is_err());

        // length should be zero
        assert!(Request::parse(0b0000_0000, 0x05, 0x00_10, 0, 1).is_err());
    }

    #[ignore]
    #[test]
    fn set_configuration() {
        // OK: SET_CONFIGURATION 1
        assert_eq!(
            Request::parse(0b0000_0000, 0x09, 0x00_01, 0, 0),
            Ok(Request::SetConfiguration {
                value: NonZeroU8::new(1)
            })
        );

        // has language id but shouldn't
        assert!(Request::parse(0b0000_0000, 0x09, 0x00_01, 1033, 0).is_err());

        // length should be zero
        assert!(Request::parse(0b0000_0000, 0x09, 0x00_01, 0, 1).is_err());
    }
}
