# DMA

[What do endpoints have to do with dma]

[What is the task in this section?]

Let's zoom into the `Ep0In` abstraction used in `usb-4.rs` next. 

✅ Open the file. Use VSCode's "Go to Definition" to see the implementation of the `Ep0In.start()` method. 

What this method does is start a DMA transfer to send `bytes` to the host. The data (`bytes`) is first copied into an internal buffer and then the DMA is configured to move the data from that internal buffer to the USBD peripheral.

The signature of the `start()` method does *not* ensure that:

- `bytes` won't be deallocated before the DMA transfer is over (e.g. `bytes` could be pointing into the stack), or that
- `bytes` won't be modified right after the DMA transfer starts (this would be a data race in the general case).

For these two safety reasons the API is implemented using an internal buffer called `buffer`. The internal buffer has a `'static` lifetime so it's guaranteed to never be deallocated -- this prevents issue (a). The `busy` flag prevents any further modification to the internal buffer -- from the public API -- while the DMA transfer is in progress.

Apart from thinking about lifetimes and explicit data races in the surface API one must internally use memory fences to prevent reordering of memory operations (e.g. by the compiler), which can also cause data races. DMA transfers run in parallel to the instructions performed by the processor and are "invisible" to the compiler.

In the implementation of the `start` method, data is copied from `bytes` to the internal buffer (a `memcpy()` operation) and then the DMA transfer is started with a write to the `TASKS_STARTEPIN0` register. The compiler sees the start of the DMA transfer (register write) as an unrelated memory operation so it may move the `memcpy()` to *after* the DMA transfer has started. This reordering results in a data race: the processor modifies the internal buffer while the DMA is reading data out from it.

To avoid this reordering a memory fence, `dma_start()`, is used. The fence pairs with the *store* operation (register write) that starts the DMA transfer and prevents the previous `memcpy()`, and any other memory operation, from being move to *after* the store operation.

Another memory fence, `dma_end()`, is needed at the end of the DMA transfer. In the general case, this prevents instruction reordering that would result in the processor accessing the internal buffer *before* the DMA transfer has finished. This is particularly problematic with DMA transfers that modify a region of memory which the processor intends to read after the transfer.

> Note: Not relevant to the DMA operation but relevant to the USB specification, the `start()` method sets a shortcut in the USBD peripheral to issue a STATUS stage right after the DATA stage is finished. Thanks to this it is not necessary to manually start a STATUS stage after calling the `end` method.
