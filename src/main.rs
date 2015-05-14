// very minimal rust program
#![crate_type="lib"]
#![feature(start)]
#![feature(lang_items)]
#![feature(no_std)]
#![no_std]

extern crate syscall;
use syscall::*;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    syscall::WRITE("test");
    0
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
