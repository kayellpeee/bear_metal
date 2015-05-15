// very minimal rust program
#![feature(lang_items, start, no_std, core)]
#![no_std]

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments,
                 _file_line: &(&'static str, usize)) -> ! {
    loop { }
}
