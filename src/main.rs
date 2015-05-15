// very minimal rust program
#![feature(start, core)]

#[macro_use]
extern crate core;

#[start]
fn start(_arg_count: isize, _arg_vector: *const *const u8) -> isize {
    println!("this is a test {:?} argument length {:?} arguments",
           _arg_count, _arg_vector);
    0
}

