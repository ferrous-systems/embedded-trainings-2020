# Help

## Use a dictionary.

Our suggestion is to use a dictionary / map. `std::collections::HashMap` is not available in `no_std` code (without linking to a global allocator) but you can use one of the stack-allocated maps in the [`heapless`] crate. It supplies a stack-allocated, fixed-capacity version of the `std::Vec` type which will come in handy to store byte arrays. To store character mappings we recommend using a `heapless::IndexMap`.

`heapless` is already declared as a dependency in the Cargo.toml of the project so you can directly import it into the application code using a `use` statement.

[`heapless`]: https://docs.rs/heapless
[crates.io]: https://crates.io/crates/heapless


``` rust
use heapless::Vec;         // like `std::Vec` but stack-allocated
use heapless::FnvIndexMap; // a dictionary / map
use heapless::consts::*;   // defines U16, U32, U64... etc. to set the size of the IndexMap

fn main() {
    // A hash map with a capacity of 16 key-value pairs allocated on the stack
    // note that U16 is a heapless constant, not Rust's u16
    let mut my_map = FnvIndexMap::<_, _, U16>::new();
    my_map.insert(b'A', b'~').unwrap();

    // A vector with a fixed capacity of 8 elements allocated on the stack
    // note that U8 is a heapless constant, not Rust's u8
    let mut my_vec = Vec::<_, U8>::new();
    my_vec.push(b'A').unwrap();
}
```

If you haven't used a stack-allocated collection before note that you'll need to specify the capacity of the collection as a type parameter using one of the "type-level values" in the `heapless::consts` module (e.g. `U8`, `U64` etc.). The [`heapless::IndexMap` documentation][indexMap] of the `heapless` crate has some usage examples, as does the [`heapless::Vec` documentation][vec].

[indexMap]: https://docs.rs/heapless/0.5.5/heapless/struct.IndexMap.html
[vec]: https://docs.rs/heapless/0.5.5/heapless/struct.Vec.html

## Note the difference between character literals and byte literals!

Something you will likely run into while solving this exercise are *character* literals (`'c'`) and *byte* literals (`b'c'`). The former has type [`char`] and represent a single Unicode "scalar value". The latter has type `u8` (1-byte integer) and it's mainly a convenience for getting the value of ASCII characters, for instance `b'A'` is the same as the `65u8` literal.

[`char`]: https://doc.rust-lang.org/std/primitive.char.html

*IMPORTANT* you do not need to use the `str` or `char` API to solve this problem, other than for printing purposes. Work directly with slices of bytes (`[u8]`) and bytes (`u8`); and only convert those to `str` or `char` when you are about to print them.

P.S. The plaintext string is *not* stored in `puzzle.hex` so running `strings` on it will not give you the answer.

## Make sure not to flood the log buffer
When you log more messages than can be moved from the probe to the target, the log buffer will get overwritten, resulting in data loss. This can easily happen when you repeatedly poll the dongle and log the result. The quickest solution to this is to wait a short while until you send the next packet so that the logs can be processed in the meantime.

``` rust
use core::time::Duration;

#[entry]
fn main() -> ! {

    let mut timer = board.timer;

    for plainletter in 0..=127 {
        /* ... send letter to dongle ... */
        defmt::println!("got response");
        /* ... store output ... */

        timer.wait(Duration::from_millis(20));
    }
}
```


## Recommended Steps:

 Each step is demonstrated in a separate example so if for example you only need a quick reference of how to use the map API you can step / example number 2.

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

