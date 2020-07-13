# USB-4: Supporting more Standard Requests

After responding to the `GET_DESCRIPTOR Device` request the host will start sending different requests. The parser in `common/usb` will need to be updated to handle these requests:

1. `GET_DESCRIPTOR Configuration`, see section [Handling GET_DESCRIPTOR Configuration Requests](#handling-get_descriptor-configuration-requests)
2. `SET_CONFIGURATION`, see section [SET_CONFIGURATION](#set_configuration) of this course material

The starter `common/usb` code contains unit tests for these other requests as well as extra `Request` variants for these requests. All of them have been commented out using a `#[cfg(TODO)]` attribute which you can remove once you need any new variant or new unit test.

For each green test, you can extend `usb-4.rs` to handle the new requests your parser is now able to recognize. **Make sure to read the next sections as you're working**, since they contain explanations about the concepts used and needed to complete this task.

If you need a reference, you can find solutions to parsing `GET_DESCRIPTOR Configuration` and `SET_CONFIGURATION` requests in the following files:

- `advanced/common/src/get-descriptor-configuration.rs`
- `advanced/common/src/set-configuration.rs`

Each file contains just enough code to parse the request in its name and the `GET_DESCRIPTOR Device` request. So you can refer to `set-configuration.rs` without getting "spoiled" about how to parse the `SET_CONFIGURATION` request.
