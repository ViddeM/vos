#![feature(lang_items, core_intrinsics, rustc_private)]
#![feature(start)]
#![no_std]
#![no_main]

// pub mod util;
// use crate::util::print::{clear_screen, print_str};
use core::intrinsics;
use core::panic::PanicInfo;
use core::ptr::write_volatile;

mod print;

use crate::print::{clear_screen, print_str};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main()
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // unsafe { write_volatile(0xb8000 as *mut u32, 0x16971697) }

    clear_screen();
    // print_str("Welcome to the 64-bit VOS kernel!");
    print_str("0004 008 012 016 020 024 028 032 036 040 044 048 052 056 060 064 068 072 076 080 084");
    loop {}
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_impl"]
#[no_mangle]
#[panic_handler]
pub extern "C" fn rust_begin_panic(info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}
