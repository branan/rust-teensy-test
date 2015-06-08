#![feature(no_std)]
#![feature(no_std,core,lang_items,start)]
#![no_std]
#![no_main]    

#[macro_use]
extern crate core;
use core::fmt;

// STUBs for linking. Exception handling is probably broken here.
#[lang="stack_exhausted"] extern fn stack_exhausted() { loop {} }
#[lang="eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"] #[no_mangle] pub extern fn rust_begin_unwind(msg: fmt::Arguments,
                                                                 file: &'static str, line: u32) -> ! { loop {} }

extern {
    pub fn blinky() -> ();
}

#[no_mangle]
pub extern fn main() {
    panic!();
}

// STUB for linking. Exception handling is DEFINITELY broken here.
// We should be linking in libgcc, but that requires implementing some more
// memory menagement functions first.
#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0() {}