# Handling GET_DESCRIPTOR Configuration Requests

When the host issues a GET_DESCRIPTOR *Configuration* request the device needs to respond with the requested configuration descriptor *plus* all the interface and endpoint descriptors associated to that configuration descriptor during the DATA stage.

 A GET_DESCRIPTOR Configuration request is a GET_DESCRIPTOR request where the descriptor type encoded in the high bit of `wValue` is CONFIGURATION.

Let's look into all the concepts required to respond to this request.
