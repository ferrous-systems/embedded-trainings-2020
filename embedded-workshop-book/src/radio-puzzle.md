# Radio Puzzle

For this section you'll need to flash the `puzzle.hex` program on the Dongle. Follow the instructions from the "nRF52840 Dongle" section but flash the `puzzle.hex` program instead of the `loopback.hex` one -- don't forget to put the Dongle in bootloader mode before invoking `dongle-flash`.

Like in the previous sections the Dongle will listen for radio packets -- this time over *channel 25* -- while also logging messages over a USB/serial interface.

Open the `beginner/apps` folder in VS Code; then open the `src/bin/radio-puzzle.rs` file.

Your task in this section is to decrypt the [substitution cipher] encrypted *ASCII* string stored in the Dongle. The string has been encrypted using *simple substitution*.

[substitution cipher]: https://en.wikipedia.org/wiki/Substitution_cipher

The Dongle will respond differently depending on the length of the incoming packet:

- On zero-sized packets it will respond with the encrypted string.
- On one-byte sized packets it will respond with the *direct* mapping from a *plaintext* letter (single `u8` value) -- the letter contained in the packet -- to the *ciphertext* letter (`u8` value).
- On packets of any other length the Dongle will respond with the string `correct` if it received the decrypted string, otherwise it will respond with the `incorrect` string.

The Dongle will always respond with packets that are valid UTF-8 so you can use `str::from_utf8` on the response packets.

Our suggestion is to use a dictionary / map. `std::collections::HashMap` is not available in `no_std` code (without linking to a global allocator) but you can use one of the stack-allocated maps in the [`heapless`] crate. A `Vec`-like buffer may also come in handy; `heapless` provides a stack-allocated, fixed-capacity version of the `std::Vec` type.

`heapless` is already declared as a dependency in the Cargo.toml of the project so you can directly import it into the application code using a `use` statement.

[`heapless`]: https://docs.rs/heapless
[crates.io]: https://crates.io/crates/heapless



``` rust
use heapless::IndexMap; // a dictionary / map
use heapless::Vec; // like `std::Vec` but stack-allocated
```

If you haven't used a stack-allocated collection before note that you'll need to specify the capacity of the collection as a type parameter using one of the "type-level values" in the `heapless::consts` module. The [IndexMap documentation][indexMap] of the `heapless` crate has some usage examples.
[indexMap]: https://docs.rs/heapless/0.5.5/heapless/struct.IndexMap.html

Something you will likely run into while solving this exercise are *character* literals (`'c'`) and *byte* literals (`b'c'`). The former has type [`char`] and represent a single Unicode "scalar value". The latter has type `u8` (1-byte integer) and it's mainly a convenience for getting the value of ASCII characters, for instance `b'A'` is the same as the `65u8` literal.

[`char`]: https://doc.rust-lang.org/std/primitive.char.html

*IMPORTANT* you do not need to use the `str` or `char` API to solve this problem, other than for printing purposes. Work directly with slices of bytes (`[u8]`) and bytes (`u8`); and only convert those to `str` or `char` when you are about to print them.

P.S. The plaintext string is *not* stored in `puzzle.hex` so running `strings` on it will not give you the answer.

These are our recommended steps to tackle the problem. Each step is demonstrated in a separate example so if for example you only need a quick reference of how to use the map API you can step / example number 2.

1. Send a one letter packet (e.g. `A`) to the radio to get a feel for how the mapping works. Then do a few more letters. Check out example `radio-puzzle-1`

2. Get familiar with the dictionary API. Do some insertions and look ups. What happens if the dictionary gets full? See `radio-puzzle-2`

3. Next, get mappings from the radio and insert them into the dictionary. See `radio-puzzle-3`

4. You'll probably want a buffer to place the plaintext in. We suggest using `heapless::Vec` for this. See `radio-puzzle-4` (NB It is also possible to decrypt the packet in place)

5. Simulate decryption: fetch the encrypted string and "process" each of its bytes. See `radio-puzzle-5`

6. Now merge steps 3 and 5: build a dictionary, retrieve the secret string and do the reverse mapping to decrypt the message. See `radio-puzzle-6`

7. As a final step, send the decrypted string to the Dongle and check if it was correct or not. See `radio-puzzle-7`

For your reference, we have provided a complete solution in the `src/bin/radio-puzzle-solution.rs` file. That solution is based on the seven steps outlined above. Did you solve the puzzle in a different way?

If you solved the puzzle using a `Vec` buffer you can try solving it without the buffer as a stretch goal. You may find the [slice methods][slice] that let you mutate its data useful. A solution that does not use the `Vec` buffer can be found in the `radio-puzzle-solution-2` file.

[slice]: https://doc.rust-lang.org/std/primitive.slice.html#methods

