// very minimal rust program
#![feature(start)]

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    println!("this is a test {:?} {:?}", _argv, _argc);
    0
}

