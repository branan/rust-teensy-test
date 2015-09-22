#![feature(no_std)]
#![feature(no_std,lang_items,start)]
#![no_std]
#![no_main]    

use core::fmt;

// STUBs for linking. Panicing definitely doesn't actually work here -
// we just hang when we find a problem.  For hardware with a watchdog,
// this is actually reasonable - the watchdog will eventually do the
// right thing.
#[lang="stack_exhausted"]
extern fn stack_exhausted() { loop {} }
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
    pub fn blinky() -> ();
}

#[no_mangle]
pub extern fn main() {
    unsafe {
        blinky();
    }
}

// STUB for linking. Exception handling is DEFINITELY broken here.
// We should be linking in libgcc, but that requires implementing some more
// memory menagement functions first.
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}
