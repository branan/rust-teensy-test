# cargo/teensy minimal project

This is a simple project to blink the LED on a teensy using Rust. It
still uses C for early initialization and clock setup, but configures
the output port and blinks the LED directly in Rust.


## Toolchain setup

This currently requires nightly rust, for a number of reasons. The
best way to get nightly rust is to use `rustup` to install it:

    rustup toolchain install nightly

You will then want to configure rustup to use nightly for this project:

    cd /path/to/rust-teensy-test; rustup override set nightly

You'll also need the `rust-src` component so that certain core Rust libraries can be rebuilt, and the `xargo` binary that automatically builds them for us:

    rustup component add rust-src
    cargo install xargo

## Building

Just run `make`! Sometimes errors from the underlying c bits can be
opaque, I'm still getting stderr propagation through the whole build
stack

To flash the program to a Teensy, run `make flash`.
