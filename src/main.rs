// very minimal rust program
#![crate_type="lib"]
#![feature(start, no_std, core)]
#![no_std]

#[macro_use]
extern crate core;

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    assert_eq!(_argc, 1);
    0
}

