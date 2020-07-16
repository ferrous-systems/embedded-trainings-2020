# USB-1: Dealing with USB Events

The USBD peripheral on the nRF52840 contains a series of registers, called EVENTS registers, that indicate the reason for entering the USBD event handler. These events must be handled by the application to complete the enumeration process.

✅ Open the `firmware/src/bin/usb-1.rs` file.

In this starter code the USBD peripheral is initialized in `init` and a task, named `main`, is bound to the interrupt signal USBD. This task will be called every time a new USBD event needs to be handled. The `main` task uses `usbd::next_event()` to check all the event registers; if any event is set (occurred) then the function returns the event, represented by the `Event` enum, wrapped in the `Some` variant. This `Event` is then passed to the `on_event` function for further processing.

✅ Connect the USB cable to the port J3 then run the starter code.

❗️ Keep the cable connected to the J3 port for the rest of the workshop

Go to `fn on_event`, line 39. In this section you'll need to implement the following USB events until you reach the EP0SETUP event:

- `USBRESET`. This event indicates that the host issued a USB reset signal. According to the USB specification this will move the device from any state to the `Default` state. Since we are currently not dealing with any other state, you can handle this state by doing nothing.

- `EP0SETUP`. The USBD peripheral has detected the SETUP stage of a control transfer. If you get to this point move to the next section.

- `EP0DATADONE`. The USBD peripheral is signaling the end of the DATA stage of a control transfer. You won't encounter this event just yet.

When you are done you should see this output:

``` console
(..)
INFO:usb_1 -- USB: UsbEp0Setup
INFO:usb_1 -- goal reached; move to the next section
```

Do not overthink this exercise; it is not a trick question. There is very little to do and no new functionality to add.

You can find the solution in the `usb-1-solution.rs` file.
