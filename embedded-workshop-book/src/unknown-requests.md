# Dealing with unknown requests: Stalling the endpoint

You may come across host requests other than the ones listed in previous sections.

For this situation, the USB specification defines a device-side procedure for "stalling an endpoint", which amounts to the device telling the host that it doesn't support some request.
> This procedure should be used to deal with invalid requests, requests whose SETUP stage doesn't match any USB 2.0 standard request, and requests not supported by the device– for instance the SET_DESCRIPTOR request is not mandatory.

You can use the `dk::usbd::ep0stall()` helper function to stall endpoint 0.
Your task is to do this in the right place in `usb-4.rs`.