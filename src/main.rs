#![feature(lang_items,start)]
#![no_std]
#![no_main]    

use core::fmt;

mod ports;
use ports::*;

// STUBs for linking. Panicing definitely doesn't actually work here -
// we just hang when we find a problem.  For hardware with a watchdog,
// this is actually reasonable - the watchdog will eventually do the
// right thing.
#[lang="eh_personality"]
extern fn eh_personality() {}
#[lang="panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_unwind(_msg: fmt::Arguments,
                                _file: &'static str,
                                _line: u32) -> ! {
    loop {}
}

extern {
    pub fn sysinit() -> ();
}

fn main() {
    unsafe {
        let port = Port::port(PortName::C);
        port.set_pcr_mux(5, 1);
        let gpio = Gpio::port(PortName::C);
        gpio.set_pddr(1<<5);
        loop {
            for _ in 0..100000 {
                gpio.set_psor(1<<5);
            }
            for _ in 0..100000 {
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

// STUB for linking. Exception handling is DEFINITELY broken here.
// We should be linking in libgcc, but that requires implementing some more
// memory menagement functions first.
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}
