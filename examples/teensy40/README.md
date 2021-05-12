### LCD1602-RS on Teensy 4.0

This example uses [teensy4-rs](https://github.com/mciantyre/teensy4-rs) for its embedded-hal implementation. Make sure you have the dependencies listed in that project before running this crate.

To build, simply:

```cargo build```

And then convert the binary to hex and upload it to your Teensy (this can be done via the `./build-upload.sh` script).

Make sure you have the following connections from your Teensy to the 1602 LCD:

```
P12 -> RS
P11 -> EN
P5  -> D4
P4  -> D5
P3  -> D6
P2  -> D7
```

The result should be the 1602 printing "Hello world" every second.
