# USB Endpoints

Under the USB protocol data transfers occur over *endpoints*.

Endpoints are similar to UDP or TCP ports in that they allow logical multiplexing of data over a single physical USB bus. USB endpoints, however, have directions: an endpoint can either be an IN endpoint or an OUT endpoint. The direction is always from the perspective of the host so at an IN endpoint data travels from the device to the host and at an OUT endpoint data travels from the host to the device.

Endpoints are identified by their address, a zero-based index, and direction. There are four types of endpoints: control endpoints, bulk endpoints, interrupt endpoints and isochronous endpoints. Each endpoint type has different properties: reliability, latency, etc. In this workshop we'll only need to deal with control endpoints.

All USB devices must use "endpoint 0" as the default control endpoint. "Endpoint 0" actually refers to two endpoints: endpoint 0 IN and endpoint 0 OUT. This endpoint pair is used to establish a *control pipe*, a bidirectional communication channel between the host and device where data is exchanged using a predefined format. The default control pipe over endpoint 0 is mandatory: it must always be present and must always be active.

For detailed information about endpoints check section 5.3.1, Device Endpoints, of the [USB 2.0 specification][usb20].

[usb20]: https://www.usb.org/document-library/usb-20-specification