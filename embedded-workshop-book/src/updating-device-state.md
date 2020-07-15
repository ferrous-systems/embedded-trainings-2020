# Updating Device State

At some point during the initialization you'll receive a `SET_ADDRESS` request that will move the device from the `Default` state to the `Address` state. If you are working on Linux, you'll also receive a `SET_CONFIGURATION` request that will move the device from the `Address` state to the `Configured` state. Additionally, some requests are only valid in certain states– for example `SET_CONFIGURATION` is only valid if the device is in the `Address` state. For this reason `usb-4.rs` will need to keep track of the device's current state.

The device state should be tracked using a resource so that it's preserved across multiple executions of the `USBD` event handler. The `usb2` crate has a `State` enum with the 3 possible USB states: `Default`, `Address` and `Configured`. You can use that enum or roll your own.

Start tracking and updating the device state to move your request handling forward:

1. **Update the handling of the `USBRESET` event:** Instead of ignoring it, we now want it to change the state of the USB device. See section 9.1 USB Device States of the USB specification for details on what to do.

2. **Update the handling of `SET_ADDRESS` requests:** See the section on [Handling SET_ADDRESS Requests](./set-address.md#handling-set_address-requests) of this tutorial for details.

3. **Implement the handling of `GET_DESCRIPTOR Configuration` requests:** See the section on [Handling GET_DESCRIPTOR Configuration Requests](./get-descriptor-config.md#handling-get_descriptor-configuration-requests) of this tutorial for details.
