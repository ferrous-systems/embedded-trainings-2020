# Radio Puzzle

Your task in this section is to decrypt the [substitution cipher] encrypted *ASCII* string stored in the Dongle. The string has been encrypted using *simple substitution*.


[substitution cipher]: https://en.wikipedia.org/wiki/Substitution_cipher

  ✅ Flash the `puzzle.hex` program on the Dongle. Follow the instructions from the "nRF52840 Dongle" section but flash the `puzzle.hex` program instead of the `loopback.hex` one -- don't forget to put the Dongle in bootloader mode before invoking `dongle-flash`.

> Note: If you experienced USB issues with `loopback.hex` you use the `puzzle-nousb*.hex` variants.

Like in the previous sections the Dongle will listen for radio packets -- this time over *channel 25* -- while also logging messages over a USB/serial interface.

✅ Open the `beginner/apps` folder in VS Code; then open the `src/bin/radio-puzzle.rs` file. Run the program. 


## Dongle Responses

The Dongle will respond differently depending on the length of the incoming packet:

- On zero-sized packets it will respond with the encrypted string.
- On one-byte sized packets it will respond with the *direct* mapping from a *plaintext* letter (single `u8` value) -- the letter contained in the packet -- to the *ciphertext* letter (`u8` value).
- On packets of any other length the Dongle will respond with the string `correct` if it received the decrypted string, otherwise it will respond with the `incorrect` string.

The Dongle will always respond with packets that are valid UTF-8 so you can use `str::from_utf8` on the response packets.
