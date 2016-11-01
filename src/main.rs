#![feature(lang_items,start,core_intrinsics)]
#![no_std]
#![no_main]    

use core::fmt;

mod ports;
use ports::*;
mod watchdog;
use watchdog::*;

extern {
    pub fn sysinit() -> ();
}

fn main() {
    unsafe {
        let port = Port::reg(PortName::C);
        port.set_pcr(5, 1<<8);
        let gpio = Gpio::reg(PortName::C);
        gpio.set_pddr(1<<5);
        loop {
            for _ in 0..10000000 {
                gpio.set_psor(1<<5);
            }
            for _ in 0..10000000 {
                gpio.set_pcor(1<<5);
            }
        }
    }
}

#[no_mangle]
pub extern fn start() {
    unsafe {
        sysinit();
    }
    main();
    panic!("Control left main()");
}

#[no_mangle]
pub extern fn wdog_disable() {
    unsafe {
        Wdog::reg().disable();
    }
}

// These functions and traits are used by the compiler, but are not
// clearly useful for embedded (would you really want an unwinder?)
// Right now a panic will cause the device to hang, which will end up
// eventually tripping up a hardware watchdog if you have it set.
//
// Possibly in the future we could implement a panic that uses the
// serial port to send panic information back to a host computer.

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(_msg: core::fmt::Arguments,
                               _file: &'static str,
                               _line: u32) -> ! {
    loop {};
}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}
