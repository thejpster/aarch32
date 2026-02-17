# Examples for Arm Versatile Application Board

This package contains example binaries for the Arm Versatile Application
baseboard evaluation system, featuring an Arm Cortex-R5 processor or Arm
Cortex-A8 processor core. This crate is tested on the following targets:

- `armv4t-none-eabi` - ARMv4T, soft-float, Arm mode
- `armv5te-none-eabi` - ARMv5TE, soft-float, Arm mode
- `armv6-none-eabi` - ARMv6K, soft-float, Arm mode
- `armv6-none-eabihf` - ARMv6K, hard-float, Arm mode
- `armv7r-none-eabi` - ARMv7-R, soft-float, Arm mode
- `armv7r-none-eabihf` - ARMv7-R, hard-float, Arm mode
- `armv7a-none-eabi` - ARMv7-A, soft-float, Arm mode
- `armv7a-none-eabihf` - ARMv7-R, hard-float, Arm mode
- `thumbv6-none-eabi` - ARMv6K, soft-float, Thumb mode
- `thumbv7r-none-eabi` - ARMv7-R, soft-float, Thumb mode
- `thumbv7r-none-eabihf` - ARMv7-R, hard-float, Thumb mode
- `thumbv7a-none-eabi` - ARMv7-A, soft-float, Thumb mode
- `thumbv7a-none-eabihf` - ARMv7-A, hard-float, Thumb mode

The repo-level [`.cargo/config.toml`] will ensure the code runs on the
appropriate QEMU configuration. Note that there is no FPU support for ARMv4T or
ARMv5TE or for ARMv6K when in Thumb mode.

Several of our supported platforms have Tier 3 targets, which means Nightly Rust
is required. This folder contains a [`rust-toolchain.toml`] which pins us to a
specific release of nightly that is known to work.

We have only tested this crate on `qemu-system-arm` emulating the Arm
Versatile Application Board, not the real thing.

[`.cargo/config.toml`]: ../../.cargo/config.toml
[`rust-toolchain.toml`]: ./rust-toolchain.toml

## Running

Run these examples as follows:

```console
$ cargo run --bin hello
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.19s
     Running `qemu-system-arm -machine versatileab -cpu cortex-r5f -semihosting -nographic -audio none -kernel target/armv7r-none-eabihf/debug/hello`
Hello, this is semihosting! x = 1.000, y = 2.000
PANIC: PanicInfo {
    message: I am an example panic,
    location: Location {
        file: "src/bin/hello.rs",
        line: 19,
        column: 5,
    },
    can_unwind: true,
    force_no_backtrace: false,
}
$ cargo run --bin hello --target armv4t-none-eabi -Zbuild-std=core
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.72s
     Running `qemu-system-arm -machine versatileab -cpu arm926 -semihosting -nographic -audio none -kernel target/armv4t-none-eabi/debug/hello`
Hello, this is semihosting! x = 1.000, y = 2.000
PANIC: PanicInfo {
    message: I am an example panic,
    location: Location {
        file: "src/bin/hello.rs",
        line: 19,
        column: 5,
    },
    can_unwind: true,
    force_no_backtrace: false,
}
```

## Debugging

You can start a GDB server by adding `-- -s -S` to the end of the `cargo run`
command, and the connect with GDB as follows:

```console
$ cargo run --bin hello -- -s -S
# QEMU runs and hangs waiting for a connection. In another terminal run:
$ arm-none-eabi-gdb -x commands.gdb target/armv7r-none-eabihf/debug/hello
# GDB will start and connect to QEMU's GDB server. The commands.gdb file sets up some useful defaults.
```

## Minimum Supported Rust Version (MSRV)

These examples are guaranteed to compile on the version of Rust given in the
[`rust-toolchain.toml`] file. These examples are not version controlled and we
may change the MSRV at any time.

## Licence

- Copyright (c) Ferrous Systems
- Copyright (c) The Rust Embedded Devices Working Group developers

Licensed under either [MIT](../LICENSE-MIT) or [Apache-2.0](../LICENSE-APACHE) at
your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be licensed as above, without any
additional terms or conditions.
