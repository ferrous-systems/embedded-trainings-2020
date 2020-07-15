# Control Transfers

Before we continue we need to discuss how data transfers work under the USB protocol.

The control pipe handles *control transfers*, a special kind of data transfer used by the host to issue *requests*. A control transfer is a data transfer that occurs in three stages: a SETUP stage, an optional DATA stage and a STATUS stage.

During the SETUP stage the host sends 8 bytes of data that identify the control request. Depending on the issued request there may be a DATA stage or not; during the DATA stage data is transferred either from the device to the host or the other way around. During the STATUS stage the device acknowledges, or not, the whole control request.

For detailed information about control transfers check section 5.5, Control Transfers, of the [USB 2.0 specification][usb20].