// very minimal rust program
#![crate_type="lib"]
#![feature(lang_items, start, no_std)]
#![no_std]

// http://stackoverflow.com/questions/28649120/
// unable-to-compile-rust-with-no-std-lang-items
#[lang="phantom_fn"]
trait PhantomFn<A: ?Sized, R: ?Sized = ()> {}

#[lang="sized"]
trait Sized: PhantomFn<Self> {}

#[lang="copy"]
trait Copy: PhantomFn<Self> {}

#[lang="sync"]
trait Sync: PhantomFn<Self> {}

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}


