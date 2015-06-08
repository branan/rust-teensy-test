# cargo/teensy minimal project

This is a simple project to blink the LED on a teensy using
Rust. Right now it the Rust code just calls back out to C for the
actual blinking implementation. Porting all that code to Rust is the
next step.

The runtime isn't full-featured yet. Noticably missing is the mem*
functions, and malloc/free. Implementing those will allow us to link
libgcc to get proper exception handling support.

## Building

Just run `make`! Sometimes errors can be opaque, I'm still getting
stderr propagation through the whole build stack sane.

To flash the program to a Teensy, run `make flash`.
