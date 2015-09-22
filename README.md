# cargo/teensy minimal project

This is a simple project to blink the LED on a teensy using
Rust. Right now the Rust code just calls back out to C for the actual
blinking implementation. Porting all that code to Rust is the next
step.

The runtime isn't full-featured yet. Noticably missing is the mem*
functions, and malloc/free. Implementing those will allow us to link
libgcc for proper unwinding. Unwinding is questionable in an embedded
environment, though, so handling errors by locking up and letting a
hardawre watchdog solve the problem is my current plan :)

An advantage of unwinding is the ability to potentially log to a
serial device after the panic.

## Note

This currently requires nightly rust. The build script for the libcore
crate I'm using doesn't know how to handle stable releases. I don't
know if there are other issues with unstable features used by the
stack.

## Building

Just run `make`! Sometimes errors can be opaque, I'm still getting
stderr propagation through the whole build stack sane.

To flash the program to a Teensy, run `make flash`.
