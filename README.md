# `lm3s6965evb`

> An example of running a [`cortex-m-rt`] program on a QEMUlated Cortex-M core

[`cortex-m-rt`]: https://crates.io/crates/cortex-m-rt

`qemu-system-arm` supports emulation of the LM3S6965EVB (Cortex-M3) and LM3S811EVB (Cortex-M3)
evaluation boards. This Cargo project demonstrates how to debug a Rust program running on
`qemu-system-arm`.

## Try it yourself

- Build the example program

``` console
$ # from within this repository
$ cargo build
```

- Start the emulator. This command will block the terminal.

``` console
$ qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -gdb tcp::3333 \
      -S \
      -nographic \
      -kernel target/thumbv7m-none-eabi/debug/lm3s6965evb
```

Meaning of the flags:

  - `-cpu`, CPU model to use. This matches the microcontroller on the evaluation board to emulate.
  - `-machine`, device / board / system to emulate
  - `-gdb`, wait for GDB connection on $PORT
  - `-S`, freeze CPU at startup
  - `-nographic`, disable graphical output and redirect serial I/Os to console

- On another terminal: start a GDB session and connect the debugger to the emulated device.

``` console
$ # note that there is a .gdbinit file in this repository
$ # the .gdbinit file makes GDB advance to program to the `main` function in `src/main.rs`
$ arm-none-eabi-gdb target/thumbv7m-none-eabi/debug/lm3s6965evb
```

- (Optional) If you have execution of .gdbinit disabled you'll have to run these commands in GDB.

``` console
(gdb) target remote :3333

(gdb) break lm3s6965evb

(gdb) continue
```

- Inspect the program within GDB

``` console
Breakpoint 3, lm3s6965evb::main () at src/main.rs:19
19          let x = 42;
(gdb) step
21          loop {

(gdb) print x
$1 = 42

(gdb) print &x
$2 = (i32 *) 0x2000ffec
```

## How to create a `cortex-m-rt` program from scratch

Follow the instructions in [`cortex-m-quickstart`]. For convenience the required commands are shown
below:

[`cortex-m-quickstart`]: https://docs.rs/cortex-m-quickstart/0.3.1/cortex_m_quickstart/

- Clone the Cargo project template

``` console
$ cargo clone cortex-m-quickstart --vers 0.3.2

$ cd cortex-m-quickstart
```

- Specify the memory layout

``` console
$ cat >memory.x <<EOF
/* Memory layout of the LM3S6965 microcontroller */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}
EOF
```

- Set a default target

``` console
$ cat >> .cargo/config <<'EOF'

[build]
target = "thumbv7m-none-eabi" # Cortex-M3
EOF
```

- Build an example and proceed with the debugging section from before.

``` console
$ cargo build --example minimal
```

## Observations

- Unlike QEMU user emulation (`qemu-arm`), Cortex-M registers are properly emulated in
  `qemu-system-arm` so operations like `core::arch::arm::__get_BASEPRI` (appear to) work, i.e. they
  don't crash QEMU.

- The BKPT instruction triggers a HardFault exception.

On hardware, this instruction halts the processor and notifies the debugger when the processor is
  connected a physical debugger (e.g. ST-LINK); when the processor is not connected to a physical
  debugger this instruction causes a HardFault exception.

- Executing the UDF instruction (`intrinsics::abort`) in the HardFault handler causes QEMU to
  terminate with a fatal error ("Lockup: can't escalate 3 to HardFault (current priority -1)").

On hardware, the UDF instruction triggers the HardFault exception; executing the UDF instruction
from the HardFault handler causes the processor to jump to the start of the HardFault handler.

- The WFI and WFE instructions appears to be ignored (i.e. interpreted as NOP).

On hardware, these instructions make the processor sleep until an interrupt signal arrives (WFI) or
until an event occurs (WFE).

## Just want to run on QEMU?

Run without `-gdb` and `-S` (stop) option.
Add `-semihosting-config enable=on,target=native` to enable semihosting console output and exit.

```
$ qemu-system-arm \
       -cpu cortex-m3 \
       -machine lm3s6965evb \
       -semihosting-config enable=on,target=native \
       -nographic \
       -kernel target/thumbv7m-none-eabi/debug/lm3s6965evb
x = 42
$
```

The command exits immediately.

## References

- [Data sheet for the Stellaris LM3S6965
  microcontroller](http://www.ti.com/lit/ds/symlink/lm3s6965.pdf).

- [Data sheet for the Stellaris LM3S811
  microcontroller](http://www.ti.com/lit/ds/symlink/lm3s811.pdf).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
